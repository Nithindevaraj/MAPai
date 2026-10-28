use crate::collision_tracker::{ReadSetCollisionTracker, CollisionEvent};
use crate::context_fork::{ContextSnapshot, ForkedSubagentBranch, ContextTurn};
use crate::rpc_batcher::{RpcBatchDispatcher, ToolCallInvocation, RpcBatchSummary};
use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::Utc;

/// Unified MAPai Orchestration Harness combining:
/// 1. Swarm Read-Set Collision Tracking
/// 2. In-Process Context Forking
/// 3. Tool RPC Batch Collapsing
#[derive(Default)]
pub struct MapaiHarness {
    /// Active multi-agent read-set collision bus
    pub collision_tracker: Arc<Mutex<ReadSetCollisionTracker>>,
    /// Sandboxed tool batch execution dispatcher
    pub rpc_dispatcher: RpcBatchDispatcher,
}

impl MapaiHarness {
    pub fn new() -> Self {
        Self {
            collision_tracker: Arc::new(Mutex::new(ReadSetCollisionTracker::new())),
            rpc_dispatcher: RpcBatchDispatcher::new(),
        }
    }

    /// Creates a speculative subagent branch sharing the parent context prefix.
    pub fn fork_speculative_agent(
        &self,
        branch_id: &str,
        parent_turns: Vec<ContextTurn>,
        total_tokens: usize,
    ) -> ForkedSubagentBranch {
        let snapshot = ContextSnapshot {
            snapshot_id: format!("snap_{}", branch_id),
            base_turns: Arc::new(parent_turns),
            total_base_tokens: total_tokens,
            created_at: Utc::now(),
        };
        ForkedSubagentBranch::fork_from(branch_id, snapshot)
    }

    /// Records an agent file read to protect against background code-shifting collisions.
    pub fn record_agent_read(&self, agent_id: &str, path: impl AsRef<Path>, content: &[u8]) {
        if let Ok(mut tracker) = self.collision_tracker.lock() {
            tracker.register_read(agent_id, path, content);
        }
    }

    /// Validates an agent file write, returning any collision alerts triggered for peers.
    pub fn validate_agent_write(
        &self,
        agent_id: &str,
        path: impl AsRef<Path>,
        new_content: &[u8],
    ) -> Vec<CollisionEvent> {
        if let Ok(mut tracker) = self.collision_tracker.lock() {
            tracker.propose_write(agent_id, path, new_content)
        } else {
            Vec::new()
        }
    }

    /// Dispatches an atomic array of tool calls in a single batch turn.
    pub fn execute_tool_batch(&self, batch_id: &str, calls: &[ToolCallInvocation]) -> RpcBatchSummary {
        self.rpc_dispatcher.execute_batch(batch_id, calls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_unified_harness_integration() {
        let harness = MapaiHarness::new();
        let file = PathBuf::from("src/engine.rs");
        let initial_code = b"fn run() {}";

        // 1. Swarm lease test
        harness.record_agent_read("agent_worker_1", &file, initial_code);
        let collisions = harness.validate_agent_write("agent_worker_2", &file, b"fn run() { panic!(); }");
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].affected_agent, "agent_worker_1");

        // 2. Speculative context fork test
        let branch = harness.fork_speculative_agent("sub_01", vec![], 5000);
        assert_eq!(branch.tokens_saved_by_forking, 5000);
    }
}
