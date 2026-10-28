use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::capsule::PipelineStage;
use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};
use walkdir::WalkDir;

/// A single immutable node in the Continuity-Aware Workflow Orchestration (CAWO) chain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let p = path.as_ref();
        if !p.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(p)?;
        let journal: Self = serde_json::from_str(&content)?;
        Ok(journal)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let p = path.as_ref();
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(p, content)?;
        Ok(())
    }

    /// Computes deterministic SHA-256 tree hash across non-ignored source files
    pub fn compute_workspace_tree_hash<P: AsRef<Path>>(root: P) -> anyhow::Result<String> {
        let mut hasher = Sha256::new();
        let mut file_entries = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                if e.depth() == 0 {
                    return true;
                }
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') 
                    && name != "target" 
                    && name != "node_modules" 
                    && name != "__pycache__"
                    && name != "AppData"
                    && name != "CrossDevice"
                    && name != "OneDrive"
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::fs::MetadataExt;
                    if let Ok(meta) = entry.metadata() {
                        let attrs = meta.file_attributes();
                        // Skip Windows on-demand cloud placeholder files
                        if (attrs & 0x00400000 != 0) || (attrs & 0x00040000 != 0) || (attrs & 0x00001000 != 0) {
                            continue;
                        }
                    }
                }

                let path = entry.path().to_string_lossy().replace('\\', "/");
                file_entries.push(path);
            }
        }

        // Sort paths for deterministic ordering
        file_entries.sort();

        for file_path in file_entries {
            hasher.update(file_path.as_bytes());
            if let Ok(bytes) = fs::read(&file_path) {
                hasher.update(&bytes);
            }
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuity_journal_append_and_last_valid() {
        let mut journal = ContinuityJournal::default();
        assert!(journal.last_valid_checkpoint().is_none());

        let cp1 = CawoCheckpoint {
            step_id: 1,
            parent_step_id: None,
            timestamp: Utc::now(),
            stage: PipelineStage::Stage1ProblemIntent,
            intent: "Init workspace".into(),
            codebase_ast_hash: "abc123hash".into(),
            reality_status: GateStatus::Passed,
            rationale: Some("Baseline".into()),
        };

        journal.append(cp1.clone());
        assert_eq!(journal.checkpoints.len(), 1);
        assert_eq!(journal.last_valid_checkpoint().unwrap().step_id, 1);

        let cp2 = CawoCheckpoint {
            step_id: 2,
            parent_step_id: Some(1),
            timestamp: Utc::now(),
            stage: PipelineStage::Stage4Implementation,
            intent: "Unverified experimental code".into(),
            codebase_ast_hash: "def456hash".into(),
            reality_status: GateStatus::Rejected { reasons: vec!["Stubs detected".into()] },
            rationale: None,
        };

        journal.append(cp2);
        assert_eq!(journal.checkpoints.len(), 2);
        // last_valid must still be cp1 because cp2 was Rejected
        assert_eq!(journal.last_valid_checkpoint().unwrap().step_id, 1);
    }
}
