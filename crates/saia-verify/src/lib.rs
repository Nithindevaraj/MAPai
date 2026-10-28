pub mod scanner;
pub mod report;
pub mod online_scout;
pub mod omni_scout;
pub mod browser;
pub mod screen;
pub mod blast_radius;
pub mod secret_mask;

pub use scanner::TruthScanner;
pub use report::{VerificationReport, StubViolation, ViolationSeverity};
pub use online_scout::{OnlineScoutVerifier, OnlineScoutReport};
pub use omni_scout::{OmniScoutEngine, OmniScoutMatrixReport, ScoutDomain, ScoutStatus, ScoutVectorFinding};
pub use browser::{HybridBrowserRouter, BrowseRequest, BrowseExecutionMode, HybridBrowseResponse, HeadlessScoutEngine, RealBrowserCdpEngine};
pub use screen::{ScreenCaptureEngine, ScreenCapture, VisualScreenAnalyzer, ScreenAnalysisReport, RgbColor, VisualVerificationGate, VisualGoalRequest, VisualGateStatus, VisualVerificationResult, SpatialLayoutEngine, SpatialManifest, DisplayMetrics, AppShellHierarchy, ToolStatePerception, RectBounds};
pub use blast_radius::{BlastRadiusGuard, RiskFinding, RiskLevel};
pub use secret_mask::SecretMasker;
