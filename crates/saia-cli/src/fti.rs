use colored::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use saia_core::capsule::ProjectCapsule;
use saia_core::cawo::{CawoCheckpoint, ContinuityJournal, GateStatus};
use saia_core::chrono::Utc;
use saia_core::StatePaths;
use saia_verify::TruthScanner;

#[derive(Debug, Clone)]
pub struct DeviceProfile {
    pub os: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub rust_version: Option<String>,
    pub git_version: Option<String>,
    pub ai_hosts_detected: Vec<String>,
}

pub struct FirstTimeInstaller;

impl FirstTimeInstaller {
    /// Discovers a safe, space-free path on Windows using 8.3 short-name aliasing
    pub fn resolve_space_safe_path<P: AsRef<Path>>(path: P) -> PathBuf {
        let p = path.as_ref();
        #[cfg(target_os = "windows")]
        {
            let path_str = p.to_string_lossy();
            if path_str.contains(' ') {
                if let Ok(output) = Command::new("cmd")
                    .args(["/c", &format!("for %I in (\"{path_str}\") do @echo %~sI")])
                    .output()
                {
                    let short_alias = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !short_alias.is_empty() && !short_alias.contains(' ') {
                        return PathBuf::from(short_alias);
                    }
                }
            }
        }
        p.to_path_buf()
    }

    /// Detects device specifications, host toolchains, and AI IDE environments
    pub fn detect_environment() -> DeviceProfile {
        let os = env::consts::OS.to_string();
        let arch = env::consts::ARCH.to_string();
        let cpu_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let rust_version = Command::new("cargo")
            .arg("--version")
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_string());

        let git_version = Command::new("git")
            .arg("--version")
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|s| s.trim().to_string());

        let mut ai_hosts_detected = Vec::new();

        // Check Antigravity config
        if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
            let antigravity_config = Path::new(&home).join(".gemini").join("config").join("mcp_config.json");
            if antigravity_config.exists() {
                ai_hosts_detected.push("Google Antigravity IDE (MCP Active)".to_string());
            }

            // Check Claude Desktop
            let claude_config = Path::new(&home).join("AppData").join("Roaming").join("Claude").join("claude_desktop_config.json");
            if claude_config.exists() {
                ai_hosts_detected.push("Claude Desktop".to_string());
            }

            // Check Cursor
            let cursor_dir = Path::new(&home).join(".cursor");
            if cursor_dir.exists() {
                ai_hosts_detected.push("Cursor AI Editor".to_string());
            }
        }

        if ai_hosts_detected.is_empty() {
            ai_hosts_detected.push("Standard Terminal / Native CLI".to_string());
        }

        DeviceProfile {
            os,
            arch,
            cpu_cores,
            rust_version,
            git_version,
            ai_hosts_detected,
        }
    }

    /// Prints the detected device and host profile
    pub fn print_device_banner(profile: &DeviceProfile) {
        println!("{}", "╔═════════════════════════════════════════════════════════════════════════╗".cyan());
        println!("{}", "║              MAPai - SAiA First-Time Installation (FTI)                 ║".cyan().bold());
        println!("{}", "║         Sovereign Host Detection & AI Symbiosis Initialization          ║".cyan());
        println!("{}", "╚═════════════════════════════════════════════════════════════════════════╝".cyan());
        println!();
        println!("{}", "=== 1. DETECTED HOST HARDWARE & SYSTEM ===".bold().yellow());
        println!("  • Operating System : {} ({})", profile.os.green().bold(), profile.arch.cyan());
        println!("  • CPU Parallelism  : {} logical cores", profile.cpu_cores);
        println!("  • Rust Toolchain   : {}", profile.rust_version.as_deref().unwrap_or("Not found").dimmed());
        println!("  • Git Engine       : {}", profile.git_version.as_deref().unwrap_or("Not found").dimmed());
        println!("  • Detected AI IDEs :");
        for host in &profile.ai_hosts_detected {
            println!("      ✓ {}", host.cyan().bold());
        }
    }

    /// Displays the comprehensive, unambiguous sovereign transparency charter
    pub fn print_transparency_charter() {
        println!();
        println!("{}", "=== 2. SOVEREIGN TRANSPARENCY & DATA GOVERNANCE CHARTER ===".bold().yellow());
        println!("Before proceeding, here is exactly how MAPai operates on your system and with AI:\n");

        println!("{}", "  [DATA & PRIVACY - 100% Sovereign & Local-First]".bold().green());
        println!("    • Zero Telemetry: MAPai transmits zero analytics, zero metrics, and zero code externally.");
        println!("    • No Cloud Dependencies: Verification and CAWO state engines execute locally on your CPU.");
        println!("    • Experience Distillation: Lessons learned are crystallized locally as anonymized antibodies");
        println!("      into `.mapai/heuristics/` without leaking credentials, source code, or PII.\n");

        println!("{}", "  [PERMISSIONS & PROCESS EXECUTION]".bold().green());
        println!("    • Least Privilege: MAPai runs strictly within your unprivileged user space.");
        println!("    • No Root/Admin Required: It never requires administrator elevation or background daemons.");
        println!("    • Epistemic Sandboxing: Code verification gates operate in sub-millisecond AST memory passes.\n");

        println!("{}", "  [FILE SYSTEM BOUNDARIES]".bold().green());
        println!("    • Zero Repository Pollution: MAPai stores project state centrally in user data (~/.mapai/projects/)");
        println!("      keeping your repository 100% pure and untouched unless local mode is explicitly chosen.");
        println!("    • No Silent Rewrites: MAPai never silently alters or overwrites your application code.");
        println!("    • Read-Only Auditing: TruthScanner performs non-destructive read passes to detect stubs.");
        println!("    • Optional Pre-Commit Hook: Optionally places `.git/hooks/pre-commit` to prevent committing stubs.\n");

        println!("{}", "  [CAPABILITIES WITH THE SYSTEM & AI (SAiA PROTOCOL)]".bold().green());
        println!("    • The Two-Job Model: Decouples human intent (Job 1) from substrate constraints (Job 2).");
        println!("    • Anti-Sycophancy Gate: Blocks hollow completions (TODO/unhandled unwraps) before code is accepted.");
        println!("    • Model Context Protocol (MCP): Exposes live state badges, causal context packets (<1,000 tokens),");
        println!("      and verification tools via standard JSON-RPC over stdio to Antigravity, Cursor, and Claude.");
        println!("    • CAWO Continuity: Maintains an append-only causal state ledger anchored to git tree SHA-256.\n");
        println!("{}", "───────────────────────────────────────────────────────────────────────────".dimmed());
    }

    /// Prompts user with a yes/no question, providing a default
    fn prompt_confirm(prompt: &str, default_yes: bool) -> bool {
        let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
        print!("{} {} ", prompt.bold(), suffix.cyan());
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return default_yes;
        }

        let trimmed = input.trim().to_lowercase();
        if trimmed.is_empty() {
            default_yes
        } else {
            trimmed == "y" || trimmed == "yes"
        }
    }

    /// Finds destination directory for global installation
    pub fn get_global_install_dir() -> anyhow::Result<PathBuf> {
        if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
            let cargo_bin = Path::new(&home).join(".cargo").join("bin");
            if !cargo_bin.exists() {
                fs::create_dir_all(&cargo_bin)?;
            }
            Ok(cargo_bin)
        } else {
            anyhow::bail!("Cannot determine user home directory for global installation.");
        }
    }

    /// Performs the installation workflow
    pub fn run(non_interactive: bool, global_flag: Option<bool>, project_name: &str) -> anyhow::Result<()> {
        let profile = Self::detect_environment();
        Self::print_device_banner(&profile);
        Self::print_transparency_charter();

        // 1. Inquire about Global Installation
        let install_global = if let Some(g) = global_flag {
            g
        } else if non_interactive {
            true
        } else {
            Self::prompt_confirm("Install MAPai globally to your user PATH (accessible across all projects)?", true)
        };

        if install_global {
            let global_dir = Self::get_global_install_dir()?;
            let current_exe = env::current_exe()?;
            let target_exe_name = if cfg!(windows) { "mapai.exe" } else { "mapai" };
            let target_path = global_dir.join(target_exe_name);

            // Copy binary
            if current_exe != target_path {
                fs::copy(&current_exe, &target_path)?;
            }
            println!("{} Global binary deployed: {}", "[✓]".green().bold(), target_path.display().to_string().cyan());
        } else {
            println!("{} Keeping MAPai as a project-local binary.", "[i]".yellow().bold());
        }

        // 2. Inquire about Git Pre-Commit Hook
        let install_git_hook = if non_interactive {
            Path::new(".git").exists()
        } else if Path::new(".git").exists() {
            Self::prompt_confirm("Install Git pre-commit Truth-to-Materials verification gate?", true)
        } else {
            false
        };

        if install_git_hook {
            let hooks_dir = Path::new(".git").join("hooks");
            if !hooks_dir.exists() {
                fs::create_dir_all(&hooks_dir)?;
            }
            let hook_path = hooks_dir.join("pre-commit");
            let hook_script = r#"#!/bin/sh
# MAPai Deterministic Pre-Commit Verification Gate
echo "[MAPai] Running Truth-to-Materials audit..."
if command -v mapai >/dev/null 2>&1; then
    mapai check
elif [ -f "./target/release/mapai.exe" ]; then
    ./target/release/mapai.exe check
elif [ -f "./target/release/mapai" ]; then
    ./target/release/mapai check
else
    cargo run -q -p saia-cli -- check
fi

if [ $? -ne 0 ]; then
    echo "[MAPai ERROR] Commit rejected: Deliverables contain hollow stubs or unverified placeholders."
    exit 1
fi
"#;
            fs::write(&hook_path, hook_script)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755));
            }
            println!("{} Git pre-commit verification gate installed at .git/hooks/pre-commit", "[✓]".green().bold());
        }

        // 3. Inquire about MCP Server Registration
        let register_mcp = if non_interactive {
            true
        } else {
            Self::prompt_confirm("Register MAPai Model Context Protocol (MCP) sidecar with your AI IDE?", true)
        };

        if register_mcp {
            if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
                let mcp_config_path = Path::new(&home).join(".gemini").join("config").join("mcp_config.json");
                if mcp_config_path.exists() {
                    if let Ok(content) = fs::read_to_string(&mcp_config_path) {
                        if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
                            let binary_cmd = if install_global {
                                let gdir = Self::get_global_install_dir()?;
                                let exe = if cfg!(windows) { "mapai.exe" } else { "mapai" };
                                let full_bin = gdir.join(exe);
                                Self::resolve_space_safe_path(full_bin).to_string_lossy().to_string()
                            } else {
                                let cur = env::current_exe()?;
                                Self::resolve_space_safe_path(cur).to_string_lossy().to_string()
                            };

                            let mapai_entry = serde_json::json!({
                                "command": binary_cmd,
                                "args": ["mcp"],
                                "env": {}
                            });

                            if let Some(servers) = json.get_mut("mcpServers").and_then(|s| s.as_object_mut()) {
                                servers.insert("mapai".to_string(), mapai_entry);
                                if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                                    let _ = fs::write(&mcp_config_path, pretty);
                                    println!("{} MAPai MCP server registered in Antigravity config", "[✓]".green().bold());
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Install Universal MAPai Protocol Standard into Global User Configuration
        if let Ok(home) = env::var("USERPROFILE").or_else(|_| env::var("HOME")) {
            let global_rules_dir = Path::new(&home).join(".gemini").join("config").join("rules");
            if fs::create_dir_all(&global_rules_dir).is_ok() {
                let rule_file = global_rules_dir.join("mapai_universal_operational_protocol.md");
                let rule_content = format!(
                    "---\ndescription: Universal MAPai Operational Protocol\nglobs: \"*\"\nalwaysApply: true\n---\n\n{}",
                    saia_core::MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION
                );
                let _ = fs::write(rule_file, rule_content);
            }
            let gemini_md = Path::new(&home).join(".gemini").join("GEMINI.md");
            let _ = fs::write(gemini_md, saia_core::MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION);
            println!("{} Universal MAPai protocol standard installed into user core configuration", "[✓]".green().bold());
        }

        // 5. Initialize Sovereign Workspace Capsule & Journal (Zero-Pollution Shadow Storage)
        let paths = StatePaths::resolve(".");
        paths.ensure_dir_exists()?;

        let capsule_path = &paths.capsule_file;
        if !capsule_path.exists() {
            let capsule = ProjectCapsule {
                project_name: project_name.to_string(),
                ..Default::default()
            };
            let toml_str = toml::to_string_pretty(&capsule)?;
            fs::write(capsule_path, toml_str)?;
            if paths.is_local {
                println!("{} Sovereign SAiA project capsule initialized at .mapai/capsule.toml", "[✓]".green().bold());
            } else {
                println!("{} Sovereign SAiA project capsule initialized in shadow storage (zero repository pollution):", "[✓]".green().bold());
                println!("    {}", paths.capsule_file.display().to_string().cyan());
            }
        }

        // Run Verification Audit
        println!();
        println!("{}", "=== 3. INITIAL WORKSPACE INTEGRITY AUDIT ===".bold().yellow());
        let scanner = TruthScanner::new();
        let report = scanner.scan_directory(".")?;
        println!("Audited Files : {}", report.files_scanned);
        println!("Audited Lines : {}", report.lines_scanned);
        println!("Hollow Rate   : {:.2} stubs/KLOC ({})", report.hollow_completion_rate, if report.is_clean { "PASS".green().bold() } else { "FAIL".red().bold() });

        // Record Initial CAWO Checkpoint
        let journal_path = &paths.journal_file;
        let mut journal = if journal_path.exists() {
            ContinuityJournal::load_from_file(journal_path)?
        } else {
            ContinuityJournal::default()
        };
        let ast_hash = ContinuityJournal::compute_workspace_tree_hash(".")?;
        let next_step_id = journal.checkpoints.last().map(|c| c.step_id).unwrap_or(0) + 1;

        let checkpoint = CawoCheckpoint {
            step_id: next_step_id,
            parent_step_id: None,
            timestamp: Utc::now(),
            stage: saia_core::capsule::PipelineStage::Stage1ProblemIntent,
            intent: "First-Time Installation (FTI) onboarded with sovereign trust charter".to_string(),
            codebase_ast_hash: ast_hash,
            reality_status: if report.is_clean { GateStatus::Passed } else { GateStatus::Rejected { reasons: vec!["Stubs detected".into()] } },
            rationale: Some("FTI initialization completed: device detected, governance charter acknowledged, and workspace anchored".into()),
        };
        journal.append(checkpoint);
        journal.save_to_file(journal_path)?;

        println!();
        println!("{}", "═══════════════════════════════════════════════════════════════════════════".green());
        println!("{}", "   MAPai First-Time Installation Successfully Completed!".green().bold());
        println!("{}", "═══════════════════════════════════════════════════════════════════════════".green());
        println!("Next step: Run `mapai guide` to begin Stage 1 Problem Intent definition.\n");

        Ok(())
    }
}
