use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents the foundational specification and working memory of a project under the SAiA protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCapsule {
    pub schema_version: String,
    pub project_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    /// Job 1: Human intent and functional requirements
    pub job_1_intent: JobIntent,
    
    /// Job 2: Non-negotiable physical and architectural constraints
    pub job_2_constraints: MaterialConstraints,

    /// Active state in the formal pipeline
    pub current_stage: PipelineStage,
    pub active_sprint: SprintObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobIntent {
    pub problem_statement: String,
    pub target_domain: String,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialConstraints {
    pub primary_language: String,
    pub target_platforms: Vec<String>,
    pub performance_envelope: Vec<String>,
    pub forbidden_patterns: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStage {
    Stage1ProblemIntent,
    Stage2ArchitectureConstraints,
    Stage3MaterialSelection,
    Stage4Implementation,
    Stage5EmpiricalRealityCheck,
    Stage6HardeningDiagnostics,
    Stage7SovereignDelivery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintObjective {
    pub title: String,
    pub checklist: Vec<SprintTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintTask {
    pub id: String,
    pub description: String,
    pub completed: bool,
}

impl Default for ProjectCapsule {
    fn default() -> Self {
        Self {
            schema_version: "saia/v1".to_string(),
            project_name: "unnamed-project".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            job_1_intent: JobIntent {
                problem_statement: String::new(),
                target_domain: "general-software".to_string(),
                success_criteria: Vec::new(),
            },
            job_2_constraints: MaterialConstraints {
                primary_language: "rust".to_string(),
                target_platforms: vec!["native".to_string()],
                performance_envelope: vec!["sub-millisecond verification".to_string()],
                forbidden_patterns: vec!["no-todo-stubs".to_string(), "no-unhandled-unwrap".to_string()],
            },
            current_stage: PipelineStage::Stage1ProblemIntent,
            active_sprint: SprintObjective {
                title: "Initial Scaffold".to_string(),
                checklist: Vec::new(),
            },
        }
    }
}
