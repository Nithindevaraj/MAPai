pub mod capture;
pub mod analyzer;
pub mod verifier;
pub mod spatial_layout;
pub mod actuator;

pub use capture::{ScreenCaptureEngine, ScreenCapture};
pub use analyzer::{VisualScreenAnalyzer, ScreenAnalysisReport, RgbColor, VisualRegion};
pub use verifier::{VisualVerificationGate, VisualGoalRequest, VisualGateStatus, VisualVerificationResult};
pub use spatial_layout::{SpatialLayoutEngine, SpatialManifest, DisplayMetrics, AppShellHierarchy, ToolStatePerception, RectBounds};
pub use actuator::{SpatialActuator, ActionResult};
