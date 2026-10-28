use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for autonomous overnight execution sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvernightConfig {
    pub run_id: String,
    pub mission: String,
    pub target_wake_at: DateTime<Utc>,
    pub battery_threshold_percent: u8,
    pub max_iterations: usize,
}

impl Default for OvernightConfig {
    fn default() -> Self {
        Self {
            run_id: format!("overnight_{}", Utc::now().timestamp()),
            mission: "Autonomous verification and test hardening".into(),
            target_wake_at: Utc::now() + Duration::hours(8),
            battery_threshold_percent: 20,
            max_iterations: 100,
        }
    }
}

/// Instant snapshot of device physical resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub battery_percent: Option<u8>,
    pub is_charging: bool,
    pub memory_used_mb: u64,
    pub timestamp: DateTime<Utc>,
}

/// An executive morning handoff brief delivered when the user wakes up.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningHandoffReport {
    pub run_id: String,
    pub mission: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub total_iterations: usize,
    pub tests_passed: bool,
    pub commits_staged: usize,
    pub battery_at_completion: Option<u8>,
    pub executive_summary: String,
    pub staged_files: Vec<String>,
    pub blockers_and_questions: Vec<String>,
}

/// Manages long-running overnight execution with battery safety and morning reporting.
pub struct OvernightCoordinator {
    config: OvernightConfig,
    started_at: DateTime<Utc>,
    iterations_run: usize,
}

impl OvernightCoordinator {
    pub fn new(config: OvernightConfig) -> Self {
        Self {
            config,
            started_at: Utc::now(),
            iterations_run: 0,
        }
    }

    /// Evaluates whether the system is physically safe to continue overnight execution.
    pub fn is_safe_to_continue(&self, snapshot: &ResourceSnapshot) -> (bool, Option<String>) {
        if let Some(batt) = snapshot.battery_percent {
            if !snapshot.is_charging && batt <= self.config.battery_threshold_percent {
                return (
                    false,
                    Some(format!(
                        "Battery level ({}%) is at or below safety threshold ({}%) and not charging. Parking safely.",
                        batt, self.config.battery_threshold_percent
                    )),
                );
            }
        }

        if Utc::now() >= self.config.target_wake_at {
            return (
                false,
                Some("Target wake time reached. Finalizing morning handoff report.".into()),
            );
        }

        if self.iterations_run >= self.config.max_iterations {
            return (
                false,
                Some("Maximum configured overnight iterations reached.".into()),
            );
        }

        (true, None)
    }

    /// Records completion of an autonomous cycle.
    pub fn record_iteration(&mut self) {
        self.iterations_run += 1;
    }

    /// Generates the structured morning handoff report for the user.
    pub fn generate_morning_report(
        &self,
        tests_passed: bool,
        commits_staged: usize,
        staged_files: Vec<String>,
        summary: &str,
        blockers: Vec<String>,
        snapshot: &ResourceSnapshot,
    ) -> MorningHandoffReport {
        MorningHandoffReport {
            run_id: self.config.run_id.clone(),
            mission: self.config.mission.clone(),
            started_at: self.started_at,
            completed_at: Utc::now(),
            total_iterations: self.iterations_run,
            tests_passed,
            commits_staged,
            battery_at_completion: snapshot.battery_percent,
            executive_summary: summary.to_string(),
            staged_files,
            blockers_and_questions: blockers,
        }
    }

    /// Renders the morning brief into clean markdown.
    pub fn render_markdown(report: &MorningHandoffReport) -> String {
        let mut out = String::new();
        out.push_str("# ☀️ MAPai Morning Handoff Report\n\n");
        out.push_str(&format!("**Mission**: {}\n", report.mission));
        out.push_str(&format!("**Run ID**: `{}`\n", report.run_id));
        out.push_str(&format!(
            "**Duration**: {} minutes ({} iterations)\n",
            (report.completed_at - report.started_at).num_minutes(),
            report.total_iterations
        ));
        out.push_str(&format!(
            "**Verification Status**: {}\n",
            if report.tests_passed { "✅ All Tests Passing" } else { "⚠️ Test Failures Detected" }
        ));
        if let Some(batt) = report.battery_at_completion {
            out.push_str(&format!("**Device Battery**: {}%\n", batt));
        }
        out.push_str(&format!("**Commits Staged (2PC)**: {}\n\n", report.commits_staged));

        out.push_str("## Executive Summary\n");
        out.push_str(&format!("{}\n\n", report.executive_summary));

        if !report.staged_files.is_empty() {
            out.push_str("## Staged Files (Ready for Review)\n");
            for f in &report.staged_files {
                out.push_str(&format!("- `{}`\n", f));
            }
            out.push('\n');
        }

        if !report.blockers_and_questions.is_empty() {
            out.push_str("## Questions Requiring Human Intent Alignment\n");
            for q in &report.blockers_and_questions {
                out.push_str(&format!("- ❓ {}\n", q));
            }
            out.push('\n');
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_throttle_safety() {
        let config = OvernightConfig {
            battery_threshold_percent: 25,
            ..Default::default()
        };
        let coord = OvernightCoordinator::new(config);

        // Discharging with 20% battery -> MUST pause
        let low_battery_snap = ResourceSnapshot {
            battery_percent: Some(20),
            is_charging: false,
            memory_used_mb: 512,
            timestamp: Utc::now(),
        };
        let (safe, reason) = coord.is_safe_to_continue(&low_battery_snap);
        assert!(!safe);
        assert!(reason.unwrap().contains("Battery level (20%)"));

        // Charging with 20% battery -> Safe to proceed
        let charging_snap = ResourceSnapshot {
            battery_percent: Some(20),
            is_charging: true,
            memory_used_mb: 512,
            timestamp: Utc::now(),
        };
        let (safe2, _) = coord.is_safe_to_continue(&charging_snap);
        assert!(safe2);
    }

    #[test]
    fn test_morning_report_rendering() {
        let mut coord = OvernightCoordinator::new(OvernightConfig::default());
        coord.record_iteration();
        coord.record_iteration();

        let snap = ResourceSnapshot {
            battery_percent: Some(85),
            is_charging: true,
            memory_used_mb: 450,
            timestamp: Utc::now(),
        };

        let report = coord.generate_morning_report(
            true,
            2,
            vec!["crates/saia-verify/src/blast_radius.rs".into()],
            "Completed 2 autonomous refinement passes with 100% test coverage.",
            vec!["Approve Phase 2 commit of blast radius guard?".into()],
            &snap,
        );

        let md = OvernightCoordinator::render_markdown(&report);
        assert!(md.contains("# ☀️ MAPai Morning Handoff Report"));
        assert!(md.contains("✅ All Tests Passing"));
        assert!(md.contains("crates/saia-verify/src/blast_radius.rs"));
        assert!(md.contains("Approve Phase 2 commit"));
    }
}
