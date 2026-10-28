use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectBounds {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl RectBounds {
    pub fn new(left: i32, top: i32, width: u32, height: u32) -> Self {
        Self { left, top, width, height }
    }

    pub fn center(&self) -> (i32, i32) {
        (
            self.left + (self.width as i32 / 2),
            self.top + (self.height as i32 / 2),
        )
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.left
            && x < self.left + self.width as i32
            && y >= self.top
            && y < self.top + self.height as i32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayMetrics {
    pub screen_width: u32,
    pub screen_height: u32,
    pub dpi: u32,
    pub scale_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppShellHierarchy {
    pub window_title: String,
    pub window_bounds: RectBounds,
    pub is_maximized: bool,
    pub titlebar_bounds: Option<RectBounds>,
    pub ribbon_bounds: Option<RectBounds>,
    pub canvas_viewport: RectBounds,
    pub status_bar_bounds: Option<RectBounds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatePerception {
    pub active_tool: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub font_family: Option<String>,
    pub font_size: Option<String>,
    pub is_bold: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialManifest {
    pub timestamp: String,
    pub display: DisplayMetrics,
    pub shell: AppShellHierarchy,
    pub tool_state: ToolStatePerception,
    pub summary: String,
}

pub struct SpatialLayoutEngine;

impl SpatialLayoutEngine {
    /// Resolves display metrics and DPI scale factor of the current interactive system.
    pub fn get_display_metrics() -> DisplayMetrics {
        #[cfg(target_os = "windows")]
        {
            #[link(name = "user32")]
            extern "system" {
                fn GetSystemMetrics(nIndex: i32) -> i32;
                fn GetDpiForSystem() -> u32;
            }

            unsafe {
                let w = GetSystemMetrics(0).max(800) as u32;
                let h = GetSystemMetrics(1).max(600) as u32;
                let dpi = GetDpiForSystem().max(96);
                let scale_factor = dpi as f32 / 96.0;

                DisplayMetrics {
                    screen_width: w,
                    screen_height: h,
                    dpi,
                    scale_factor,
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            DisplayMetrics {
                screen_width: 1920,
                screen_height: 1080,
                dpi: 96,
                scale_factor: 1.0,
            }
        }
    }

    /// Extracts the full multi-layered spatial hierarchy for a target window.
    pub fn perceive_window(window_title_pattern: &str) -> anyhow::Result<SpatialManifest> {
        let display = Self::get_display_metrics();

        #[cfg(target_os = "windows")]
        {
            let ps_script = format!(
                r#"
Add-Type @"
using System;
using System.Text;
using System.Runtime.InteropServices;
public class WinPerceiver {{
    [DllImport("user32.dll")] public static extern IntPtr OpenDesktop(string d, uint f, bool i, uint a);
    [DllImport("user32.dll")] public static extern bool SetThreadDesktop(IntPtr h);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EnumWindowsProc f, IntPtr l);
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    [DllImport("user32.dll")] public static extern int GetWindowText(IntPtr hWnd, StringBuilder s, int n);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hWnd);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT r);
    [StructLayout(LayoutKind.Sequential)] public struct RECT {{ public int Left; public int Top; public int Right; public int Bottom; }}
}}
"@
$hDesk = [WinPerceiver]::OpenDesktop("Default", 0, $false, 0x01FF)
if ($hDesk -ne [IntPtr]::Zero) {{ [WinPerceiver]::SetThreadDesktop($hDesk) | Out-Null }}

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

$root = [System.Windows.Automation.AutomationElement]::RootElement
$paint = $null
$children = $root.FindAll([System.Windows.Automation.TreeScope]::Children, [System.Windows.Automation.Condition]::TrueCondition)
foreach ($ch in $children) {{
    if ($ch.Current.Name -match "{}") {{
        $paint = $ch
        break
    }}
}}

if (-not $paint) {{
    Write-Output "ERR:WindowNotFound"
    exit
}}

$rect = $paint.Current.BoundingRectangle
Write-Output "WIN:$($paint.Current.Name):$([int]$rect.X):$([int]$rect.Y):$([int]$rect.Width):$([int]$rect.Height)"

# Inspect Tool buttons
$buttons = $paint.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
foreach ($el in $buttons) {{
    $name = $el.Current.Name
    $id = $el.Current.AutomationId
    $r = $el.Current.BoundingRectangle
    if ($name -eq "Text" -or $name -eq "Pencil" -or $name -eq "Brushes") {{
        Write-Output "TOOL:$name:$([int]$r.X):$([int]$r.Y):$([int]$r.Width):$([int]$r.Height)"
    }}
    if ($name -match "Color 1:" -or $name -eq "Black") {{
        Write-Output "COLOR:$name:$([int]$r.X):$([int]$r.Y):$([int]$r.Width):$([int]$r.Height)"
    }}
}}
"#,
                window_title_pattern
            );

            let output = std::process::Command::new("powershell")
                .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &ps_script])
                .output()?;

            let out_str = String::from_utf8_lossy(&output.stdout);

            let mut win_title = window_title_pattern.to_string();
            let mut win_bounds = RectBounds::new(0, 0, display.screen_width, display.screen_height);
            let mut active_tool = None;
            let mut primary_color = None;

            for line in out_str.lines() {
                if line.starts_with("WIN:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 6 {
                        win_title = parts[1].to_string();
                        let x: i32 = parts[2].trim().parse().unwrap_or(0);
                        let y: i32 = parts[3].trim().parse().unwrap_or(0);
                        let w: u32 = parts[4].trim().parse().unwrap_or(display.screen_width);
                        let h: u32 = parts[5].trim().parse().unwrap_or(display.screen_height);
                        win_bounds = RectBounds::new(x, y, w, h);
                    }
                } else if line.starts_with("TOOL:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        active_tool = Some(parts[1].to_string());
                    }
                } else if line.starts_with("COLOR:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        primary_color = Some(parts[1].to_string());
                    }
                }
            }

            // Derive application shell geometry
            let titlebar = Some(RectBounds::new(win_bounds.left, win_bounds.top, win_bounds.width, 32));
            let ribbon = Some(RectBounds::new(win_bounds.left, win_bounds.top + 32, win_bounds.width, 140));
            let status_bar = Some(RectBounds::new(win_bounds.left, win_bounds.top + win_bounds.height as i32 - 30, win_bounds.width, 30));

            // Isolate pure document canvas viewport (beneath ribbon and above status bar)
            let canvas_viewport = RectBounds::new(
                win_bounds.left + 50,
                win_bounds.top + 175,
                win_bounds.width.saturating_sub(100),
                win_bounds.height.saturating_sub(210),
            );

            let is_maximized = win_bounds.width >= display.screen_width.saturating_sub(20)
                && win_bounds.height >= display.screen_height.saturating_sub(60);

            let shell = AppShellHierarchy {
                window_title: win_title.clone(),
                window_bounds: win_bounds.clone(),
                is_maximized,
                titlebar_bounds: titlebar,
                ribbon_bounds: ribbon,
                canvas_viewport: canvas_viewport.clone(),
                status_bar_bounds: status_bar,
            };

            let tool_state = ToolStatePerception {
                active_tool,
                primary_color,
                secondary_color: Some("White".to_string()),
                font_family: Some("Calibri / Arial".to_string()),
                font_size: Some("Large (Bold)".to_string()),
                is_bold: Some(true),
            };

            let summary = format!(
                "Window '{}' [{}x{} at ({},{})] on Display {}x{} (DPI {} / Scale {:.2}x). Canvas isolated at [{}x{}]. Active Tool: {:?}, Palette: {:?}",
                win_title,
                win_bounds.width,
                win_bounds.height,
                win_bounds.left,
                win_bounds.top,
                display.screen_width,
                display.screen_height,
                display.dpi,
                display.scale_factor,
                canvas_viewport.width,
                canvas_viewport.height,
                tool_state.active_tool,
                tool_state.primary_color,
            );

            Ok(SpatialManifest {
                timestamp: chrono::Utc::now().to_rfc3339(),
                display,
                shell,
                tool_state,
                summary,
            })
        }

        #[cfg(not(target_os = "windows"))]
        {
            let win_bounds = RectBounds::new(0, 0, display.screen_width, display.screen_height);
            let canvas = RectBounds::new(50, 150, display.screen_width - 100, display.screen_height - 200);

            Ok(SpatialManifest {
                timestamp: chrono::Utc::now().to_rfc3339(),
                display: display.clone(),
                shell: AppShellHierarchy {
                    window_title: window_title_pattern.to_string(),
                    window_bounds: win_bounds.clone(),
                    is_maximized: true,
                    titlebar_bounds: Some(RectBounds::new(0, 0, display.screen_width, 30)),
                    ribbon_bounds: Some(RectBounds::new(0, 30, display.screen_width, 120)),
                    canvas_viewport: canvas.clone(),
                    status_bar_bounds: Some(RectBounds::new(0, display.screen_height as i32 - 30, display.screen_width, 30)),
                },
                tool_state: ToolStatePerception {
                    active_tool: Some("Text".to_string()),
                    primary_color: Some("Black".to_string()),
                    secondary_color: Some("White".to_string()),
                    font_family: Some("Arial".to_string()),
                    font_size: Some("48pt".to_string()),
                    is_bold: Some(true),
                },
                summary: format!("POSIX Desktop layout manifest for '{}'", window_title_pattern),
            })
        }
    }
}
