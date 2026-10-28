pub mod headless_scout;
pub mod real_browser_cdp;
pub mod router;

pub use headless_scout::{HeadlessScoutEngine, HeadlessBrowseResult};
pub use real_browser_cdp::{RealBrowserCdpEngine, BrowserLaunchOptions, SupportedBrowser, CdpActionResult};
pub use router::{HybridBrowserRouter, BrowseRequest, BrowseExecutionMode, HybridBrowseResponse};
