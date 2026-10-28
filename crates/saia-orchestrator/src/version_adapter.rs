use saia_core::capsule::{ProjectCapsule, VersionStatus, CURRENT_CLI_VERSION, CURRENT_SCHEMA_VERSION};
use saia_core::cawo::{CawoCheckpoint, ContinuityJournal, GateStatus};
use saia_core::StatePaths;
use saia_verify::TruthScanner;
use chrono::Utc;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct VersionUpdateReport {
    pub status: VersionStatus,
    pub migration_required: bool,
    pub changes_summary: Vec<String>,
    pub prescribed_actions: Vec<String>,
}

pub struct VersionAdaptiveEngine;

impl VersionAdaptiveEngine {
    /// Inspects the workspace capsule for version updates or schema drift
    pub fn inspect_project<P: AsRef<Path>>(workspace_root: P) -> anyhow::Result<VersionUpdateReport> {
        let paths = StatePaths::resolve(&workspace_root);
        let capsule_path = &paths.capsule_file;
        if !capsule_path.exists() {
            return Ok(VersionUpdateReport {
                status: VersionStatus::OutdatedCapsule {
                    capsule_schema: "missing".into(),
                    current_schema: CURRENT_SCHEMA_VERSION.into(),
                    capsule_cli: None,
                    current_cli: CURRENT_CLI_VERSION.into(),
                },
                migration_required: true,
                changes_summary: vec!["Workspace lacks a capsule.toml specification.".into()],
                prescribed_actions: vec!["Run `mapai init` or `mapai migrate` to initialize standard capsule.".into()],
            });
        }

        let content = fs::read_to_string(capsule_path)?;
        let capsule: ProjectCapsule = toml::from_str(&content)?;
        let status = capsule.check_version_compatibility();

        let mut changes_summary = Vec::new();
        let mut prescribed_actions = Vec::new();
        let mut migration_required = false;

        match &status {
            VersionStatus::UpToDate => {
                changes_summary.push(format!("Capsule and binary are fully synchronized at schema {} (CLI v{}).", CURRENT_SCHEMA_VERSION, CURRENT_CLI_VERSION));
                prescribed_actions.push("Workspace is up-to-date. Ready for standard pipeline progression.".into());
            }
            VersionStatus::OutdatedCapsule { capsule_schema, current_schema, capsule_cli, current_cli } => {
                migration_required = true;
                if capsule_schema != current_schema {
                    changes_summary.push(format!("Schema update detected: workspace is using '{}', latest protocol is '{}'.", capsule_schema, current_schema));
                }
                if capsule_cli.as_deref() != Some(current_cli) {
                    changes_summary.push(format!(
                        "CLI update detected: workspace last managed by v{}, active binary is v{}.",
                        capsule_cli.as_deref().unwrap_or("unversioned"),
                        current_cli
                    ));
                }
                prescribed_actions.push("Run `mapai migrate` to non-destructively adapt configuration schema and record a CAWO upgrade checkpoint.".into());
                prescribed_actions.push("Run `mapai check` to verify existing deliverables against latest Truth-to-Materials rules.".into());
            }
            VersionStatus::NewerCapsule { capsule_schema, current_schema } => {
                changes_summary.push(format!("Workspace schema '{}' is newer than this MAPai binary's supported schema '{}'.", capsule_schema, current_schema));
                prescribed_actions.push("Please update your MAPai binary to the latest release (`cargo install --path crates/saia-cli`).".into());
            }
        }

        Ok(VersionUpdateReport {
            status,
            migration_required,
            changes_summary,
            prescribed_actions,
        })
    }

    /// Executes an automated, non-destructive migration on the workspace
    pub fn execute_migration<P: AsRef<Path>>(workspace_root: P) -> anyhow::Result<Vec<String>> {
        let root = workspace_root.as_ref();
        let paths = StatePaths::resolve(root);
        paths.ensure_dir_exists()?;

        let capsule_path = &paths.capsule_file;
        let mut capsule = if capsule_path.exists() {
            let content = fs::read_to_string(capsule_path)?;
            toml::from_str::<ProjectCapsule>(&content)?
        } else {
            ProjectCapsule::default()
        };

        let migrations = capsule.migrate_to_latest();
        let updated_toml = toml::to_string_pretty(&capsule)?;
        fs::write(capsule_path, updated_toml)?;

        // Record a formal migration checkpoint into the CAWO Continuity Journal
        let journal_path = &paths.journal_file;
        let mut journal = ContinuityJournal::load_from_file(journal_path)?;

        let scanner = TruthScanner::new();
        let report = scanner.scan_directory(root)?;
        let reality_status = if report.is_clean {
            GateStatus::Passed
        } else {
            GateStatus::Rejected {
                reasons: report.violations.into_iter().map(|v| format!("{}:{}: {}", v.file_path, v.line_number, v.pattern_type)).collect(),
            }
        };

        let ast_hash = ContinuityJournal::compute_workspace_tree_hash(root)?;
        let parent_step_id = journal.checkpoints.last().map(|c| c.step_id);
        let next_step_id = parent_step_id.unwrap_or(0) + 1;

        let migration_summary = if migrations.is_empty() {
            "Verified version alignment".to_string()
        } else {
            migrations.join("; ")
        };

        let checkpoint = CawoCheckpoint {
            step_id: next_step_id,
            parent_step_id,
            timestamp: Utc::now(),
            stage: capsule.current_stage,
            intent: format!("Version migration to protocol v{}", CURRENT_CLI_VERSION),
            codebase_ast_hash: ast_hash,
            reality_status,
            rationale: Some(format!("Automated version migration: {}", migration_summary)),
        };

        journal.append(checkpoint);
        journal.save_to_file(journal_path)?;

        Ok(migrations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_adapter_clean_and_outdated() {
        let test_id = Utc::now().timestamp_nanos_opt().unwrap_or(12345);
        let temp_path = std::env::temp_dir().join(format!("test_mapai_adapter_{}", test_id));
        let mapai_dir = temp_path.join(".mapai");
        fs::create_dir_all(&mapai_dir).unwrap();

        let capsule = ProjectCapsule {
            schema_version: "saia/v0".to_string(),
            cli_version: Some("0.0.1".to_string()),
            ..Default::default()
        };
        let toml_str = toml::to_string(&capsule).unwrap();
        fs::write(mapai_dir.join("capsule.toml"), toml_str).unwrap();

        let report = VersionAdaptiveEngine::inspect_project(&temp_path).unwrap();
        assert!(report.migration_required);
        assert!(!report.changes_summary.is_empty());

        let migrations = VersionAdaptiveEngine::execute_migration(&temp_path).unwrap();
        assert!(!migrations.is_empty());

        let report_after = VersionAdaptiveEngine::inspect_project(&temp_path).unwrap();
        assert!(!report_after.migration_required);

        let _ = fs::remove_dir_all(&temp_path);
    }
}
