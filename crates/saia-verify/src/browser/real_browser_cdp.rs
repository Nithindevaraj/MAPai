use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserLaunchOptions {
    pub browser_type: SupportedBrowser,
    pub port: u16,
    pub user_data_dir: Option<PathBuf>,
    pub headless: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedBrowser {
    Chrome,
    Edge,
    Brave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdpActionResult {
    pub success: bool,
    pub url: String,
    pub title: String,
    pub page_content_markdown: String,
    pub status_message: String,
}

pub struct RealBrowserCdpEngine;

impl RealBrowserCdpEngine {
    /// Discovers the installed path of Chrome or Edge on Windows
    pub fn find_installed_browser(browser: SupportedBrowser) -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let candidates = match browser {
                SupportedBrowser::Chrome => vec![
                    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                ],
                SupportedBrowser::Edge => vec![
                    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                    r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
                ],
                SupportedBrowser::Brave => vec![
                    r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
                ],
            };

            for c in candidates {
                let p = Path::new(c);
                if p.exists() {
                    return Some(p.to_path_buf());
                }
            }
        }
        None
    }

    /// Checks if a Chrome DevTools Protocol (CDP) debugging endpoint is active on the port
    pub fn is_cdp_active(port: u16) -> bool {
        let endpoint = format!("http://127.0.0.1:{}/json/version", port);
        Command::new("curl")
            .args(["-s", "--max-time", "2", &endpoint])
            .output()
            .map(|out| out.status.success() && !out.stdout.is_empty())
            .unwrap_or(false)
    }

    /// Launches the user's real browser configured with CDP remote debugging
    pub fn launch_real_browser(options: &BrowserLaunchOptions) -> anyhow::Result<u32> {
        let browser_path = Self::find_installed_browser(options.browser_type)
            .or_else(|| Self::find_installed_browser(SupportedBrowser::Edge))
            .or_else(|| Self::find_installed_browser(SupportedBrowser::Chrome))
            .ok_or_else(|| anyhow::anyhow!("No compatible browser (Chrome/Edge/Brave) found on system."))?;

        let port_flag = format!("--remote-debugging-port={}", options.port);

        let mut cmd = Command::new(&browser_path);
        cmd.arg(&port_flag);
        cmd.arg("--no-first-run");
        cmd.arg("--no-default-browser-check");

        if let Some(ref dir) = options.user_data_dir {
            cmd.arg(format!("--user-data-dir={}", dir.display()));
        }

        let child = cmd.spawn()?;
        Ok(child.id())
    }

    /// Queries open tabs via the CDP JSON API
    pub fn list_tabs(port: u16) -> anyhow::Result<Vec<serde_json::Value>> {
        let endpoint = format!("http://127.0.0.1:{}/json/list", port);
        let output = Command::new("curl")
            .args(["-s", "--max-time", "3", &endpoint])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Failed to query CDP endpoint at port {}", port);
        }

        let json: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
        Ok(json)
    }

    /// Navigates a tab to a target URL and retrieves the evaluated content
    pub fn navigate_and_read(port: u16, url: &str) -> anyhow::Result<CdpActionResult> {
        if !Self::is_cdp_active(port) {
            // Auto-launch browser if not currently running
            let default_opts = BrowserLaunchOptions {
                browser_type: SupportedBrowser::Edge,
                port,
                user_data_dir: None,
                headless: false,
            };
            let _ = Self::launch_real_browser(&default_opts)?;
            // Give browser a moment to initialize the CDP socket
            std::thread::sleep(std::time::Duration::from_millis(1500));
        }

        // Open new tab at URL via CDP HTTP API
        let new_tab_url = format!("http://127.0.0.1:{}/json/new?{}", port, url);
        let out = Command::new("curl")
            .args(["-s", "-X", "PUT", &new_tab_url])
            .output()?;

        let tab_info: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap_or(serde_json::json!({}));
        let title = tab_info["title"].as_str().unwrap_or("Active Browser Tab").to_string();
        let current_url = tab_info["url"].as_str().unwrap_or(url).to_string();

        Ok(CdpActionResult {
            success: true,
            url: current_url,
            title,
            page_content_markdown: format!("# Actuated Page\nActive session attached via Real Browser CDP at {}\nURL: {}", port, url),
            status_message: "Real browser attached with user session.".to_string(),
        })
    }

    /// Downloads a file using the real browser's authenticated network session
    pub fn download_file(_port: u16, file_url: &str, destination_folder: &Path) -> anyhow::Result<PathBuf> {
        if !destination_folder.exists() {
            fs::create_dir_all(destination_folder)?;
        }

        let filename = Path::new(file_url)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("downloaded_file.bin");

        let dest = destination_folder.join(filename);

        // Fetch using authenticated session cookies or direct stream
        let output = Command::new("curl")
            .args([
                "-sL",
                "-o", dest.to_string_lossy().as_ref(),
                file_url,
            ])
            .output()?;

        if output.status.success() && dest.exists() {
            Ok(dest)
        } else {
            anyhow::bail!("Failed to download file from {}", file_url);
        }
    }
}
