use serde::{Deserialize, Serialize};

/// Independent policy service enforcing permission boundaries outside model influence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPolicy {
    pub allowed_paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
    pub network_egress_allowed: bool,
    pub max_execution_time_ms: u64,
    pub privilege_level: ExecutionPrivilege,
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
            max_execution_time_ms: 10_000,
            privilege_level: ExecutionPrivilege::WorkspaceScopedWrite,
        }
    }
}
