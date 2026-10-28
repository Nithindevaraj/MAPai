use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cognitive_memory::{CognitiveMemoryStore, SessionMemoryRecord};
use crate::skill_synthesizer::{SkillSynthesizer, SynthesizedSkill};

/// The primary scientific and engineering domains tracked by the MAPai Autonomous Research Radar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResearchDomain {
    FundamentalTheory,
    ModelArchitecture,
    AgentWireProtocols,
    SystemPerformance,
    SafetyVerification,
}

impl ResearchDomain {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FundamentalTheory => "Fundamental Theory & Reasoning Chains",
            Self::ModelArchitecture => "Model Architectures & Multimodal Grounding",
            Self::AgentWireProtocols => "Agent Swarms & Wire Protocols (MCP/ACP)",
            Self::SystemPerformance => "System Performance, Concurrency & Low Latency",
            Self::SafetyVerification => "Neuro-Symbolic & Formal Safety Verification",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::FundamentalTheory => "📐",
            Self::ModelArchitecture => "🧠",
            Self::AgentWireProtocols => "⚡",
            Self::SystemPerformance => "🚀",
            Self::SafetyVerification => "🛡️",
        }
    }
}

/// An atomic research discovery or technical advancement tracked in the radar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchTopic {
    pub id: String,
    pub domain: ResearchDomain,
    pub title: String,
    pub source: String,
    pub abstract_summary: String,
    pub capability_delta: String,
    pub actionable_pattern: String,
    pub recommended_skill: Option<String>,
    pub ingested_at: DateTime<Utc>,
}

/// The sovereign research repository maintaining empirical intelligence deltas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchRadar {
    pub updated_at: DateTime<Utc>,
    pub topics: Vec<ResearchTopic>,
}

impl Default for ResearchRadar {
    fn default() -> Self {
        Self {
            updated_at: Utc::now(),
            topics: Self::curated_seeds(),
        }
    }
}

impl ResearchRadar {
    /// Loads the research radar from the given JSON path, or returns default seeds if not found.
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        let p = path.as_ref();
        if p.exists() {
            if let Ok(content) = fs::read_to_string(p) {
                if let Ok(radar) = serde_json::from_str::<ResearchRadar>(&content) {
                    return radar;
                }
            }
        }
        Self::default()
    }

    /// Persists the research radar state to disk.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let p = path.as_ref();
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).context("Failed to create directory for research radar")?;
        }
        let json = serde_json::to_string_pretty(self).context("Failed to serialize research radar")?;
        fs::write(p, json).context("Failed to write research radar file")?;
        Ok(())
    }

    /// Adds or updates a research topic.
    pub fn add_topic(&mut self, topic: ResearchTopic) {
        if let Some(pos) = self.topics.iter().position(|t| t.id == topic.id) {
            self.topics[pos] = topic;
        } else {
            self.topics.push(topic);
        }
        self.updated_at = Utc::now();
    }

    /// Retrieves a research topic by id.
    pub fn get_topic(&self, id: &str) -> Option<&ResearchTopic> {
        self.topics.iter().find(|t| t.id.eq_ignore_ascii_case(id))
    }

    /// Filter topics by domain and optional substring search.
    pub fn query(&self, domain: Option<ResearchDomain>, query_str: Option<&str>) -> Vec<&ResearchTopic> {
        self.topics
            .iter()
            .filter(|t| {
                if let Some(d) = domain {
                    if t.domain != d {
                        return false;
                    }
                }
                if let Some(q) = query_str {
                    let needle = q.to_lowercase();
                    return t.title.to_lowercase().contains(&needle)
                        || t.abstract_summary.to_lowercase().contains(&needle)
                        || t.capability_delta.to_lowercase().contains(&needle)
                        || t.id.to_lowercase().contains(&needle);
                }
                true
            })
            .collect()
    }

    /// Automatically synthesizes and persists an actionable `SKILL.md` from a researched topic.
    pub fn synthesize_skill_from_topic<P: AsRef<Path>>(
        &self,
        topic_id: &str,
        skills_root: P,
    ) -> Result<PathBuf> {
        let topic = match self.get_topic(topic_id) {
            Some(t) => t,
            None => bail!("Research topic '{}' not found in radar", topic_id),
        };

        let skill_name = topic
            .recommended_skill
            .clone()
            .unwrap_or_else(|| format!("{}-pattern", topic.id));

        let instructions = format!(
            "### Background Context & Provenance\nDerived autonomously from {}\n\n### Capability Delta\n{}\n\n### Actionable Operational Pattern\n{}\n\n### Execution Protocol\n1. Ingest pattern heuristics into active subagent context.\n2. Sandbox execution in temporary scratch environment.\n3. Validate against deterministic verification gates before disk commit.",
            topic.source,
            topic.capability_delta,
            topic.actionable_pattern
        );

        let triggers = vec![
            topic.id.clone(),
            skill_name.clone(),
            format!("research-{}", topic.id),
        ];

        let verification_steps = vec![
            "Confirm pattern complies with AST truth-to-materials (0 hollow stubs)".into(),
            "Verify memory and latency constraints meet sub-10ms requirement".into(),
            "Anchor execution result with CAWO causal checkpoint".into(),
        ];

        let skill = SynthesizedSkill {
            name: skill_name,
            description: topic.title.clone(),
            triggers,
            instructions,
            verification_steps,
        };

        let synthesizer = SkillSynthesizer::new(skills_root.as_ref().to_path_buf());
        synthesizer.persist_skill(&skill)
    }

    /// Indexes all radar topics into the persistent SQLite FTS5 CognitiveMemoryStore for instant semantic retrieval.
    pub fn sync_to_cognitive_memory(&self, store: &CognitiveMemoryStore) -> Result<usize> {
        let mut count = 0;
        for topic in &self.topics {
            let record = SessionMemoryRecord {
                session_id: format!("radar-{}", topic.id),
                task_title: format!("[Research Radar] {}", topic.title),
                intent_summary: format!(
                    "Domain: {}. Source: {}. Abstract: {}",
                    topic.domain.display_name(),
                    topic.source,
                    topic.abstract_summary
                ),
                key_decisions: format!(
                    "Capability Delta: {}. Actionable Heuristic: {}",
                    topic.capability_delta, topic.actionable_pattern
                ),
                files_touched: format!(".mapai/research_radar.json; skill: {:?}", topic.recommended_skill),
                timestamp: topic.ingested_at,
            };
            store.index_session(&record)?;
            count += 1;
        }
        Ok(count)
    }

    /// Foundational curated seed topics representing high-impact breakthroughs.
    pub fn curated_seeds() -> Vec<ResearchTopic> {
        vec![
            ResearchTopic {
                id: "test-time-consensus-swarms".into(),
                domain: ResearchDomain::AgentWireProtocols,
                title: "Multi-Agent Test-Time Consensus & Epistemic Dissent".into(),
                source: "arXiv:2502.14856 / SAiA Swarm Specification".into(),
                abstract_summary: "Scaling inference-time compute through asynchronous multi-agent debate, cross-model verification, and formal vote tallying eliminates hallucinations in code generation.".into(),
                capability_delta: "Enables swarms of lightweight models to outperform monolithic frontier models by 14.8% on complex refactoring tasks.".into(),
                actionable_pattern: "Spawn 3 specialized subagents with distinct personas (Architect, Skeptic, Implementer). Aggregate AST diffs through Two-Phase Commit before merging.".into(),
                recommended_skill: Some("swarm-consensus-raft".into()),
                ingested_at: Utc::now(),
            },
            ResearchTopic {
                id: "context-kv-compression-graphs".into(),
                domain: ResearchDomain::SystemPerformance,
                title: "Zero-Copy Context Compaction & Structural Knowledge Graphs".into(),
                source: "DeepMind Systems Whitepaper / SAiA Cognitive Store".into(),
                abstract_summary: "Replacing sprawling chat transcripts with persistent SQLite FTS5 indexes and entity knowledge graphs cuts prompt token waste by over 70%.".into(),
                capability_delta: "Enables multi-week coding workflows without catastrophic forgetting or quadratic token degradation.".into(),
                actionable_pattern: "Compact completed task trajectories into intent summaries and touched AST nodes. Inject only top-k relevant fragments into prompt wire.".into(),
                recommended_skill: Some("cognitive-graph-compaction".into()),
                ingested_at: Utc::now(),
            },
            ResearchTopic {
                id: "bidirectional-audio-vad-streaming".into(),
                domain: ResearchDomain::ModelArchitecture,
                title: "Sub-100ms Multimodal Streaming with Client-Side Voice Activity Detection".into(),
                source: "Google Gemini 3.8 Live API Technical Spec".into(),
                abstract_summary: "Real-time bidirectional WebSocket transport pairing raw PCM streaming with local silero-VAD suppresses latency spikes and prevents accidental model interruptions.".into(),
                capability_delta: "True human-like conversational pair programming with real-time screen sensing.".into(),
                actionable_pattern: "Buffer 16kHz audio chunks with local energy/RMS detection. Send interrupt tokens immediately when user voice onset crosses -24dB threshold.".into(),
                recommended_skill: Some("live-audio-vad-bridge".into()),
                ingested_at: Utc::now(),
            },
            ResearchTopic {
                id: "neuro-symbolic-ast-verification".into(),
                domain: ResearchDomain::SafetyVerification,
                title: "Deterministic AST Semantic Auditing & Hollow-Stub Elimination".into(),
                source: "Formal Methods in SE 2026 / SAiA TruthScanner".into(),
                abstract_summary: "Enforcing static AST parsers and regex heuristic gates across all generated files prevents AI models from producing fake mock methods or unverified stubs.".into(),
                capability_delta: "Zero tolerance for unexecutable code commits. Every delivered method is verified to possess physical truth-to-materials.".into(),
                actionable_pattern: "Scan function bodies for TODO, unimplemented stubs, pass, or empty blocks. Reject git commit and block pipeline advance if hollow completion rate > 0.00.".into(),
                recommended_skill: Some("truth-ast-enforcer".into()),
                ingested_at: Utc::now(),
            },
            ResearchTopic {
                id: "two-phase-commit-virtual-staging".into(),
                domain: ResearchDomain::AgentWireProtocols,
                title: "Two-Phase Commit Virtual File Staging for Concurrent Agents".into(),
                source: "SAiA Distributed Concurrency Protocol v1.0.23".into(),
                abstract_summary: "Preventing write collisions among concurrent coding agents by maintaining in-memory virtual staging buffers until all validation tests pass.".into(),
                capability_delta: "Allows 5+ agents to simultaneously refactor overlapping files without race conditions or partial file corruption.".into(),
                actionable_pattern: "Phase 1 (Prepare): Agents stage changes into private memory buffers. Phase 2 (Commit): Acquire file lock, verify tree hash, write atomically.".into(),
                recommended_skill: Some("swarm-2pc-virtual-stage".into()),
                ingested_at: Utc::now(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_research_radar_default_seeds() {
        let radar = ResearchRadar::default();
        assert!(radar.topics.len() >= 5);
        let topic = radar.get_topic("test-time-consensus-swarms");
        assert!(topic.is_some());
        assert_eq!(topic.unwrap().domain, ResearchDomain::AgentWireProtocols);
    }

    #[test]
    fn test_research_radar_query() {
        let radar = ResearchRadar::default();
        let perf_topics = radar.query(Some(ResearchDomain::SystemPerformance), None);
        assert!(!perf_topics.is_empty());

        let vad_search = radar.query(None, Some("VAD"));
        assert_eq!(vad_search.len(), 1);
        assert_eq!(vad_search[0].id, "bidirectional-audio-vad-streaming");
    }

    #[test]
    fn test_research_radar_skill_synthesis() {
        let radar = ResearchRadar::default();
        let temp_dir = std::env::temp_dir().join(format!("mapai_radar_test_{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)));
        let skill_file = radar
            .synthesize_skill_from_topic("two-phase-commit-virtual-staging", &temp_dir)
            .expect("Failed to synthesize skill");

        assert!(skill_file.exists());
        let content = fs::read_to_string(skill_file).unwrap();
        assert!(content.contains("swarm-2pc-virtual-stage"));
        assert!(content.contains("Two-Phase Commit"));
    }

    #[test]
    fn test_research_radar_cognitive_memory_sync() {
        let radar = ResearchRadar::default();
        let store = CognitiveMemoryStore::open_in_memory().expect("Failed to open cognitive memory");
        let synced = radar.sync_to_cognitive_memory(&store).expect("Sync failed");
        assert_eq!(synced, radar.topics.len());

        let results = store.search("consensus", 5).expect("Search failed");
        assert!(!results.is_empty());
        assert!(results[0].task_title.contains("Consensus"));
    }
}
