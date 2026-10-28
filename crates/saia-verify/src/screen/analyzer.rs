use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const BLACK: RgbColor = RgbColor { r: 0, g: 0, b: 0 };
    pub const WHITE: RgbColor = RgbColor { r: 255, g: 255, b: 255 };

    /// Computes WCAG 2.1 relative luminance:
    /// L = 0.2126 * R + 0.7152 * G + 0.0722 * B (with sRGB gamma expansion)
    pub fn relative_luminance(&self) -> f32 {
        let f = |val: u8| -> f32 {
            let s = val as f32 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };

        0.2126 * f(self.r) + 0.7152 * f(self.g) + 0.0722 * f(self.b)
    }

    /// Computes WCAG 2.1 contrast ratio between two colors (ranging from 1.0 to 21.0).
    pub fn contrast_ratio(&self, other: &RgbColor) -> f32 {
        let l1 = self.relative_luminance();
        let l2 = other.relative_luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualRegion {
    pub min_x: u32,
    pub max_x: u32,
    pub min_y: u32,
    pub max_y: u32,
    pub width: u32,
    pub height: u32,
    pub foreground_pixels: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenAnalysisReport {
    pub width: u32,
    pub height: u32,
    pub background_color: RgbColor,
    pub foreground_color: RgbColor,
    pub contrast_ratio: f32,
    pub is_visually_legible: bool,
    pub legibility_rating: String,
    pub content_region: Option<VisualRegion>,
}

pub struct VisualScreenAnalyzer;

impl VisualScreenAnalyzer {
    /// Analyzes a standard BMP screenshot file and extracts luminance, contrast, and visual bounding boxes.
    pub fn analyze_bmp(path: &Path) -> anyhow::Result<ScreenAnalysisReport> {
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        if data.len() < 54 || &data[0..2] != b"BM" {
            anyhow::bail!("Invalid BMP image: header magic missing");
        }

        // Parse BMP header
        let offset = u32::from_le_bytes([data[10], data[11], data[12], data[13]]) as usize;
        let width = i32::from_le_bytes([data[18], data[19], data[20], data[21]]).unsigned_abs();
        let height = i32::from_le_bytes([data[22], data[23], data[24], data[25]]).unsigned_abs();
        let bpp = u16::from_le_bytes([data[28], data[29]]) as usize;

        if bpp != 24 && bpp != 32 {
            anyhow::bail!("Unsupported BMP bit depth: {} (expected 24 or 32 bpp)", bpp);
        }

        let bytes_per_pixel = bpp / 8;
        let row_stride = (width as usize * bytes_per_pixel).div_ceil(4) * 4;

        // Sample colors to find background and dominant foreground
        let mut color_histogram: HashMap<RgbColor, u32> = HashMap::new();
        let step = (width / 150).max(1) as usize;

        for y in (0..height as usize).step_by(step) {
            let row_offset = offset + (height as usize - 1 - y) * row_stride;
            if row_offset + (width as usize * bytes_per_pixel) > data.len() {
                continue;
            }

            for x in (0..width as usize).step_by(step) {
                let px_offset = row_offset + x * bytes_per_pixel;
                let b = data[px_offset];
                let g = data[px_offset + 1];
                let r = data[px_offset + 2];

                // Quantize slightly to group minor compression/anti-aliasing variations
                let qr = (r / 8) * 8;
                let qg = (g / 8) * 8;
                let qb = (b / 8) * 8;
                *color_histogram.entry(RgbColor::new(qr, qg, qb)).or_insert(0) += 1;
            }
        }

        let mut sorted_colors: Vec<(RgbColor, u32)> = color_histogram.into_iter().collect();
        sorted_colors.sort_by_key(|a| std::cmp::Reverse(a.1));

        let background = sorted_colors.first().map(|c| c.0).unwrap_or(RgbColor::WHITE);

        // Find the most prominent foreground color that differs from the background
        let foreground = sorted_colors
            .iter()
            .find(|(c, _)| {
                let diff = (c.r as i32 - background.r as i32).abs()
                    + (c.g as i32 - background.g as i32).abs()
                    + (c.b as i32 - background.b as i32).abs();
                diff > 40
            })
            .map(|c| c.0)
            .unwrap_or(RgbColor::BLACK);

        let contrast = background.contrast_ratio(&foreground);
        let is_legible = contrast >= 3.0;

        let legibility_rating = if contrast >= 7.0 {
            "AAA (High Contrast / Optimal Visibility)".to_string()
        } else if contrast >= 4.5 {
            "AA (Standard Contrast / Legible)".to_string()
        } else if contrast >= 3.0 {
            "Large-Text Only (Marginal Contrast)".to_string()
        } else {
            "FAIL (Invisible / Severe Low-Contrast)".to_string()
        };

        // Compute bounding box of non-background content
        let mut min_x = width;
        let mut max_x = 0;
        let mut min_y = height;
        let mut max_y = 0;
        let mut fg_pixels: u64 = 0;

        for y in (0..height).step_by(2) {
            let row_offset = offset + (height as usize - 1 - y as usize) * row_stride;
            if row_offset + (width as usize * bytes_per_pixel) > data.len() {
                continue;
            }

            for x in (0..width).step_by(2) {
                let px_offset = row_offset + x as usize * bytes_per_pixel;
                let b = data[px_offset];
                let g = data[px_offset + 1];
                let r = data[px_offset + 2];

                let diff = (r as i32 - background.r as i32).abs()
                    + (g as i32 - background.g as i32).abs()
                    + (b as i32 - background.b as i32).abs();

                if diff > 50 {
                    fg_pixels += 1;
                    if x < min_x { min_x = x; }
                    if x > max_x { max_x = x; }
                    if y < min_y { min_y = y; }
                    if y > max_y { max_y = y; }
                }
            }
        }

        let content_region = if fg_pixels > 20 && min_x <= max_x && min_y <= max_y {
            Some(VisualRegion {
                min_x,
                max_x,
                min_y,
                max_y,
                width: max_x - min_x + 1,
                height: max_y - min_y + 1,
                foreground_pixels: fg_pixels,
            })
        } else {
            None
        };

        Ok(ScreenAnalysisReport {
            width,
            height,
            background_color: background,
            foreground_color: foreground,
            contrast_ratio: contrast,
            is_visually_legible: is_legible,
            legibility_rating,
            content_region,
        })
    }
}
