use super::analyzer::{VisualScreenAnalyzer, ScreenAnalysisReport};
use super::capture::ScreenCaptureEngine;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualGoalRequest {
    pub window_title_pattern: String,
    pub expected_text: Option<String>,
    pub required_color: Option<String>,
    pub minimum_contrast_ratio: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualGateStatus {
    Verified,
    WindowNotVisible,
    BlankContent,
    InsufficientContrast,
    ColorMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualVerificationResult {
    pub status: VisualGateStatus,
    pub is_passed: bool,
    pub window_title: String,
    pub contrast_ratio: f32,
    pub is_legible: bool,
    pub summary: String,
    pub remediation: Option<String>,
    pub analysis: Option<ScreenAnalysisReport>,
}

pub struct VisualVerificationGate;

impl VisualVerificationGate {
    /// Executes a closed-loop visual IV&V audit against an active desktop application.
    pub fn verify_visual_goal(
        request: &VisualGoalRequest,
        temp_screenshot_path: &Path,
    ) -> anyhow::Result<VisualVerificationResult> {
        // Step 1: Capture the target window
        let capture_result = ScreenCaptureEngine::capture_window(
            &request.window_title_pattern,
            temp_screenshot_path,
        );

        let capture = match capture_result {
            Ok(c) => c,
            Err(e) => {
                return Ok(VisualVerificationResult {
                    status: VisualGateStatus::WindowNotVisible,
                    is_passed: false,
                    window_title: request.window_title_pattern.clone(),
                    contrast_ratio: 1.0,
                    is_legible: false,
                    summary: format!("Target window '{}' could not be found or captured on the user's interactive desktop: {}", request.window_title_pattern, e),
                    remediation: Some(format!("Ensure '{}' is launched on the active user desktop (WinSta0\\Default) using Windows Shell.", request.window_title_pattern)),
                    analysis: None,
                });
            }
        };

        // Step 2: Analyze the visual pixels of the captured window
        let analysis = VisualScreenAnalyzer::analyze_bmp(temp_screenshot_path)?;

        // Step 3: Check for blank content
        if analysis.content_region.is_none() {
            return Ok(VisualVerificationResult {
                status: VisualGateStatus::BlankContent,
                is_passed: false,
                window_title: capture.window_title.unwrap_or_else(|| request.window_title_pattern.clone()),
                contrast_ratio: analysis.contrast_ratio,
                is_legible: false,
                summary: "Window is open, but canvas/client area is completely blank. No text or shapes were committed.".to_string(),
                remediation: Some("Ensure the Text Tool clicked inside the canvas and input was committed by clicking outside the text box or pressing Enter.".to_string()),
                analysis: Some(analysis),
            });
        }

        // Step 4: Verify contrast and color requirements
        let min_contrast = request.minimum_contrast_ratio.unwrap_or(3.0);
        if analysis.contrast_ratio < min_contrast {
            return Ok(VisualVerificationResult {
                status: VisualGateStatus::InsufficientContrast,
                is_passed: false,
                window_title: capture.window_title.unwrap_or_else(|| request.window_title_pattern.clone()),
                contrast_ratio: analysis.contrast_ratio,
                is_legible: false,
                summary: format!(
                    "Text/content rendered with insufficient contrast ({:.2}:1, required >= {:.2}:1). The text is virtually invisible against the background.",
                    analysis.contrast_ratio, min_contrast
                ),
                remediation: Some("Set Color 1 explicitly to Black (#000000) or high-contrast foreground color before typing or drawing.".to_string()),
                analysis: Some(analysis),
            });
        }

        // Step 5: Check explicit color requirement (e.g. "black", "red")
        if let Some(ref req_color) = request.required_color {
            let req_lower = req_color.to_lowercase();
            let fg = analysis.foreground_color;

            if req_lower == "black" {
                let is_dark_enough = (fg.r as u32 + fg.g as u32 + fg.b as u32) / 3 < 60;
                if !is_dark_enough {
                    return Ok(VisualVerificationResult {
                        status: VisualGateStatus::ColorMismatch,
                        is_passed: false,
                        window_title: capture.window_title.unwrap_or_else(|| request.window_title_pattern.clone()),
                        contrast_ratio: analysis.contrast_ratio,
                        is_legible: analysis.is_visually_legible,
                        summary: format!(
                            "Rendered foreground color rgb({}, {}, {}) does not match required black font.",
                            fg.r, fg.g, fg.b
                        ),
                        remediation: Some("Select the primary color palette index 0 (Black) in the application toolbar.".to_string()),
                        analysis: Some(analysis),
                    });
                }
            } else if req_lower == "red" {
                let is_red_dominant = fg.r > 130 && fg.r > fg.g.saturating_add(30) && fg.r > fg.b.saturating_add(30);
                if !is_red_dominant {
                    return Ok(VisualVerificationResult {
                        status: VisualGateStatus::ColorMismatch,
                        is_passed: false,
                        window_title: capture.window_title.unwrap_or_else(|| request.window_title_pattern.clone()),
                        contrast_ratio: analysis.contrast_ratio,
                        is_legible: analysis.is_visually_legible,
                        summary: format!(
                            "Rendered foreground color rgb({}, {}, {}) does not match required red font.",
                            fg.r, fg.g, fg.b
                        ),
                        remediation: Some("Select the Red color swatch in the application toolbar palette.".to_string()),
                        analysis: Some(analysis),
                    });
                }
            }
        }

        // Goal successfully verified!
        let region = analysis.content_region.as_ref().unwrap();
        Ok(VisualVerificationResult {
            status: VisualGateStatus::Verified,
            is_passed: true,
            window_title: capture.window_title.unwrap_or_else(|| request.window_title_pattern.clone()),
            contrast_ratio: analysis.contrast_ratio,
            is_legible: analysis.is_visually_legible,
            summary: format!(
                "VISUALLY VERIFIED: Content detected in {}x{} region (Contrast {:.2}:1, {}).",
                region.width, region.height, analysis.contrast_ratio, analysis.legibility_rating
            ),
            remediation: None,
            analysis: Some(analysis),
        })
    }
}
