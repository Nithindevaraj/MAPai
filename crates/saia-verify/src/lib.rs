pub mod scanner;
pub mod report;

pub use scanner::TruthScanner;
pub use report::{VerificationReport, StubViolation, ViolationSeverity};
