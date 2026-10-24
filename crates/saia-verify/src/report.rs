use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub is_clean: bool,
    pub files_scanned: usize,
    pub lines_scanned: usize,
    pub violations: Vec<StubViolation>,
    pub hollow_completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubViolation {
    pub file_path: String,
    pub line_number: usize,
    pub pattern_type: String,
    pub matched_text: String,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    FatalHollowStub,
    WarningUnverifiedAssumption,
}
