#![allow(clippy::manual_c_str_literals)]

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Represents captured screen image data in 32-bit RGBA or BMP format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenCapture {
    pub width: u32,
    pub height: u32,
    pub timestamp: String,
    pub save_path: Option<PathBuf>,
    pub window_title: Option<String>,
}

#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
struct RECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
struct BITMAPINFOHEADER {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn OpenDesktopA(lpszDesktop: *const u8, dwFlags: u32, fInherit: i32, dwDesiredAccess: u32) -> isize;
    fn SetThreadDesktop(hDesktop: isize) -> i32;
    fn EnumWindows(lpEnumFunc: unsafe extern "system" fn(isize, isize) -> i32, lParam: isize) -> i32;
    fn GetWindowTextW(hWnd: isize, lpString: *mut u16, nMaxCount: i32) -> i32;
    fn IsWindowVisible(hWnd: isize) -> i32;
    fn GetWindowRect(hWnd: isize, lpRect: *mut RECT) -> i32;
    fn GetDC(hWnd: isize) -> isize;
    fn ReleaseDC(hWnd: isize, hDC: isize) -> i32;
    fn GetSystemMetrics(nIndex: i32) -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "gdi32")]
extern "system" {
    fn CreateCompatibleDC(hdc: isize) -> isize;
    fn CreateCompatibleBitmap(hdc: isize, cx: i32, cy: i32) -> isize;
    fn SelectObject(hdc: isize, h: isize) -> isize;
    fn BitBlt(hdc: isize, x: i32, y: i32, cx: i32, cy: i32, hdcSrc: isize, x1: i32, y1: i32, rop: u32) -> i32;
    fn DeleteObject(ho: isize) -> i32;
    fn DeleteDC(hdc: isize) -> i32;
    fn GetDIBits(hdc: isize, hbm: isize, start: u32, lines: u32, lpvBits: *mut u8, lpbmi: *mut BITMAPINFOHEADER, usage: u32) -> i32;
}

/// Native Screen Capture Engine for MAPai / SAiA.
pub struct ScreenCaptureEngine;

impl ScreenCaptureEngine {
    /// Captures the primary user screen on the interactive desktop.
    pub fn capture_screen(output_path: &Path) -> anyhow::Result<ScreenCapture> {
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let out_path = output_path.to_path_buf();

        #[cfg(target_os = "windows")]
        {
            let handle = std::thread::spawn(move || -> anyhow::Result<ScreenCapture> {
                unsafe {
                    let h_desk = OpenDesktopA(b"Default\0".as_ptr(), 0, 0, 0x01FF);
                    if h_desk != 0 {
                        SetThreadDesktop(h_desk);
                    }

                    let w = GetSystemMetrics(0);
                    let h = GetSystemMetrics(1);

                    let hdc_screen = GetDC(0);
                    let hdc_mem = CreateCompatibleDC(hdc_screen);
                    let hbm = CreateCompatibleBitmap(hdc_screen, w, h);
                    SelectObject(hdc_mem, hbm);

                    // SRCCOPY = 0x00CC0020
                    BitBlt(hdc_mem, 0, 0, w, h, hdc_screen, 0, 0, 0x00CC0020);

                    let mut bmi = BITMAPINFOHEADER {
                        bi_size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                        bi_width: w,
                        bi_height: -h, // top-down
                        bi_planes: 1,
                        bi_bit_count: 32,
                        bi_compression: 0,
                        bi_size_image: (w * h * 4) as u32,
                        bi_x_pels_per_meter: 0,
                        bi_y_pels_per_meter: 0,
                        bi_clr_used: 0,
                        bi_clr_important: 0,
                    };

                    let buf_size = (w * h * 4) as usize;
                    let mut raw_pixels = vec![0u8; buf_size];

                    GetDIBits(
                        hdc_mem,
                        hbm,
                        0,
                        h as u32,
                        raw_pixels.as_mut_ptr(),
                        &mut bmi,
                        0,
                    );

                    // Write 24-bit / 32-bit BMP file
                    write_bmp_file(&out_path, w as u32, h as u32, &raw_pixels)?;

                    DeleteObject(hbm);
                    DeleteDC(hdc_mem);
                    ReleaseDC(0, hdc_screen);

                    Ok(ScreenCapture {
                        width: w as u32,
                        height: h as u32,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        save_path: Some(out_path),
                        window_title: None,
                    })
                }
            });

            handle.join().map_err(|_| anyhow::anyhow!("Capture thread panicked"))?
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(ScreenCapture {
                width: 1920,
                height: 1080,
                timestamp: chrono::Utc::now().to_rfc3339(),
                save_path: Some(output_path.to_path_buf()),
                window_title: None,
            })
        }
    }

    /// Captures a specific active window by matching its window title.
    pub fn capture_window(title_pattern: &str, output_path: &Path) -> anyhow::Result<ScreenCapture> {
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let out_path = output_path.to_path_buf();
        let pattern = title_pattern.to_lowercase();

        #[cfg(target_os = "windows")]
        {
            let handle = std::thread::spawn(move || -> anyhow::Result<ScreenCapture> {
                unsafe {
                    let h_desk = OpenDesktopA(b"Default\0".as_ptr(), 0, 0, 0x01FF);
                    if h_desk != 0 {
                        SetThreadDesktop(h_desk);
                    }

                    // Enumerate windows to find match
                    static mut TARGET_HWND: isize = 0;
                    static mut TARGET_TITLE: [u16; 256] = [0; 256];
                    static mut PATTERN_PTR: *const String = std::ptr::null();

                    TARGET_HWND = 0;
                    PATTERN_PTR = &pattern as *const String;

                    unsafe extern "system" fn enum_proc(hwnd: isize, _lparam: isize) -> i32 {
                        if IsWindowVisible(hwnd) != 0 {
                            let mut buff = [0u16; 256];
                            let len = GetWindowTextW(hwnd, buff.as_mut_ptr(), 256);
                            if len > 0 {
                                let title = String::from_utf16_lossy(&buff[..len as usize]).to_lowercase();
                                if let Some(pat) = PATTERN_PTR.as_ref() {
                                    if title.contains(pat) {
                                        TARGET_HWND = hwnd;
                                        TARGET_TITLE[..len as usize].copy_from_slice(&buff[..len as usize]);
                                        TARGET_TITLE[len as usize] = 0;
                                        return 0; // stop enumeration
                                    }
                                }
                            }
                        }
                        1 // continue
                    }

                    EnumWindows(enum_proc, 0);

                    if TARGET_HWND == 0 {
                        anyhow::bail!("Window matching '{}' was not found on the interactive desktop", pattern);
                    }

                    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                    GetWindowRect(TARGET_HWND, &mut rect);
                    let w = (rect.right - rect.left).max(100);
                    let h = (rect.bottom - rect.top).max(100);

                    let hdc_screen = GetDC(0);
                    let hdc_mem = CreateCompatibleDC(hdc_screen);
                    let hbm = CreateCompatibleBitmap(hdc_screen, w, h);
                    SelectObject(hdc_mem, hbm);

                    BitBlt(hdc_mem, 0, 0, w, h, hdc_screen, rect.left, rect.top, 0x00CC0020);

                    let mut bmi = BITMAPINFOHEADER {
                        bi_size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                        bi_width: w,
                        bi_height: -h,
                        bi_planes: 1,
                        bi_bit_count: 32,
                        bi_compression: 0,
                        bi_size_image: (w * h * 4) as u32,
                        bi_x_pels_per_meter: 0,
                        bi_y_pels_per_meter: 0,
                        bi_clr_used: 0,
                        bi_clr_important: 0,
                    };

                    let buf_size = (w * h * 4) as usize;
                    let mut raw_pixels = vec![0u8; buf_size];

                    GetDIBits(
                        hdc_mem,
                        hbm,
                        0,
                        h as u32,
                        raw_pixels.as_mut_ptr(),
                        &mut bmi,
                        0,
                    );

                    write_bmp_file(&out_path, w as u32, h as u32, &raw_pixels)?;

                    DeleteObject(hbm);
                    DeleteDC(hdc_mem);
                    ReleaseDC(0, hdc_screen);

                    let mut resolved_title = String::new();
                    let mut title_chars = Vec::new();
                    for &c in (*std::ptr::addr_of!(TARGET_TITLE)).iter() {
                        if c == 0 { break; }
                        title_chars.push(c);
                    }
                    if !title_chars.is_empty() {
                        resolved_title = String::from_utf16_lossy(&title_chars);
                    }

                    Ok(ScreenCapture {
                        width: w as u32,
                        height: h as u32,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        save_path: Some(out_path),
                        window_title: Some(resolved_title),
                    })
                }
            });

            handle.join().map_err(|_| anyhow::anyhow!("Capture thread panicked"))?
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(ScreenCapture {
                width: 1200,
                height: 800,
                timestamp: chrono::Utc::now().to_rfc3339(),
                save_path: Some(output_path.to_path_buf()),
                window_title: Some(title_pattern.to_string()),
            })
        }
    }
}

/// Writes standard 32-bit BMP file directly to disk with proper headers.
fn write_bmp_file(path: &Path, width: u32, height: u32, raw_pixels_bgra: &[u8]) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    let header_size = 14u32;
    let info_size = 40u32;
    let image_size = width * height * 4;
    let file_size = header_size + info_size + image_size;

    // BMP File Header (14 bytes)
    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&0u16.to_le_bytes())?; // reserved1
    file.write_all(&0u16.to_le_bytes())?; // reserved2
    file.write_all(&(header_size + info_size).to_le_bytes())?; // offset to pixels

    // BITMAPINFOHEADER (40 bytes)
    file.write_all(&info_size.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(-(height as i32)).to_le_bytes())?; // top-down
    file.write_all(&1u16.to_le_bytes())?; // planes
    file.write_all(&32u16.to_le_bytes())?; // 32 bpp
    file.write_all(&0u32.to_le_bytes())?; // compression BI_RGB
    file.write_all(&image_size.to_le_bytes())?;
    file.write_all(&0i32.to_le_bytes())?; // x ppm
    file.write_all(&0i32.to_le_bytes())?; // y ppm
    file.write_all(&0u32.to_le_bytes())?; // clr used
    file.write_all(&0u32.to_le_bytes())?; // clr important

    // Pixel data
    file.write_all(raw_pixels_bgra)?;
    file.flush()?;

    Ok(())
}
