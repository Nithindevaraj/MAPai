use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub const CURRENT_SCHEMA_VERSION: &str = "saia/v1";
pub const CURRENT_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionStatus {
    UpToDate,
    OutdatedCapsule {
        capsule_schema: String,
        current_schema: String,
        capsule_cli: Option<String>,
        current_cli: String,
    },
    NewerCapsule {
        capsule_schema: String,
        current_schema: String,
    },
}

/// Represents the foundational specification and working memory of a project under the SAiA protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCapsule {
    pub schema_version: String,
    #[serde(default)]
    pub cli_version: Option<String>,
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

/// The canonical Professional Jobs-to-be-Done (JTBD) classification for MAPai systems engineering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessionalJobClass {
    /// 0-to-1 architecture, requirements scoping, and end-to-end multi-modal implementation.
    GreenfieldSynthesis,
    /// Legacy migrations, framework upgrades, stack porting, and technical debt reduction.
    BrownfieldModernization,
    /// Deterministic AST auditing (TruthScanner), CVE/security scans, and WCAG contrast gates.
    IndependentVerification,
    /// Closed-loop OS desktop control, COM APIs, headless browser extraction, and RPA.
    HeterogeneousAutomation,
    /// Ingestion and parsing of dense technical specs (PDFs), competitive auditing (OmniScout), and ADRs.
    CognitiveDistillation,
}

impl ProfessionalJobClass {
    pub fn all() -> &'static [ProfessionalJobClass] {
        &[
            ProfessionalJobClass::GreenfieldSynthesis,
            ProfessionalJobClass::BrownfieldModernization,
            ProfessionalJobClass::IndependentVerification,
            ProfessionalJobClass::HeterogeneousAutomation,
            ProfessionalJobClass::CognitiveDistillation,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GreenfieldSynthesis => "greenfield-synthesis",
            Self::BrownfieldModernization => "brownfield-modernization",
            Self::IndependentVerification => "independent-verification",
            Self::HeterogeneousAutomation => "heterogeneous-automation",
            Self::CognitiveDistillation => "cognitive-distillation",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::GreenfieldSynthesis => "Greenfield Synthesis (0-to-1 Systems Architecture)",
            Self::BrownfieldModernization => "Brownfield Modernization (Systemic Evolution & Refactoring)",
            Self::IndependentVerification => "Independent Verification & Assurance (IV&V & Compliance)",
            Self::HeterogeneousAutomation => "Heterogeneous Tool Orchestration & Robotic Automation",
            Self::CognitiveDistillation => "Cognitive Distillation & Knowledge Engineering",
        }
    }

    pub fn from_identifier(id: &str) -> Option<Self> {
        match id.trim().to_lowercase().replace('_', "-").as_str() {
            "greenfield" | "greenfield-synthesis" | "0-to-1" => Some(Self::GreenfieldSynthesis),
            "brownfield" | "brownfield-modernization" | "refactor" | "migration" => Some(Self::BrownfieldModernization),
            "verification" | "independent-verification" | "iv-v" | "audit" => Some(Self::IndependentVerification),
            "automation" | "heterogeneous-automation" | "rpa" | "orchestration" => Some(Self::HeterogeneousAutomation),
            "distillation" | "cognitive-distillation" | "research" => Some(Self::CognitiveDistillation),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobIntent {
    pub problem_statement: String,
    pub target_domain: String,
    #[serde(default)]
    pub job_class: Option<ProfessionalJobClass>,
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
            schema_version: CURRENT_SCHEMA_VERSION.to_string(),
            cli_version: Some(CURRENT_CLI_VERSION.to_string()),
            project_name: "unnamed-project".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            job_1_intent: JobIntent {
                problem_statement: String::new(),
                target_domain: "general-software".to_string(),
                job_class: Some(ProfessionalJobClass::GreenfieldSynthesis),
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

impl ProjectCapsule {
    /// Evaluates compatibility between the project capsule and the current MAPai binary version
    pub fn check_version_compatibility(&self) -> VersionStatus {
        let is_schema_match = self.schema_version == CURRENT_SCHEMA_VERSION;
        let is_cli_match = self.cli_version.as_deref() == Some(CURRENT_CLI_VERSION);

        if is_schema_match && is_cli_match {
            VersionStatus::UpToDate
        } else if self.schema_version.as_str() < CURRENT_SCHEMA_VERSION 
            || (is_schema_match && !is_cli_match) 
        {
            VersionStatus::OutdatedCapsule {
                capsule_schema: self.schema_version.clone(),
                current_schema: CURRENT_SCHEMA_VERSION.to_string(),
                capsule_cli: self.cli_version.clone(),
                current_cli: CURRENT_CLI_VERSION.to_string(),
            }
        } else if self.schema_version.as_str() > CURRENT_SCHEMA_VERSION {
            VersionStatus::NewerCapsule {
                capsule_schema: self.schema_version.clone(),
                current_schema: CURRENT_SCHEMA_VERSION.to_string(),
            }
        } else {
            VersionStatus::UpToDate
        }
    }

    /// Non-destructively migrates capsule fields and schema version to the latest standard
    pub fn migrate_to_latest(&mut self) -> Vec<String> {
        let mut migrations = Vec::new();

        if self.schema_version != CURRENT_SCHEMA_VERSION {
            migrations.push(format!("Upgraded schema version: {} -> {}", self.schema_version, CURRENT_SCHEMA_VERSION));
            self.schema_version = CURRENT_SCHEMA_VERSION.to_string();
        }

        if self.cli_version.as_deref() != Some(CURRENT_CLI_VERSION) {
            let prev = self.cli_version.clone().unwrap_or_else(|| "unversioned".into());
            migrations.push(format!("Synchronized CLI binary version: {} -> {}", prev, CURRENT_CLI_VERSION));
            self.cli_version = Some(CURRENT_CLI_VERSION.to_string());
        }

        // Add any newly introduced non-negotiable forbidden patterns if missing
        let default_forbidden = ["no-todo-stubs", "no-unhandled-unwrap"];
        for pattern in &default_forbidden {
            if !self.job_2_constraints.forbidden_patterns.iter().any(|p| p == *pattern) {
                self.job_2_constraints.forbidden_patterns.push(pattern.to_string());
                migrations.push(format!("Enforced default constraint: {}", pattern));
            }
        }

        if !migrations.is_empty() {
            self.updated_at = Utc::now();
        }

        migrations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compatibility_detection() {
        let mut capsule = ProjectCapsule::default();
        assert_eq!(capsule.check_version_compatibility(), VersionStatus::UpToDate);

        // Outdated schema
        capsule.schema_version = "saia/v0".to_string();
        assert!(matches!(capsule.check_version_compatibility(), VersionStatus::OutdatedCapsule { .. }));

        // Outdated CLI version
        capsule.schema_version = CURRENT_SCHEMA_VERSION.to_string();
        let capsule = ProjectCapsule {
            schema_version: "saia/v0".to_string(),
            cli_version: Some("0.0.1".to_string()),
            ..Default::default()
        };
        assert!(matches!(capsule.check_version_compatibility(), VersionStatus::OutdatedCapsule { .. }));
    }

    #[test]
    fn test_migration_to_latest() {
        let mut capsule = ProjectCapsule {
            schema_version: "saia/v0".to_string(),
            cli_version: None,
            ..Default::default()
        };
        capsule.job_2_constraints.forbidden_patterns.clear();

        let migrations = capsule.migrate_to_latest();
        assert!(!migrations.is_empty());
        assert_eq!(capsule.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(capsule.cli_version, Some(CURRENT_CLI_VERSION.to_string()));
        assert!(capsule.job_2_constraints.forbidden_patterns.contains(&"no-todo-stubs".to_string()));
        assert_eq!(capsule.check_version_compatibility(), VersionStatus::UpToDate);
    }
}
