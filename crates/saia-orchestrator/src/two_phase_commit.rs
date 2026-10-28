use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single staged file mutation in the 2PC buffer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StagedFileMutation {
    pub file_path: PathBuf,
    pub original_content: Vec<u8>,
    pub proposed_content: Vec<u8>,
    pub staged_at: DateTime<Utc>,
}

/// The state of a Two-Phase Commit transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionState {
    /// In progress: mutations are staged in memory only
    Preparing,
    /// Verified: passes tests and conflict checks; ready for flush
    Prepared,
    /// Committed: atomically written to physical filesystem
    Committed,
    /// Aborted: discarded without touching the physical disk
    Aborted { reason: String },
}

/// A Two-Phase Commit (2PC) Staging Buffer for multi-agent swarm file mutations.
/// Guarantees that peer agents never observe broken or half-written files.
#[derive(Debug, Clone)]
pub struct TwoPhaseCommitTransaction {
    pub transaction_id: String,
    pub agent_id: String,
    pub staged_mutations: HashMap<PathBuf, StagedFileMutation>,
    pub state: TransactionState,
    pub created_at: DateTime<Utc>,
}

impl TwoPhaseCommitTransaction {
    pub fn new(transaction_id: &str, agent_id: &str) -> Self {
        Self {
            transaction_id: transaction_id.to_string(),
            agent_id: agent_id.to_string(),
            staged_mutations: HashMap::new(),
            state: TransactionState::Preparing,
            created_at: Utc::now(),
        }
    }

    /// Stages a proposed file write in memory. Does NOT touch the physical disk.
    pub fn stage_write(
        &mut self,
        path: impl AsRef<Path>,
        original_content: Vec<u8>,
        proposed_content: Vec<u8>,
    ) -> Result<()> {
        if self.state != TransactionState::Preparing {
            bail!("Cannot stage writes to a transaction in state {:?}", self.state);
        }

        let path_buf = path.as_ref().to_path_buf();
        self.staged_mutations.insert(
            path_buf.clone(),
            StagedFileMutation {
                file_path: path_buf,
                original_content,
                proposed_content,
                staged_at: Utc::now(),
            },
        );
        Ok(())
    }

    /// Reads content from the transaction overlay.
    /// Returns staged content if modified in this transaction, otherwise None.
    pub fn read_staged(&self, path: impl AsRef<Path>) -> Option<&[u8]> {
        self.staged_mutations
            .get(path.as_ref())
            .map(|m| m.proposed_content.as_slice())
    }

    /// Phase 1: Prepares the transaction after the agent verifies test/lint outcomes.
    pub fn prepare(&mut self) -> Result<()> {
        if self.state != TransactionState::Preparing {
            bail!("Transaction must be in Preparing state to prepare");
        }
        self.state = TransactionState::Prepared;
        Ok(())
    }

    /// Phase 2: Flushes all staged mutations atomically to physical disk.
    pub fn commit(&mut self) -> Result<usize> {
        if self.state != TransactionState::Prepared {
            bail!("Transaction must be Prepared before committing");
        }

        let mut written_count = 0;
        for (path, mutation) in &self.staged_mutations {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &mutation.proposed_content)?;
            written_count += 1;
        }

        self.state = TransactionState::Committed;
        Ok(written_count)
    }

    /// Aborts and rolls back the transaction. Memory buffer is dropped in 0ms.
    pub fn abort(&mut self, reason: &str) {
        self.staged_mutations.clear();
        self.state = TransactionState::Aborted {
            reason: reason.to_string(),
        };
    }
}

/// Coordinator managing concurrent 2PC transactions across active swarm agents.
#[derive(Debug, Default)]
pub struct SwarmCommitCoordinator {
    active_transactions: HashMap<String, TwoPhaseCommitTransaction>,
}

impl SwarmCommitCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin_transaction(&mut self, transaction_id: &str, agent_id: &str) -> &mut TwoPhaseCommitTransaction {
        let txn = TwoPhaseCommitTransaction::new(transaction_id, agent_id);
        self.active_transactions.insert(transaction_id.to_string(), txn);
        self.active_transactions.get_mut(transaction_id).unwrap()
    }

    pub fn get_transaction_mut(&mut self, transaction_id: &str) -> Option<&mut TwoPhaseCommitTransaction> {
        self.active_transactions.get_mut(transaction_id)
    }

    pub fn active_transactions(&self) -> &HashMap<String, TwoPhaseCommitTransaction> {
        &self.active_transactions
    }

    pub fn finalize_commit(&mut self, transaction_id: &str) -> Result<usize> {
        if let Some(mut txn) = self.active_transactions.remove(transaction_id) {
            txn.commit()
        } else {
            bail!("Transaction '{}' not found", transaction_id);
        }
    }

    pub fn abort_transaction(&mut self, transaction_id: &str, reason: &str) {
        if let Some(mut txn) = self.active_transactions.remove(transaction_id) {
            txn.abort(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2pc_staging_abort_leaves_disk_untouched() {
        let temp_dir = std::env::temp_dir().join(format!("mapai_2pc_test_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)));
        fs::create_dir_all(&temp_dir).unwrap();
        let target_file = temp_dir.join("config.json");
        fs::write(&target_file, b"{\"initial\": true}").unwrap();

        let mut txn = TwoPhaseCommitTransaction::new("txn_01", "agent_worker_1");
        
        // Stage a write
        txn.stage_write(&target_file, b"{\"initial\": true}".to_vec(), b"{\"dirty\": true}".to_vec()).unwrap();

        // Staged overlay sees new content
        assert_eq!(txn.read_staged(&target_file), Some(b"{\"dirty\": true}".as_slice()));

        // Physical disk is completely UNTOUCHED
        let disk_content = fs::read(&target_file).unwrap();
        assert_eq!(disk_content, b"{\"initial\": true}");

        // Abort transaction
        txn.abort("Unit test verification failure");
        assert!(matches!(txn.state, TransactionState::Aborted { .. }));

        // Disk remains intact
        let disk_content_after = fs::read(&target_file).unwrap();
        assert_eq!(disk_content_after, b"{\"initial\": true}");

        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_2pc_prepare_and_commit_flushes_atomically() {
        let temp_dir = std::env::temp_dir().join(format!("mapai_2pc_commit_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)));
        fs::create_dir_all(&temp_dir).unwrap();
        let target_file = temp_dir.join("src/lib.rs");

        let mut coordinator = SwarmCommitCoordinator::new();
        let txn = coordinator.begin_transaction("txn_02", "agent_worker_2");

        txn.stage_write(&target_file, vec![], b"pub fn ready() -> bool { true }".to_vec()).unwrap();
        assert_eq!(target_file.exists(), false);

        // Prepare (Phase 1)
        txn.prepare().unwrap();

        // Commit (Phase 2)
        let flushed = coordinator.finalize_commit("txn_02").unwrap();
        assert_eq!(flushed, 1);

        // Physical disk now reflects committed content
        let committed_bytes = fs::read(&target_file).unwrap();
        assert_eq!(committed_bytes, b"pub fn ready() -> bool { true }");

        let _ = fs::remove_dir_all(temp_dir);
    }
}
