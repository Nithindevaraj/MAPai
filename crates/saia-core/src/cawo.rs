use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::capsule::PipelineStage;

/// A single immutable node in the Continuity-Aware Workflow Orchestration (CAWO) chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CawoCheckpoint {
    pub step_id: u64,
    pub parent_step_id: Option<u64>,
    pub timestamp: DateTime<Utc>,
    pub stage: PipelineStage,
    
    /// Causal Intent: Formal description of the proposed state mutation
    pub intent: String,
    
    /// Cryptographic anchor: SHA-256 hash of the codebase tree state
    pub codebase_ast_hash: String,
    
    /// Verification status at this step
    pub reality_status: GateStatus,
    
    /// Architectural rationale and trade-off justification
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateStatus {
    Passed,
    Rejected { reasons: Vec<String> },
    EscalatedToHuman { ambiguity: String },
}

/// The append-only continuity journal maintaining causal provenance
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ContinuityJournal {
    pub checkpoints: Vec<CawoCheckpoint>,
}

impl ContinuityJournal {
    pub fn last_valid_checkpoint(&self) -> Option<&CawoCheckpoint> {
        self.checkpoints.iter().rev().find(|c| c.reality_status == GateStatus::Passed)
    }

    pub fn append(&mut self, checkpoint: CawoCheckpoint) {
        self.checkpoints.push(checkpoint);
    }
}
