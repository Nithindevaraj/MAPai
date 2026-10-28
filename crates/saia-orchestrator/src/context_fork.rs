use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an individual turn in the conversation or execution stream.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextTurn {
    pub turn_index: usize,
    pub role: String,
    pub content: String,
    pub token_count: usize,
    pub timestamp: DateTime<Utc>,
}

/// An immutable, frozen context snapshot representing the parent session state.
#[derive(Debug, Clone)]
pub struct ContextSnapshot {
    pub snapshot_id: String,
    pub base_turns: Arc<Vec<ContextTurn>>,
    pub total_base_tokens: usize,
    pub created_at: DateTime<Utc>,
}

/// A lightweight, in-process forked subagent branch that inherits the parent's
/// immutable context prefix without duplicating tokens in prompt re-inflation.
#[derive(Debug, Clone)]
pub struct ForkedSubagentBranch {
    pub branch_id: String,
    pub parent_snapshot: ContextSnapshot,
    pub speculative_turns: Vec<ContextTurn>,
    pub status: BranchStatus,
    pub tokens_saved_by_forking: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchStatus {
    Active,
    Committed,
    Discarded { reason: String },
}

impl ForkedSubagentBranch {
    /// Creates a new speculative branch from an immutable parent context snapshot.
    pub fn fork_from(branch_id: &str, snapshot: ContextSnapshot) -> Self {
        // Tokens saved equals the frozen prefix tokens that do not need to be re-sent
        let tokens_saved = snapshot.total_base_tokens;
        Self {
            branch_id: branch_id.to_string(),
            parent_snapshot: snapshot,
            speculative_turns: Vec::new(),
            status: BranchStatus::Active,
            tokens_saved_by_forking: tokens_saved,
        }
    }

    /// Records a new turn executed speculatively within the branch.
    pub fn push_speculative_turn(&mut self, role: &str, content: &str, token_count: usize) {
        if self.status != BranchStatus::Active {
            return;
        }
        let turn_idx = self.parent_snapshot.base_turns.len() + self.speculative_turns.len();
        self.speculative_turns.push(ContextTurn {
            turn_index: turn_idx,
            role: role.to_string(),
            content: content.to_string(),
            token_count,
            timestamp: Utc::now(),
        });
    }

    /// Merges and commits the speculative branch outcomes back to the parent session.
    pub fn commit_to_parent(mut self) -> Result<Vec<ContextTurn>, String> {
        if self.status != BranchStatus::Active {
            return Err("Branch is not active".to_string());
        }
        self.status = BranchStatus::Committed;
        Ok(self.speculative_turns)
    }

    /// Discards the branch upon failure or cancellation without polluting the parent state.
    pub fn discard(&mut self, reason: &str) {
        self.status = BranchStatus::Discarded {
            reason: reason.to_string(),
        };
        self.speculative_turns.clear();
    }

    /// Returns the effective token savings percentage achieved by forking vs cold-spawning.
    pub fn token_savings_ratio(&self) -> f64 {
        let speculative_tokens: usize = self.speculative_turns.iter().map(|t| t.token_count).sum();
        let total_context = self.parent_snapshot.total_base_tokens + speculative_tokens;
        if total_context == 0 {
            0.0
        } else {
            (self.tokens_saved_by_forking as f64) / (total_context as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_fork_token_savings_and_commit() {
        // 1. Setup parent session with 10,000 baseline tokens
        let parent_turns = vec![
            ContextTurn {
                turn_index: 0,
                role: "system".into(),
                content: "MAPai Core Instructions".into(),
                token_count: 4000,
                timestamp: Utc::now(),
            },
            ContextTurn {
                turn_index: 1,
                role: "user".into(),
                content: "Refactor database engine".into(),
                token_count: 6000,
                timestamp: Utc::now(),
            },
        ];

        let snapshot = ContextSnapshot {
            snapshot_id: "snap_101".into(),
            base_turns: Arc::new(parent_turns),
            total_base_tokens: 10000,
            created_at: Utc::now(),
        };

        // 2. Fork speculative subagent
        let mut branch = ForkedSubagentBranch::fork_from("fork_agent_01", snapshot);
        assert_eq!(branch.tokens_saved_by_forking, 10000);

        // 3. Child executes 2 speculative turns (1,500 tokens)
        branch.push_speculative_turn("assistant", "Running test harness", 500);
        branch.push_speculative_turn("tool", "Tests verified successfully", 1000);

        // Verify savings ratio: 10,000 / 11,500 = ~86.9% token savings
        assert!(branch.token_savings_ratio() > 0.85);

        // 4. Commit results back to parent
        let committed = branch.commit_to_parent().unwrap();
        assert_eq!(committed.len(), 2);
        assert_eq!(committed[0].turn_index, 2);
        assert_eq!(committed[1].turn_index, 3);
    }
}
