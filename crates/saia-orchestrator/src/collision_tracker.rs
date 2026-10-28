use std::collections::HashMap;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Represents a lease held by an agent that has inspected or read a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileLease {
    pub agent_id: String,
    pub file_path: PathBuf,
    pub content_hash: String,
    pub leased_at: DateTime<Utc>,
}

/// A collision notification generated when another agent mutates a file
/// that is actively leased by a peer agent ("code shifting under its feet").
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CollisionEvent {
    pub affected_agent: String,
    pub mutating_agent: String,
    pub file_path: PathBuf,
    pub original_hash: String,
    pub new_hash: String,
    pub diff_summary: String,
    pub timestamp: DateTime<Utc>,
}

/// In-memory Read-Set Collision Tracker for multi-agent swarms.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReadSetCollisionTracker {
    /// Mapping of normalized file path to all active agent leases.
    leases: HashMap<PathBuf, Vec<FileLease>>,
    /// Active collision notifications pending delivery to agents.
    pending_events: HashMap<String, Vec<CollisionEvent>>,
}

impl ReadSetCollisionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Computes deterministic SHA-256 hash of file content.
    pub fn hash_content(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Registers that an agent has read/inspected a specific file.
    pub fn register_read(&mut self, agent_id: &str, path: impl AsRef<Path>, content: &[u8]) -> FileLease {
        let norm_path = path.as_ref().to_path_buf();
        let hash = Self::hash_content(content);
        
        let lease = FileLease {
            agent_id: agent_id.to_string(),
            file_path: norm_path.clone(),
            content_hash: hash,
            leased_at: Utc::now(),
        };

        let file_leases = self.leases.entry(norm_path).or_default();
        // Update or replace existing lease for this agent on this file
        file_leases.retain(|l| l.agent_id != agent_id);
        file_leases.push(lease.clone());

        lease
    }

    /// Validates a proposed write by an agent. If any other agent holds a lease on the file,
    /// a collision event is generated and queued for the affected agents.
    pub fn propose_write(
        &mut self,
        mutating_agent: &str,
        path: impl AsRef<Path>,
        new_content: &[u8],
    ) -> Vec<CollisionEvent> {
        let norm_path = path.as_ref().to_path_buf();
        let new_hash = Self::hash_content(new_content);
        let mut triggered_events = Vec::new();

        if let Some(file_leases) = self.leases.get_mut(&norm_path) {
            for lease in file_leases.iter() {
                if lease.agent_id != mutating_agent && lease.content_hash != new_hash {
                    let event = CollisionEvent {
                        affected_agent: lease.agent_id.clone(),
                        mutating_agent: mutating_agent.to_string(),
                        file_path: norm_path.clone(),
                        original_hash: lease.content_hash.clone(),
                        new_hash: new_hash.clone(),
                        diff_summary: format!(
                            "File '{:?}' mutated by agent '{}' while leased by '{}'",
                            norm_path, mutating_agent, lease.agent_id
                        ),
                        timestamp: Utc::now(),
                    };

                    self.pending_events
                        .entry(lease.agent_id.clone())
                        .or_default()
                        .push(event.clone());

                    triggered_events.push(event);
                }
            }

            // Revoke peer leases with outdated content (lease broken by collision)
            file_leases.retain(|l| l.content_hash == new_hash);
            // The mutating agent now holds the authoritative lease with the new hash
            file_leases.retain(|l| l.agent_id != mutating_agent);
            file_leases.push(FileLease {
                agent_id: mutating_agent.to_string(),
                file_path: norm_path,
                content_hash: new_hash,
                leased_at: Utc::now(),
            });
        }

        triggered_events
    }

    /// Drains and returns all pending collision notifications for a specific agent.
    pub fn drain_events_for_agent(&mut self, agent_id: &str) -> Vec<CollisionEvent> {
        self.pending_events.remove(agent_id).unwrap_or_default()
    }

    /// Releases all leases held by an agent when its task completes.
    pub fn release_agent(&mut self, agent_id: &str) {
        for leases in self.leases.values_mut() {
            leases.retain(|l| l.agent_id != agent_id);
        }
        self.pending_events.remove(agent_id);
    }

    /// Returns count of active leases across all files.
    pub fn active_lease_count(&self) -> usize {
        self.leases.values().map(|v| v.len()).sum()
    }

    /// Returns a reference to all active leases mapped by file path.
    pub fn all_leases(&self) -> &HashMap<PathBuf, Vec<FileLease>> {
        &self.leases
    }

    /// Returns a reference to all active collision notifications pending delivery.
    pub fn all_pending_events(&self) -> &HashMap<String, Vec<CollisionEvent>> {
        &self.pending_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_detection_between_two_agents() {
        let mut tracker = ReadSetCollisionTracker::new();
        let file_path = PathBuf::from("src/auth.rs");
        let initial_code = b"pub fn authenticate() -> bool { false }";

        // Agent B reads the file
        tracker.register_read("agent_b", &file_path, initial_code);
        assert_eq!(tracker.active_lease_count(), 1);

        // Agent A edits the file
        let new_code = b"pub fn authenticate() -> bool { true }";
        let collisions = tracker.propose_write("agent_a", &file_path, new_code);

        // A collision must be triggered for Agent B
        assert_eq!(collisions.len(), 1);
        assert_eq!(collisions[0].affected_agent, "agent_b");
        assert_eq!(collisions[0].mutating_agent, "agent_a");

        // Agent B drains its mailbox
        let pending = tracker.drain_events_for_agent("agent_b");
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].original_hash, ReadSetCollisionTracker::hash_content(initial_code));
        assert_eq!(pending[0].new_hash, ReadSetCollisionTracker::hash_content(new_code));

        // Subsequent drain is empty
        assert!(tracker.drain_events_for_agent("agent_b").is_empty());
    }

    #[test]
    fn test_no_collision_when_mutating_agent_writes_own_lease() {
        let mut tracker = ReadSetCollisionTracker::new();
        let file_path = PathBuf::from("src/main.rs");
        let initial_code = b"fn main() {}";

        tracker.register_read("agent_a", &file_path, initial_code);
        let collisions = tracker.propose_write("agent_a", &file_path, b"fn main() { println!(); }");

        assert!(collisions.is_empty(), "Writing to an agent's own lease should not self-collide");
    }

    #[test]
    fn test_five_agent_swarm_high_contention_collision() {
        let mut tracker = ReadSetCollisionTracker::new();
        let target_file = PathBuf::from("crates/saia-core/src/state.rs");
        let base_content = b"pub struct State { pub count: u32 }";

        // Agents 1 through 4 all lease the file
        let agent_ids = ["agent_alpha", "agent_beta", "agent_gamma", "agent_delta"];
        for id in &agent_ids {
            tracker.register_read(id, &target_file, base_content);
        }
        assert_eq!(tracker.active_lease_count(), 4);

        // Agent 5 (Omega) mutates the file
        let mutated_content = b"pub struct State { pub count: u64, pub active: bool }";
        let collisions = tracker.propose_write("agent_omega", &target_file, mutated_content);

        // All 4 peer agents holding active leases MUST receive collision notifications
        assert_eq!(collisions.len(), 4);
        for id in &agent_ids {
            assert!(collisions.iter().any(|c| c.affected_agent == *id));
            let mailbox = tracker.drain_events_for_agent(id);
            assert_eq!(mailbox.len(), 1);
            assert_eq!(mailbox[0].mutating_agent, "agent_omega");
        }

        // Only agent_omega now holds the authoritative lease
        assert_eq!(tracker.active_lease_count(), 1);
        tracker.release_agent("agent_omega");
        assert_eq!(tracker.active_lease_count(), 0);
    }
}
