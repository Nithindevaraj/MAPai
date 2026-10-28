use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskComplexity {
    DeterministicCheck,  // Linting, stub scanning, format checks -> Run locally (0 tokens)
    LowReasoning,        // Boilerplate, simple transforms -> Small Local SLM (8B)
    HighArchitecture,    // System architecture, conflict resolution -> Frontier Reasoning Model
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelTier {
    NativeBinaryLocal,
    LocalSmallLanguageModel { endpoint: String, model_tag: String },
    FrontierCloudAPI { provider: String, model_name: String },
}

pub struct ModelRouter;

impl ModelRouter {
    /// Determines the optimal, lowest-cost execution tier for a given task
    pub fn route(complexity: TaskComplexity) -> ModelTier {
        match complexity {
            TaskComplexity::DeterministicCheck => ModelTier::NativeBinaryLocal,
            TaskComplexity::LowReasoning => ModelTier::LocalSmallLanguageModel {
                endpoint: "http://localhost:11434".to_string(),
                model_tag: "qwen2.5-coder:7b".to_string(),
            },
            TaskComplexity::HighArchitecture => ModelTier::FrontierCloudAPI {
                provider: "open-router-or-direct".to_string(),
                model_name: "claude-3-7-sonnet".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_router_tier_selection() {
        assert_eq!(ModelRouter::route(TaskComplexity::DeterministicCheck), ModelTier::NativeBinaryLocal);
        match ModelRouter::route(TaskComplexity::LowReasoning) {
            ModelTier::LocalSmallLanguageModel { model_tag, .. } => assert_eq!(model_tag, "qwen2.5-coder:7b"),
            _ => panic!("Expected LocalSmallLanguageModel"),
        }
    }
}
