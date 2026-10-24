use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// An anonymized, crystallized technical lesson (Antibody) capturing empirical engineering solutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingHeuristic {
    pub id: String,
    pub title: String,
    pub discovered_at: DateTime<Utc>,
    
    /// The failure symptom or friction encountered
    pub symptom: String,
    
    /// The underlying physical or systemic root cause
    pub root_cause: String,
    
    /// The verified rule/invariant to prevent recurrence
    pub antibody_rule: String,
    
    /// Domain tags for indexing and retrieval
    pub tags: Vec<String>,
    
    /// Empirical confidence score
    pub validation_count: u32,
}
