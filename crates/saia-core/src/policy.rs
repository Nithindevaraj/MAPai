use serde::{Deserialize, Serialize};

/// Independent policy service enforcing permission boundaries outside model influence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPolicy {
    pub allowed_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub network_egress_allowed: bool,
    /// Always active online scouting for live dependency, API, and CVE verification
    pub online_scouting_enabled: bool,
    pub live_registry_verification: bool,
    pub cve_advisory_verification: bool,
    pub max_execution_time_ms: u64,
    pub privilege_level: ExecutionPrivilege,
}

/// Policy governing how human reflections, philosophies, and real-life lessons are interpreted.
/// Enforces that MAPai distills underlying teleological intent and invariants
/// rather than literally broadcasting or echoing personal reflections into public deliverables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentDistillationPolicy {
    /// Never copy literal human thoughts or personal reflections into public deliverables
    pub prevent_literal_echoing: bool,
    /// Distill human real-life lessons into actionable technical takeaways and constraints
    pub extract_underlying_invariants: bool,
    /// Keep raw human reflection in sovereign private context
    pub protect_cognitive_preimage: bool,
}

impl Default for IntentDistillationPolicy {
    fn default() -> Self {
        Self {
            prevent_literal_echoing: true,
            extract_underlying_invariants: true,
            protect_cognitive_preimage: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionPrivilege {
    ReadOnly,
    WorkspaceScopedWrite,
    SystemAdminGated,
}

impl Default for PermissionPolicy {
    fn default() -> Self {
        Self {
            allowed_paths: vec![".".to_string()],
            forbidden_paths: vec![".git".to_string(), ".env".to_string(), ".secrets".to_string()],
            network_egress_allowed: false,
            online_scouting_enabled: true,
            live_registry_verification: true,
            cve_advisory_verification: true,
            max_execution_time_ms: 10_000,
            privilege_level: ExecutionPrivilege::WorkspaceScopedWrite,
        }
    }
}
