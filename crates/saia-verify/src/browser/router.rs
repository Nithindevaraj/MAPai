use serde::{Deserialize, Serialize};
use super::headless_scout::HeadlessScoutEngine;
use super::real_browser_cdp::RealBrowserCdpEngine;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowseExecutionMode {
    AutoTiered,       // Headless-first with auto-escalation (Default)
    HeadlessOnly,     // Strictly headless
    RealBrowserOnly,  // Directly engage user's active browser
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowseRequest {
    pub url: String,
    pub mode: BrowseExecutionMode,
    pub cdp_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridBrowseResponse {
    pub final_url: String,
    pub title: String,
    pub markdown: String,
    pub tier_used: String,
    pub escalated: bool,
    pub escalation_reason: Option<String>,
}

pub struct HybridBrowserRouter;

impl HybridBrowserRouter {
    /// Executes browsing following the Tiered Dual-Engine Strategy
    pub fn execute(request: BrowseRequest) -> anyhow::Result<HybridBrowseResponse> {
        let cdp_port = if request.cdp_port == 0 { 9222 } else { request.cdp_port };

        // If forced to real browser directly
        if request.mode == BrowseExecutionMode::RealBrowserOnly {
            let real_res = RealBrowserCdpEngine::navigate_and_read(cdp_port, &request.url)?;
            return Ok(HybridBrowseResponse {
                final_url: real_res.url,
                title: real_res.title,
                markdown: real_res.page_content_markdown,
                tier_used: "Tier 2: Real User Browser (CDP)".to_string(),
                escalated: false,
                escalation_reason: None,
            });
        }

        // Tier 1: Attempt Headless first
        let headless_res = HeadlessScoutEngine::browse(&request.url)?;

        // Check if escalation is required
        let should_escalate = request.mode == BrowseExecutionMode::AutoTiered && (
            headless_res.requires_auth || headless_res.has_captcha
        );

        if should_escalate {
            let reason = if headless_res.requires_auth {
                "Authentication or Login Redirect Detected"
            } else {
                "Bot Protection / CAPTCHA Challenge Detected"
            };

            // Escalate to Tier 2 Real User Browser
            match RealBrowserCdpEngine::navigate_and_read(cdp_port, &request.url) {
                Ok(real_res) => {
                    return Ok(HybridBrowseResponse {
                        final_url: real_res.url,
                        title: real_res.title,
                        markdown: real_res.page_content_markdown,
                        tier_used: "Tier 2: Real User Browser (Escalated)".to_string(),
                        escalated: true,
                        escalation_reason: Some(reason.to_string()),
                    });
                }
                Err(e) => {
                    // Fallback to headless with advisory if real browser launch failed
                    return Ok(HybridBrowseResponse {
                        final_url: headless_res.url,
                        title: headless_res.title,
                        markdown: format!("{}\n\n> [!WARNING] Page requires authentication. Real browser escalation attempted but failed: {}", headless_res.markdown, e),
                        tier_used: "Tier 1: Headless (Auth Blocked)".to_string(),
                        escalated: true,
                        escalation_reason: Some(format!("{} (Escalation failed)", reason)),
                    });
                }
            }
        }

        // Successfully resolved via Tier 1 Headless
        Ok(HybridBrowseResponse {
            final_url: headless_res.url,
            title: headless_res.title,
            markdown: headless_res.markdown,
            tier_used: "Tier 1: Silent Headless Engine".to_string(),
            escalated: false,
            escalation_reason: None,
        })
    }
}
