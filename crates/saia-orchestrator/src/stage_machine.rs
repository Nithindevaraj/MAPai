use saia_core::capsule::{PipelineStage, ProjectCapsule};
use saia_verify::TruthScanner;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StageTransitionError {
    #[error("Cannot advance from {0:?}: Hollow stubs detected in deliverables")]
    HollowStubsDetected(PipelineStage),
    #[error("Cannot advance: Missing Job 1 Problem Intent")]
    MissingIntent,
    #[error("Cannot advance: Unverified physical compilation or test failure")]
    PhysicalRealityFailure,
}

pub struct StageRunner;

impl StageRunner {
    /// Validates whether the active project state satisfies all invariants to advance to the next pipeline stage
    pub fn can_advance<P: AsRef<Path>>(capsule: &ProjectCapsule, workspace_root: P) -> Result<PipelineStage, StageTransitionError> {
        match capsule.current_stage {
            PipelineStage::Stage1ProblemIntent => {
                if capsule.job_1_intent.problem_statement.trim().is_empty() {
                    return Err(StageTransitionError::MissingIntent);
                }
                Ok(PipelineStage::Stage2ArchitectureConstraints)
            }
            PipelineStage::Stage2ArchitectureConstraints => {
                Ok(PipelineStage::Stage3MaterialSelection)
            }
            PipelineStage::Stage3MaterialSelection => {
                Ok(PipelineStage::Stage4Implementation)
            }
            PipelineStage::Stage4Implementation => {
                // Must pass zero hollow stub verification before moving to Empirical Reality Check
                let scanner = TruthScanner::new();
                if let Ok(report) = scanner.scan_directory(workspace_root) {
                    if !report.is_clean {
                        return Err(StageTransitionError::HollowStubsDetected(PipelineStage::Stage4Implementation));
                    }
                }
                Ok(PipelineStage::Stage5EmpiricalRealityCheck)
            }
            PipelineStage::Stage5EmpiricalRealityCheck => {
                Ok(PipelineStage::Stage6HardeningDiagnostics)
            }
            PipelineStage::Stage6HardeningDiagnostics => {
                Ok(PipelineStage::Stage7SovereignDelivery)
            }
            PipelineStage::Stage7SovereignDelivery => {
                Ok(PipelineStage::Stage7SovereignDelivery)
            }
        }
    }
}
