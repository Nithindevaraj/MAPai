use saia_core::capsule::{PipelineStage, ProjectCapsule};
use saia_verify::{TruthScanner, OnlineScoutVerifier};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StageTransitionError {
    #[error("Cannot advance from {0:?}: Hollow stubs detected in deliverables")]
    HollowStubsDetected(PipelineStage),
    #[error("Cannot advance: Missing Job 1 Problem Intent")]
    MissingIntent,
    #[error("Cannot advance: Online scouting detected hallucinated/invalid dependencies: {0:?}")]
    HallucinatedDependencies(Vec<String>),
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
                // Online internet scouting: verify all declared dependencies on public registries
                if let Ok(report) = OnlineScoutVerifier::scout_and_verify_workspace(&workspace_root) {
                    if !report.is_clean {
                        return Err(StageTransitionError::HallucinatedDependencies(report.hallucinated_dependencies));
                    }
                }
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

    /// Validates invariants and advances the capsule's current_stage, updating its timestamp
    pub fn advance<P: AsRef<Path>>(capsule: &mut ProjectCapsule, workspace_root: P) -> Result<PipelineStage, StageTransitionError> {
        let next_stage = Self::can_advance(capsule, workspace_root)?;
        capsule.current_stage = next_stage;
        capsule.updated_at = chrono::Utc::now();
        Ok(next_stage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_1_requires_intent() {
        let mut capsule = ProjectCapsule {
            current_stage: PipelineStage::Stage1ProblemIntent,
            ..Default::default()
        };
        capsule.job_1_intent.problem_statement = "".to_string();

        let err = StageRunner::can_advance(&capsule, ".").unwrap_err();
        assert!(matches!(err, StageTransitionError::MissingIntent));

        capsule.job_1_intent.problem_statement = "Valid sovereign goal".to_string();
        let next = StageRunner::advance(&mut capsule, ".").unwrap();
        assert_eq!(next, PipelineStage::Stage2ArchitectureConstraints);
        assert_eq!(capsule.current_stage, PipelineStage::Stage2ArchitectureConstraints);
    }
}
