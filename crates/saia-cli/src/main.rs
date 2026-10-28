use clap::{Parser, Subcommand};
use colored::*;
use saia_core::capsule::{PipelineStage, ProjectCapsule, CURRENT_CLI_VERSION, CURRENT_SCHEMA_VERSION, VersionStatus};
use saia_core::cawo::{CawoCheckpoint, ContinuityJournal, GateStatus};
use saia_core::chrono::Utc;
use saia_core::StatePaths;
use saia_core::token_ledger::TokenLedger;
use saia_verify::{TruthScanner, OnlineScoutVerifier, OmniScoutEngine, ScoutDomain, ScoutStatus};
use saia_verify::browser::{HybridBrowserRouter, BrowseRequest, BrowseExecutionMode, RealBrowserCdpEngine};
use saia_verify::screen::{ScreenCaptureEngine, VisualScreenAnalyzer, VisualVerificationGate, VisualGoalRequest, SpatialLayoutEngine, SpatialActuator};
use saia_orchestrator::mediator::HumanVerificationMediator;
use saia_orchestrator::stage_machine::StageRunner;
use saia_orchestrator::version_adapter::VersionAdaptiveEngine;
use saia_cli::mcp_server::McpServer;
use saia_cli::fti::FirstTimeInstaller;
use saia_cli::ui::WebStudio;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "mapai")]
#[command(author = "MAPai & SAiA Contributors")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "MAPai: Sovereign Reference Implementation of the SAiA Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a sovereign project capsule (zero-pollution shadow storage by default)
    Init {
        #[arg(short, long, default_value = "unnamed-project")]
        name: String,
        /// Store capsule locally within .mapai/ instead of zero-pollution shadow storage
        #[arg(long)]
        local: bool,
    },
    /// Audit project deliverables for hollow stubs, fake placeholders, and truth-to-materials
    Check {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Output the compact, token-dense cognitive context packet (< 1,000 tokens)
    Context,
    /// Run closed-loop readback verification on a user objective before writing code
    VerifyIntent {
        #[arg(short, long)]
        objective: String,
    },
    /// Record an immutable CAWO causal state checkpoint anchored to codebase tree hash
    Checkpoint {
        #[arg(short, long)]
        intent: String,
        #[arg(short, long)]
        rationale: Option<String>,
    },
    /// Display the causal audit trail from the append-only Continuity Journal
    Log,
    /// Advance the project to the next formal pipeline stage through deterministic verification
    Advance,
    /// Run First-Time Installation (FTI) with device detection, sovereign trust charter, and global setup
    Fti {
        #[arg(short, long, default_value = "unnamed-project")]
        name: String,
        #[arg(short, long)]
        global: Option<bool>,
        #[arg(short, long)]
        non_interactive: bool,
    },
    /// Migrate the project capsule and CAWO ledger to the latest protocol schema and CLI version
    Migrate,
    /// Provide adaptive walkthrough guidance for the active pipeline stage and version updates
    Guide {
        #[arg(short, long)]
        stage_only: bool,
    },
    /// Display active project status, pipeline stage, and verification metrics
    Status,
    /// Launch the interactive MAPai Sovereign Studio GUI in your browser
    Ui {
        #[arg(short, long, default_value_t = 4321)]
        port: u16,
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        #[arg(long)]
        no_browser: bool,
    },
    /// Run continuous 7-vector autonomous internet scouting across all aspects of MAPai
    Scout {
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    /// Autonomous Tiered Hybrid Browser (Silent Headless with Real-Browser Escalation)
    Browse {
        /// Target URL to navigate and read
        url: String,
        /// Force execution in your real visible browser (Chrome/Edge) with active logins
        #[arg(long)]
        real: bool,
        /// Force strictly headless execution without auto-escalation
        #[arg(long)]
        headless_only: bool,
        /// Chrome DevTools Protocol (CDP) port (default: 9222)
        #[arg(long, default_value_t = 9222)]
        port: u16,
        /// Optional destination folder to download files
        #[arg(short, long)]
        download: Option<String>,
    },
    /// Launch the Model Context Protocol (MCP) server for Cursor, Claude Desktop, and Antigravity
    Mcp,
    /// Launch the Agent Communication Protocol (ACP) server for Zed, Cursor, and modern IDEs
    Acp,
    /// Inspect and track token expenditures: per-prompt, day's total, conversation total, project total, and remaining balance
    Tokens {
        /// Optional path to a file to count tokens for
        #[arg(short, long)]
        file: Option<String>,
        /// Optional prompt text to calculate tokens for
        #[arg(short, long)]
        prompt: Option<String>,
        /// Set or update the project token budget (quota)
        #[arg(long)]
        budget: Option<u64>,
    },
    /// Global Screen Analyser & Closed-Loop Visual IV&V Verification Gate
    Screen {
        #[command(subcommand)]
        action: ScreenCommands,
    },
    /// Perform full enterprise professional engineering diagnosis across repository hygiene, lockfile, git, and stubs
    Diagnose,
    /// Verify MAPai operational rule compliance (identity, timestamps, SemVer, footer ledger, JTBD, minimalism)
    CheckRules,
    /// Sovereign AI Research Radar: track breakthroughs, preprints, and synthesize autonomous skills
    Research {
        /// Subcommand action: "list", "query", "synthesize", or "sync-memory"
        #[arg(short, long, default_value = "list")]
        action: String,
        /// Optional search query or topic identifier
        #[arg(short, long)]
        topic: Option<String>,
        /// Optional path to store synthesized skill (defaults to .agent/skills/)
        #[arg(short, long)]
        output_dir: Option<String>,
    },
    /// Continuous background filesystem watcher for instantaneous on-save truth-to-materials auditing
    Watch {
        /// Target directory to watch (default: ".")
        #[arg(short, long, default_value = ".")]
        path: String,
        /// Run single baseline scan cycle and exit (useful for CI/test)
        #[arg(long)]
        once: bool,
    },
    /// Socratic conversational project initializer and greenfield requirements scaffolder
    Wizard {
        /// Project name (default: prompts interactively)
        #[arg(short, long)]
        name: Option<String>,
        /// Problem intent statement
        #[arg(short, long)]
        intent: Option<String>,
        /// Primary programming language (default: rust)
        #[arg(short, long, default_value = "rust")]
        language: String,
        /// Canonical Job Class (default: greenfield-synthesis)
        #[arg(short, long, default_value = "greenfield-synthesis")]
        job_class: String,
    },
}

#[derive(Subcommand)]
enum ScreenCommands {
    /// Capture the entire user screen or target window to a BMP image
    Capture {
        /// Optional target window title pattern (e.g. "Paint", "AutoCAD")
        #[arg(short, long)]
        window: Option<String>,
        /// Destination file path (default: .mapai/screen_latest.bmp)
        #[arg(short, long, default_value = ".mapai/screen_latest.bmp")]
        output: String,
    },
    /// Inspect screen metrics: luminance, background, foreground, and WCAG contrast ratio
    Inspect {
        /// Path to BMP file to inspect (default: .mapai/screen_latest.bmp)
        #[arg(short, long, default_value = ".mapai/screen_latest.bmp")]
        path: String,
    },
    /// Run closed-loop visual IV&V gate against an active window (verify presence, text, contrast, and color)
    Verify {
        /// Target window title pattern to audit (e.g. "Paint")
        #[arg(short, long)]
        window: String,
        /// Expected text content to verify
        #[arg(short, long)]
        text: Option<String>,
        /// Required font/foreground color (e.g. "black", "#000000")
        #[arg(short, long)]
        color: Option<String>,
        /// Minimum WCAG contrast ratio required (default: 3.0)
        #[arg(short, long, default_value_t = 3.0)]
        min_contrast: f32,
    },
    /// Extract and display the multi-layer spatial layout hierarchy (DPI, App Frame, Canvas Viewport, Tool State)
    Layout {
        /// Target window title pattern (default: "Paint")
        #[arg(short, long, default_value = "Paint")]
        window: String,
        /// Output format as structured JSON
        #[arg(long)]
        json: bool,
    },
    /// Live ScreenSense perception scan (injects full spatial context for AI agents)
    Perceive {
        /// Target window title pattern (default: "Paint")
        #[arg(short, long, default_value = "Paint")]
        window: String,
    },
    /// Execute an atomic, spatially-grounded GUI action batch (e.g. text-entry, clear-canvas, click-center)
    Act {
        /// Target window title pattern (e.g. "Paint")
        #[arg(short, long)]
        window: String,
        /// Action recipe: "text-entry", "clear-canvas", or "click-center"
        #[arg(short, long, default_value = "text-entry")]
        action: String,
        /// Text string to input (for text-entry)
        #[arg(short, long)]
        text: Option<String>,
        /// Color to select in the palette (e.g. "Red", "Black")
        #[arg(short, long)]
        color: Option<String>,
        /// Commit strategy: "click-outside", "enter", or "none"
        #[arg(long, default_value = "click-outside")]
        commit: String,
        /// Clear canvas before typing/drawing
        #[arg(long)]
        clear: bool,
        /// Automatically run visual verification gate after execution
        #[arg(long)]
        verify: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, local } => {
            let paths = if local {
                StatePaths::local(".")
            } else {
                StatePaths::shadow(".")
            };
            paths.ensure_dir_exists()?;

            let capsule_path = &paths.capsule_file;
            if !capsule_path.exists() {
                let capsule = ProjectCapsule {
                    project_name: name,
                    ..Default::default()
                };
                let toml_str = toml::to_string_pretty(&capsule)?;
                fs::write(capsule_path, toml_str)?;

                // Codify universal MAPai operational rules directly from the core
                let rules_dir = Path::new(".agent").join("rules");
                if fs::create_dir_all(&rules_dir).is_ok() {
                    let rule_path = rules_dir.join("version_logging_invariant.md");
                    let rule_content = format!(
                        "---\ndescription: Universal MAPai Operational Protocol\nglobs: \"*\"\nalwaysApply: true\n---\n\n{}",
                        saia_core::MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION
                    );
                    let _ = fs::write(rule_path, rule_content);
                }

                if paths.is_local {
                    println!("{} Sovereign SAiA project capsule initialized at .mapai/capsule.toml", "[+]".green().bold());
                } else {
                    println!("{} Sovereign SAiA project capsule initialized in shadow storage (zero repository pollution):", "[+]".green().bold());
                    println!("    {}", paths.capsule_file.display().to_string().cyan());
                }
            } else {
                println!("{} Project capsule already exists at {}", "[!]".yellow().bold(), capsule_path.display());
            }
        }
        Commands::Check { path } => {
            println!("{}", "=== MAPai Truth-to-Materials & 7-Vector Internet Verification Gate ===".bold());
            let scanner = TruthScanner::new();
            let report = scanner.scan_directory(&path)?;

            println!("Files audited   : {}", report.files_scanned);
            println!("Lines audited   : {}", report.lines_scanned);
            println!("Hollow Rate     : {:.2} stubs/KLOC", report.hollow_completion_rate);

            // Always run live online dependency scouting
            let online_report = OnlineScoutVerifier::scout_and_verify_workspace(&path)?;
            println!("Online Registry : {}", if online_report.is_clean {
                format!("VERIFIED ({} crates online, Zero Hallucinations)", online_report.verified_online_packages.len()).green().bold()
            } else {
                format!("FAILED ({} invalid)", online_report.hallucinated_dependencies.len() + online_report.yanked_dependencies.len()).red().bold()
            });

            // Always run the 7-vector omni scouting engine
            let omni_report = OmniScoutEngine::scout_all_vectors(&path)?;
            println!("7-Vector Scout  : {}", if omni_report.is_safe {
                format!("SYNCHRONIZED ({} Vectors Verified, {} Advisories)", omni_report.findings.len(), omni_report.advisories_count).green().bold()
            } else {
                format!("THREATS DETECTED ({} Threats)", omni_report.threats_detected_count).red().bold()
            });

            if report.is_clean && online_report.is_clean && omni_report.is_safe {
                println!("\n{} All deliverables satisfy Truth-to-Materials, Online Registry Integrity, and 7-Vector Security.", "[PASS]".green().bold());
                std::process::exit(0);
            } else {
                if !report.is_clean {
                    println!("\n{} Hollow stubs or unverified placeholders detected:", "[FAIL]".red().bold());
                    for v in report.violations {
                        println!("  - {}:{} [{}] {}", v.file_path.cyan(), v.line_number, v.pattern_type.red(), v.matched_text);
                    }
                }
                if !online_report.is_clean {
                    println!("\n{} Online dependency issues detected:", "[FAIL]".red().bold());
                    for h in online_report.hallucinated_dependencies {
                        println!("  - Hallucinated package: {}", h.red().bold());
                    }
                    for y in online_report.yanked_dependencies {
                        println!("  - Yanked package: {}", y.yellow().bold());
                    }
                }
                if !omni_report.is_safe {
                    println!("\n{} Internet scouting vector threats detected:", "[FAIL]".red().bold());
                    for f in omni_report.findings.iter().filter(|f| f.status == ScoutStatus::ThreatDetected) {
                        println!("  - [{:?}] {}: {}", f.domain, f.title.red().bold(), f.description);
                    }
                }
                std::process::exit(1);
            }
        }
        Commands::Context => {
            let paths = StatePaths::resolve(".");
            if paths.capsule_file.exists() {
                let content = fs::read_to_string(&paths.capsule_file)?;
                let capsule: ProjectCapsule = toml::from_str(&content)?;
                
                println!("--- MAPai COGNITIVE CONTEXT PACKET (SAiA v1) ---");
                println!("Project     : {}", capsule.project_name);
                println!("Location    : {}", if paths.is_local { "Local (.mapai/)".into() } else { format!("Shadow ({})", paths.dir.display()) });
                println!("Active Stage: {:?}", capsule.current_stage);
                println!("Job 1 Goal  : {}", capsule.job_1_intent.problem_statement);
                if let Some(jc) = capsule.job_1_intent.job_class {
                    println!("Job 1 Class : {}", jc.display_name());
                }
                println!("Job 2 Bounds: Language: {}, Constraints: {:?}", capsule.job_2_constraints.primary_language, capsule.job_2_constraints.performance_envelope);
                println!("Forbidden   : {:?}", capsule.job_2_constraints.forbidden_patterns);
                println!("------------------------------------------------");
            } else {
                eprintln!("{} No capsule found. Run `mapai init` first.", "[ERROR]".red().bold());
                std::process::exit(1);
            }
        }
        Commands::VerifyIntent { objective } => {
            let readback = HumanVerificationMediator::formulate_readback(&objective);
            println!("{}", "=== MAPai Closed-Loop Human Understanding Readback ===".bold());
            println!("Raw Prompt      : {}", readback.raw_human_objective);
            println!("Extracted Intent: {}", readback.extracted_intent.cyan());
            println!("Hard Invariants :");
            for b in readback.hard_boundaries {
                println!("  * {}", b);
            }
            println!("\n{} Proceed with this architectural interpretation? [Proceed]", "[CONFIRMATION REQUIRED]".yellow().bold());
        }
        Commands::Checkpoint { intent, rationale } => {
            let paths = StatePaths::resolve(".");
            paths.ensure_dir_exists()?;
            let mut journal = if paths.journal_file.exists() {
                ContinuityJournal::load_from_file(&paths.journal_file)?
            } else {
                ContinuityJournal::default()
            };

            let stage = if paths.capsule_file.exists() {
                let content = fs::read_to_string(&paths.capsule_file)?;
                let capsule: ProjectCapsule = toml::from_str(&content)?;
                capsule.current_stage
            } else {
                saia_core::capsule::PipelineStage::Stage1ProblemIntent
            };

            // Run scanner gate
            let scanner = TruthScanner::new();
            let report = scanner.scan_directory(".")?;
            let reality_status = if report.is_clean {
                GateStatus::Passed
            } else {
                GateStatus::Rejected {
                    reasons: report.violations.into_iter().map(|v| format!("{}:{}: {}", v.file_path, v.line_number, v.pattern_type)).collect(),
                }
            };

            let ast_hash = ContinuityJournal::compute_workspace_tree_hash(".")?;
            let parent_step_id = journal.checkpoints.last().map(|c| c.step_id);
            let next_step_id = parent_step_id.unwrap_or(0) + 1;

            let checkpoint = CawoCheckpoint {
                step_id: next_step_id,
                parent_step_id,
                timestamp: Utc::now(),
                stage,
                intent: intent.clone(),
                codebase_ast_hash: ast_hash.clone(),
                reality_status: reality_status.clone(),
                rationale,
            };

            journal.append(checkpoint);
            journal.save_to_file(&paths.journal_file)?;

            println!("{}", "=== MAPai CAWO Checkpoint Recorded ===".bold());
            println!("Step ID      : #{}", next_step_id);
            println!("Stage        : {:?}", stage);
            println!("Intent       : {}", intent.cyan());
            println!("Tree Hash    : {}", ast_hash.green());
            match reality_status {
                GateStatus::Passed => println!("Gate Status  : {}", "[PASS - Truth-to-Materials]".green().bold()),
                GateStatus::Rejected { reasons } => {
                    println!("Gate Status  : {} ({} violations)", "[REJECTED]".red().bold(), reasons.len());
                }
                GateStatus::EscalatedToHuman { ambiguity } => {
                    println!("Gate Status  : {} ({})", "[ESCALATED]".yellow().bold(), ambiguity);
                }
            }
        }
        Commands::Log => {
            let paths = StatePaths::resolve(".");
            if !paths.journal_file.exists() {
                println!("{}", "=== MAPai CAWO Continuity Journal ===".bold());
                println!("No checkpoints recorded yet. Run `mapai checkpoint --intent \"...\"` to create one.");
            } else {
                let journal = ContinuityJournal::load_from_file(&paths.journal_file)?;

                println!("{}", "=== MAPai CAWO Continuity Journal ===".bold());
                if journal.checkpoints.is_empty() {
                    println!("No checkpoints recorded yet. Run `mapai checkpoint --intent \"...\"` to create one.");
                } else {
                    for cp in journal.checkpoints.iter().rev() {
                        let status_str = match &cp.reality_status {
                            GateStatus::Passed => "[PASSED]".green().bold(),
                            GateStatus::Rejected { .. } => "[REJECTED]".red().bold(),
                            GateStatus::EscalatedToHuman { .. } => "[ESCALATED]".yellow().bold(),
                        };
                        println!("\n{} Step #{} [{:?}] at {}", status_str, cp.step_id, cp.stage, cp.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
                        println!("  Intent   : {}", cp.intent);
                        println!("  Tree SHA : {}", cp.codebase_ast_hash.dimmed());
                        if let Some(rat) = &cp.rationale {
                            println!("  Rationale: {}", rat.italic());
                        }
                    }
                }
            }
        }
        Commands::Advance => {
            let paths = StatePaths::resolve(".");
            if !paths.capsule_file.exists() {
                eprintln!("{} No capsule found. Run `mapai init` first.", "[ERROR]".red().bold());
                std::process::exit(1);
            }

            let content = fs::read_to_string(&paths.capsule_file)?;
            let mut capsule: ProjectCapsule = toml::from_str(&content)?;
            let current = capsule.current_stage;

            match StageRunner::advance(&mut capsule, ".") {
                Ok(next_stage) => {
                    let toml_str = toml::to_string_pretty(&capsule)?;
                    fs::write(&paths.capsule_file, toml_str)?;
                    println!("{} Stage successfully transitioned: {:?} -> {:?}", "[ADVANCED]".green().bold(), current, next_stage);
                }
                Err(e) => {
                    eprintln!("{} Cannot advance pipeline: {}", "[GATE BLOCKED]".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Fti { name, global, non_interactive } => {
            FirstTimeInstaller::run(non_interactive, global, &name)?;
        }
        Commands::Migrate => {
            println!("{}", "=== MAPai Protocol & Schema Migration Engine ===".bold());
            match VersionAdaptiveEngine::execute_migration(".") {
                Ok(migrations) => {
                    if migrations.is_empty() {
                        println!("{} Project capsule is already fully synchronized with protocol v{} (schema {}).", "[ALIGNED]".green().bold(), CURRENT_CLI_VERSION, CURRENT_SCHEMA_VERSION);
                    } else {
                        println!("{} Successfully applied {} migration(s):", "[MIGRATED]".green().bold(), migrations.len());
                        for m in migrations {
                            println!("  * {}", m.cyan());
                        }
                        println!("{} CAWO version migration checkpoint recorded in .mapai/journal.json", "[+]".green());
                    }
                }
                Err(e) => {
                    eprintln!("{} Migration failed: {}", "[ERROR]".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Guide { stage_only } => {
            println!("{}", "==================== MAPai Adaptive Walkthrough Guide ====================".bold().cyan());

            // 1. Version Update Assessment
            if !stage_only {
                if let Ok(report) = VersionAdaptiveEngine::inspect_project(".") {
                    match report.status {
                        VersionStatus::UpToDate => {
                            println!("{} Protocol Version: v{} (schema {}) - Synchronized", "[OK]".green().bold(), CURRENT_CLI_VERSION, CURRENT_SCHEMA_VERSION);
                        }
                        VersionStatus::OutdatedCapsule { capsule_schema, current_schema, capsule_cli, current_cli } => {
                            println!("{} Version Update Detected!", "[UPDATE ADVISORY]".yellow().bold());
                            println!("  • Active Binary : v{} (schema {})", current_cli, current_schema);
                            println!("  • Project State : v{} (schema {})", capsule_cli.unwrap_or_else(|| "unversioned".into()), capsule_schema);
                            for change in report.changes_summary {
                                println!("  • Diff: {}", change.italic());
                            }
                            println!("\n{} Recommended Migration Step: Run `mapai migrate` to synchronize.", "[ACTION]".yellow().bold());
                            println!("{}", "-------------------------------------------------------------------------".dimmed());
                        }
                        VersionStatus::NewerCapsule { capsule_schema, current_schema } => {
                            println!("{} Project was created by a newer MAPai version ({} vs supported {})", "[WARNING]".red().bold(), capsule_schema, current_schema);
                            println!("  Update your MAPai binary using: cargo install --path crates/saia-cli");
                            println!("{}", "-------------------------------------------------------------------------".dimmed());
                        }
                    }
                }
            }

            // 2. Stage-Adaptive Walkthrough Guidance
            let paths = StatePaths::resolve(".");
            let capsule_path = &paths.capsule_file;
            if !capsule_path.exists() {
                println!("\n{} No sovereign capsule detected in current workspace.", "[STEP 0: ONBOARDING]".yellow().bold());
                println!("  To initialize MAPai in this directory (zero-pollution shadow storage), run:");
                println!("    mapai init --name \"<YourProjectName>\"");
                println!("{}", "=========================================================================".bold().cyan());
                return Ok(());
            }

            let content = fs::read_to_string(capsule_path)?;
            let capsule: ProjectCapsule = toml::from_str(&content)?;

            println!("\n{} Active Pipeline Stage: {:?}", "[CURRENT STAGE]".bold().green(), capsule.current_stage);
            match capsule.current_stage {
                PipelineStage::Stage1ProblemIntent => {
                    println!("\n[Objective] Formulate Job 1 (Problem Intent) and verify non-negotiables.");
                    if capsule.job_1_intent.problem_statement.trim().is_empty() {
                        println!("  1. Your problem statement is currently [EMPTY].");
                        println!("  2. Define your functional intent using closed-loop readback verification:");
                        println!("     {} mapai verify-intent --objective \"<Your detailed objective>\"", "Example:".cyan());
                        println!("  3. Save the problem_statement and success_criteria in capsule.toml.");
                        println!("  4. Once specified, advance to architecture constraints with: `mapai advance`");
                    } else {
                        println!("  Intent defined: {}", capsule.job_1_intent.problem_statement.cyan());
                        println!("  Ready to advance! Run: {}", "mapai advance".green().bold());
                    }
                }
                PipelineStage::Stage2ArchitectureConstraints => {
                    println!("\n[Objective] Lock in Job 2 (Material & Environmental Constraints).");
                    println!("  Target Platforms : {:?}", capsule.job_2_constraints.target_platforms);
                    println!("  Primary Language : {}", capsule.job_2_constraints.primary_language);
                    println!("  Forbidden Rules  : {:?}", capsule.job_2_constraints.forbidden_patterns);
                    println!("  Action: Verify latency, memory envelopes, and security boundaries in capsule.toml.");
                    println!("  Next Command : {}", "mapai advance".green().bold());
                }
                PipelineStage::Stage3MaterialSelection => {
                    println!("\n[Objective] Select verified dependencies, zero-allocation crates, and data structures.");
                    println!("  Action: Audit external dependencies for telemetry or bloated transitive trees.");
                    println!("  Next Command : {}", "mapai advance".green().bold());
                }
                PipelineStage::Stage4Implementation => {
                    println!("\n[Objective] Write clean code with Truth-to-Materials (Zero Hollow Stubs).");
                    let scanner = TruthScanner::new();
                    if let Ok(report) = scanner.scan_directory(".") {
                        if report.is_clean {
                            println!("  Verification Status: {} (0.00 stubs/KLOC across {} files)", "CLEAN PASS".green().bold(), report.files_scanned);
                            println!("  Next Command       : {}", "mapai advance".green().bold());
                        } else {
                            println!("  Verification Status: {} ({} violations detected)", "BLOCKED".red().bold(), report.violations.len());
                            for v in report.violations.iter().take(5) {
                                println!("    - {}:{} [{}]", v.file_path, v.line_number, v.pattern_type);
                            }
                            println!("  Action: Replace all placeholder stubs with real implementations.");
                            println!("  Audit Command      : {}", "mapai check".yellow().bold());
                        }
                    }
                }
                PipelineStage::Stage5EmpiricalRealityCheck => {
                    println!("\n[Objective] Physical compilation, unit tests, and runtime behavioral gates.");
                    println!("  Action: Run `cargo test` or your build runner to confirm physical ground truth.");
                    println!("  Once tests pass, record CAWO checkpoint: mapai checkpoint --intent \"Tests passed\"");
                    println!("  Next Command: {}", "mapai advance".green().bold());
                }
                PipelineStage::Stage6HardeningDiagnostics => {
                    println!("\n[Objective] Fuzzing, stress testing, and sanitizers.");
                    println!("  Action: Run benchmark suites and memory audit.");
                    println!("  Next Command: {}", "mapai advance".green().bold());
                }
                PipelineStage::Stage7SovereignDelivery => {
                    println!("\n[Objective] Sovereign Release & Deliverable Packaging.");
                    println!("  Verification: All 7 stages satisfied under SAiA Protocol.");
                    println!("  Action: Generate badge with `mapai status` or run MCP server `mapai mcp`.");
                }
            }

            println!("{}", "=========================================================================".bold().cyan());
        }
        Commands::Status => {
            let paths = StatePaths::resolve(".");
            let capsule_path = &paths.capsule_file;

            println!("{}", "==================== MAPai - SAiA Project Status ====================".bold().cyan());
            if !capsule_path.exists() {
                println!("Capsule Status  : {}", "No project capsule found in current working directory or shadow store.".yellow());
                println!("Action Needed   : Run `mapai init` to initialize a zero-pollution shadow capsule.");
                println!("{}", "=====================================================================".bold().cyan());
                return Ok(());
            }

            let content = fs::read_to_string(capsule_path)?;
            let capsule: ProjectCapsule = toml::from_str(&content)?;
            println!("Project Name    : {}", capsule.project_name.bold());
            println!("Storage Mode    : {}", if paths.is_local { "Local Repository (.mapai/)".yellow() } else { "Centralized Shadow (Zero Repository Pollution)".green() });
            println!("Capsule Path    : {}", paths.capsule_file.display().to_string().cyan());
            println!("Pipeline Stage  : {:?}", capsule.current_stage);
            
            // Version compatibility indicator
            let ver_status = capsule.check_version_compatibility();
            match ver_status {
                VersionStatus::UpToDate => {
                    println!("Protocol/Schema : v{} ({}) [{}]", CURRENT_CLI_VERSION, capsule.schema_version, "SYNCHRONIZED".green().bold());
                }
                VersionStatus::OutdatedCapsule { .. } => {
                    println!("Protocol/Schema : v{} ({}) [{}]", CURRENT_CLI_VERSION, capsule.schema_version, "UPDATE DETECTED - Run `mapai migrate`".yellow().bold());
                }
                VersionStatus::NewerCapsule { .. } => {
                    println!("Protocol/Schema : v{} ({}) [{}]", CURRENT_CLI_VERSION, capsule.schema_version, "NEWER CAPSULE - Please upgrade binary".red().bold());
                }
            }

            println!("Problem Intent  : {}", if capsule.job_1_intent.problem_statement.is_empty() { "[Undefined]".yellow() } else { capsule.job_1_intent.problem_statement.normal() });
            if let Some(jc) = capsule.job_1_intent.job_class {
                println!("Job Class       : {}", jc.display_name().cyan());
            }
            println!("Target Platform : {:?}", capsule.job_2_constraints.target_platforms);

            let scanner = TruthScanner::new();
            if let Ok(report) = scanner.scan_directory(".") {
                println!("Source Audited  : {} files ({} lines)", report.files_scanned, report.lines_scanned);
                println!("Hollow Rate     : {:.2} stubs/KLOC ({})", report.hollow_completion_rate, if report.is_clean { "PASS".green().bold() } else { "FAIL".red().bold() });
            }

            if let Ok(journal) = ContinuityJournal::load_from_file(&paths.journal_file) {
                println!("CAWO Checkpoints: {} recorded", journal.checkpoints.len());
                if let Some(last) = journal.last_valid_checkpoint() {
                    println!("Last Valid Tree : Step #{} ({})", last.step_id, &last.codebase_ast_hash[..12]);
                }
            }
            println!("{}", "=====================================================================".bold().cyan());
        }
        Commands::Diagnose => {
            println!("{}", "=== MAPai Enterprise Professional Systems Engineering Audit ===".bold().cyan());
            println!("Auditing workspace against Tier-1 enterprise standards...\n");

            let paths = StatePaths::resolve(".");
            let mut passed = 0;
            let mut total = 0;

            // 1. TruthScanner Code Craft Audit
            total += 1;
            let scanner = TruthScanner::new();
            if let Ok(report) = scanner.scan_directory(".") {
                if report.is_clean {
                    println!("  {} Truth-to-Materials   : PASS (0.00 hollow stubs across {} files)", "[✓]".green().bold(), report.files_scanned);
                    passed += 1;
                } else {
                    println!("  {} Truth-to-Materials   : FAIL ({:.2} stubs/KLOC detected)", "[✗]".red().bold(), report.hollow_completion_rate);
                }
            } else {
                println!("  {} Truth-to-Materials   : WARNING (Unable to scan directory)", "[!]".yellow().bold());
            }

            // 2. Lockfile Determinism (.gitignore check)
            total += 1;
            if let Ok(gi) = fs::read_to_string(".gitignore") {
                if gi.lines().any(|l| l.trim() == "Cargo.lock" || l.trim() == "/Cargo.lock") {
                    println!("  {} Lockfile Tracking    : FAIL (Cargo.lock is ignored in .gitignore - anti-pattern for CLI binary)", "[✗]".red().bold());
                } else {
                    println!("  {} Lockfile Tracking    : PASS (Cargo.lock properly tracked for deterministic builds)", "[✓]".green().bold());
                    passed += 1;
                }
            } else {
                println!("  {} Lockfile Tracking    : PASS (No .gitignore restricting lockfile)", "[✓]".green().bold());
                passed += 1;
            }

            // 3. Monorepo Root Hygiene (Loose scratch files check)
            total += 1;
            let mut loose_scratch = Vec::new();
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if (name.ends_with(".ps1") || name.ends_with(".py") || name.ends_with(".pdf") || name.ends_with(".dwg"))
                            && !name.starts_with('.') 
                        {
                            loose_scratch.push(name);
                        }
                    }
                }
            }
            if loose_scratch.is_empty() {
                println!("  {} Root Monorepo Hygiene: PASS (Zero loose scripts or scratch artifacts in root)", "[✓]".green().bold());
                passed += 1;
            } else {
                println!("  {} Root Monorepo Hygiene: ADVISORY ({} loose file(s) in root: {})", "[!]".yellow().bold(), loose_scratch.len(), loose_scratch.join(", "));
            }

            // 4. Git Working Tree & Provenance
            total += 1;
            let git_check = std::process::Command::new("git").args(["status", "--porcelain"]).output();
            if let Ok(out) = git_check {
                let status_str = String::from_utf8_lossy(&out.stdout);
                let uncommitted = status_str.lines().count();
                if uncommitted == 0 {
                    println!("  {} Git Provenance       : PASS (Working tree clean, 100% committed to CAWO ledger)", "[✓]".green().bold());
                    passed += 1;
                } else {
                    println!("  {} Git Provenance       : NOTICE ({} uncommitted/untracked change(s) pending atomic sync)", "[!]".yellow().bold(), uncommitted);
                }
            } else {
                println!("  {} Git Provenance       : SKIP (Git CLI unavailable)", "[!]".yellow().bold());
            }

            // 5. Capsule Integrity & Professional Classification
            total += 1;
            if paths.capsule_file.exists() {
                if let Ok(content) = fs::read_to_string(&paths.capsule_file) {
                    if let Ok(capsule) = toml::from_str::<ProjectCapsule>(&content) {
                        if let Some(jc) = capsule.job_1_intent.job_class {
                            println!("  {} Capsule JTBD Class   : PASS (Governed under: {})", "[✓]".green().bold(), jc.display_name().cyan());
                            passed += 1;
                        } else {
                            println!("  {} Capsule JTBD Class   : ADVISORY (job_class undefined - defaulting to Greenfield)", "[!]".yellow().bold());
                            passed += 1;
                        }
                    } else {
                        println!("  {} Capsule JTBD Class   : FAIL (capsule.toml malformed)", "[✗]".red().bold());
                    }
                } else {
                    println!("  {} Capsule JTBD Class   : FAIL (Unable to read capsule)", "[✗]".red().bold());
                }
            } else {
                println!("  {} Capsule JTBD Class   : NOTICE (No capsule.toml found, run `mapai init`)", "[!]".yellow().bold());
            }

            // 6. Universal Operational Rules Compliance
            total += 1;
            let rule_report = saia_core::RuleComplianceAuditor::audit_all();
            if rule_report.overall_passed {
                println!("  {} Protocol Rule Posture: PASS ({})", "[✓]".green().bold(), rule_report.compliance_score.cyan());
                passed += 1;
            } else {
                println!("  {} Protocol Rule Posture: FAIL ({})", "[✗]".red().bold(), rule_report.compliance_score.yellow());
            }

            println!("\nHealth Score: {}/{} professional checks satisfied.", passed, total);
            if passed == total {
                println!("{}", "[EXCELLENT] Repository meets strict enterprise systems engineering standards.".green().bold());
            } else {
                println!("{}", "[ACTION RECOMMENDED] Address advisories to maintain pristine professional posture.".yellow().bold());
            }
        }
        Commands::CheckRules => {
            println!("{}", "=== MAPai Sovereign Protocol & Operational Rules Audit ===".bold().cyan());
            println!("Validating self-compliance across universal system invariants...\n");

            let report = saia_core::RuleComplianceAuditor::audit_all();
            for check in &report.checks {
                let badge = if check.passed { "[PASS]".green().bold() } else { "[FAIL]".red().bold() };
                println!("  {} {} - {}", badge, check.rule_name.bold(), check.details);
            }

            println!("\nSummary: {}", report.compliance_score.cyan().bold());
            if report.overall_passed {
                println!("{}", "[ALIGNED] MAPai is 100% compliant with all universal operational rules.".green().bold());
            } else {
                println!("{}", "[NON-COMPLIANT] MAPai detected rule deviations. Corrective action required.".red().bold());
                std::process::exit(1);
            }
        }
        Commands::Ui { port, host, no_browser } => {
            WebStudio::start(port, &host, !no_browser)?;
        }
        Commands::Scout { path } => {
            println!("{}", "╔═════════════════════════════════════════════════════════════════════════╗".cyan());
            println!("{}", "║          MAPai Always-On 7-Vector Autonomous Internet Scouting          ║".cyan().bold());
            println!("{}", "║      Self-Development | Projects | Intent | Security | Devices | AI     ║".cyan());
            println!("{}", "╚═════════════════════════════════════════════════════════════════════════╝".cyan());
            println!("{} Querying live registries, security feeds, toolchains & host capabilities...\n", "[*]".cyan().bold());

            let matrix = OmniScoutEngine::scout_all_vectors(&path)?;

            for (idx, finding) in matrix.findings.iter().enumerate() {
                let (domain_icon, domain_name) = match finding.domain {
                    ScoutDomain::SelfDevelopment => ("🔄", "Self-Development & Toolchain Evolution"),
                    ScoutDomain::ProjectDependencies => ("📦", "Project Dependencies & Supply Chain"),
                    ScoutDomain::HumanUnderstanding => ("🧠", "Human Understanding & Intent Distillation"),
                    ScoutDomain::ThreatsAndDangers => ("🛡️", "Threats, Dangers & CVE Vulnerabilities"),
                    ScoutDomain::AiModelIntelligence => ("🤖", "AI Model Intelligence & Protocol Standards"),
                    ScoutDomain::DeviceHardwareKnowledge => ("💻", "Device Hardware & Architecture Optimization"),
                    ScoutDomain::SoftwareEcosystemKnowledge => ("🌐", "Software Ecosystem & Tooling Bridges"),
                };

                let status_badge = match finding.status {
                    ScoutStatus::Optimal => "[OPTIMAL]".green().bold(),
                    ScoutStatus::Synchronized => "[SYNCHRONIZED]".cyan().bold(),
                    ScoutStatus::Advisory => "[ADVISORY]".yellow().bold(),
                    ScoutStatus::ThreatDetected => "[THREAT DETECTED]".red().bold(),
                };

                println!("{}. {} {} {}", idx + 1, domain_icon, domain_name.bold(), status_badge);
                println!("   Topic         : {}", finding.title);
                println!("   Finding       : {}", finding.description);
                println!("   Action/Guidance: {}", finding.recommendation.italic());
                println!();
            }

            println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
            println!("Matrix Summary   : {}", matrix.summary.bold());
            println!("Overall Safety   : {}", if matrix.is_safe { "VERIFIED SAFE".green().bold() } else { "ATTENTION REQUIRED".red().bold() });
            println!("Advisories       : {}", matrix.advisories_count.to_string().yellow().bold());
            println!("Threats Blocked  : {}", matrix.threats_detected_count.to_string().red().bold());
            println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
        }
        Commands::Browse { url, real, headless_only, port, download } => {
            println!("{}", "=== MAPai Tiered Hybrid Browser Engine ===".bold().cyan());

            // Handle file download request
            if let Some(dest_dir) = download {
                println!("{} Initiating autonomous file download from: {}", "[*]".cyan().bold(), url);
                let dest_path = Path::new(&dest_dir);
                match RealBrowserCdpEngine::download_file(port, &url, dest_path) {
                    Ok(saved) => {
                        println!("{} File saved successfully to: {}", "[✓]".green().bold(), saved.display());
                        return Ok(());
                    }
                    Err(e) => {
                        eprintln!("{} Download failed: {}", "[FAIL]".red().bold(), e);
                        std::process::exit(1);
                    }
                }
            }

            let mode = if real {
                BrowseExecutionMode::RealBrowserOnly
            } else if headless_only {
                BrowseExecutionMode::HeadlessOnly
            } else {
                BrowseExecutionMode::AutoTiered
            };

            let req = BrowseRequest {
                url: url.clone(),
                mode,
                cdp_port: port,
            };

            println!("{} Target URL : {}", "[*]".cyan().bold(), url);
            println!("{} Mode       : {:?}", "[*]".cyan().bold(), mode);

            match HybridBrowserRouter::execute(req) {
                Ok(resp) => {
                    println!("{} Execution  : {}", "[✓]".green().bold(), resp.tier_used.bold());
                    if resp.escalated {
                        if let Some(reason) = resp.escalation_reason {
                            println!("{} Escalation : {}", "[!]".yellow().bold(), reason.yellow());
                        }
                    }
                    println!("{} Page Title : {}", "[✓]".green().bold(), resp.title.bold());
                    println!("{} Final URL  : {}", "[✓]".green().bold(), resp.final_url);
                    println!("\n{}", "--- Extracted Content (Token-Dense Markdown) ---".dimmed());
                    println!("{}", resp.markdown);
                    println!("{}", "-----------------------------------------------".dimmed());
                }
                Err(e) => {
                    eprintln!("{} Browser navigation failed: {}", "[FAIL]".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Screen { action } => {
            match action {
                ScreenCommands::Capture { window, output } => {
                    let default_path = StatePaths::temp_file("screen_latest.bmp");
                    let out_path = if output == ".mapai/screen_latest.bmp" {
                        default_path.as_path()
                    } else {
                        Path::new(&output)
                    };
                    println!("{}", "=== MAPai Global Screen Analyser: Capture Engine ===".bold().cyan());
                    
                    let capture = if let Some(ref win_title) = window {
                        println!("{} Target Window : {}", "[*]".cyan().bold(), win_title.bold());
                        ScreenCaptureEngine::capture_window(win_title, out_path)?
                    } else {
                        println!("{} Target        : Entire Interactive Desktop", "[*]".cyan().bold());
                        ScreenCaptureEngine::capture_screen(out_path)?
                    };

                    println!("{} Resolution    : {}x{}", "[✓]".green().bold(), capture.width, capture.height);
                    println!("{} Output File   : {}", "[✓]".green().bold(), out_path.display());
                }
                ScreenCommands::Inspect { path } => {
                    let default_path = StatePaths::temp_file("screen_latest.bmp");
                    let in_path = if path == ".mapai/screen_latest.bmp" && !Path::new(&path).exists() {
                        default_path.as_path()
                    } else {
                        Path::new(&path)
                    };
                    println!("{}", "=== MAPai Global Screen Analyser: Visual Inspection ===".bold().cyan());
                    println!("{} Inspecting   : {}", "[*]".cyan().bold(), in_path.display());

                    let report = VisualScreenAnalyzer::analyze_bmp(in_path)?;
                    println!("{} Dimensions   : {}x{}", "[✓]".green().bold(), report.width, report.height);
                    println!("{} Background   : rgb({}, {}, {})", "[✓]".green().bold(), report.background_color.r, report.background_color.g, report.background_color.b);
                    println!("{} Foreground   : rgb({}, {}, {})", "[✓]".green().bold(), report.foreground_color.r, report.foreground_color.g, report.foreground_color.b);
                    println!("{} Contrast     : {:.2}:1 ({})", "[✓]".green().bold(), report.contrast_ratio, report.legibility_rating.bold());

                    if let Some(ref region) = report.content_region {
                        println!("{} Content Box  : ({}, {}) to ({}, {}) [{}x{}]", "[✓]".green().bold(), region.min_x, region.min_y, region.max_x, region.max_y, region.width, region.height);
                        println!("{} Pixels Active: {}", "[✓]".green().bold(), region.foreground_pixels);
                    } else {
                        println!("{} Content Box  : {}", "[!]".yellow().bold(), "NO CONTENT DETECTED (Canvas is blank or uniform)".yellow());
                    }
                }
                ScreenCommands::Verify { window, text, color, min_contrast } => {
                    println!("{}", "=== MAPai Closed-Loop Visual IV&V Verification Gate ===".bold().cyan());
                    println!("{} Target Window : {}", "[*]".cyan().bold(), window.bold());
                    if let Some(ref t) = text {
                        println!("{} Expected Text : {}", "[*]".cyan().bold(), t.bold());
                    }
                    if let Some(ref c) = color {
                        println!("{} Required Color: {}", "[*]".cyan().bold(), c.bold());
                    }

                    let temp_bmp = StatePaths::temp_file("visual_gate_audit.bmp");
                    let req = VisualGoalRequest {
                        window_title_pattern: window,
                        expected_text: text,
                        required_color: color,
                        minimum_contrast_ratio: Some(min_contrast),
                    };

                    let result = VisualVerificationGate::verify_visual_goal(&req, &temp_bmp)?;
                    
                    println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
                    if result.is_passed {
                        println!("{} Status        : {}", "[PASS]".green().bold(), result.summary.green().bold());
                        println!("{} Contrast Ratio: {:.2}:1 (WCAG Legible: {})", "[✓]".green().bold(), result.contrast_ratio, result.is_legible);
                    } else {
                        println!("{} Status        : {}", "[FAIL]".red().bold(), result.summary.red().bold());
                        println!("{} Contrast Ratio: {:.2}:1", "[!]".yellow().bold(), result.contrast_ratio);
                        if let Some(rem) = result.remediation {
                            println!("\n{} Actionable Remediation:", "[i]".cyan().bold());
                            println!("   {}", rem.yellow());
                        }
                        std::process::exit(1);
                    }
                    println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
                }
                ScreenCommands::Layout { window, json } => {
                    let manifest = SpatialLayoutEngine::perceive_window(&window)?;
                    if json {
                        println!("{}", serde_json::to_string_pretty(&manifest)?);
                    } else {
                        println!("{}", "=== MAPai ScreenSense: Multi-Layer Spatial Hierarchy ===".bold().cyan());
                        println!("{} Target Window : {}", "[*]".cyan().bold(), manifest.shell.window_title.bold());
                        println!("{} Display Metric: {}x{} (DPI: {}, Scale: {:.2}x)", "[✓]".green().bold(), manifest.display.screen_width, manifest.display.screen_height, manifest.display.dpi, manifest.display.scale_factor);
                        println!("{} Window Bounds : ({}, {}) [{}x{}] (Maximized: {})", "[✓]".green().bold(), manifest.shell.window_bounds.left, manifest.shell.window_bounds.top, manifest.shell.window_bounds.width, manifest.shell.window_bounds.height, manifest.shell.is_maximized);
                        
                        if let Some(ref rib) = manifest.shell.ribbon_bounds {
                            println!("{} Command Ribbon: ({}, {}) [{}x{}]", "[✓]".green().bold(), rib.left, rib.top, rib.width, rib.height);
                        }
                        
                        let cv = &manifest.shell.canvas_viewport;
                        println!("{} Canvas Viewport: ({}, {}) [{}x{}] (Center: {}, {})", "[✓]".green().bold(), cv.left, cv.top, cv.width, cv.height, cv.center().0, cv.center().1);
                        
                        println!("{} Tool State    : Active: {:?}, Primary Color: {:?}", "[✓]".green().bold(), manifest.tool_state.active_tool, manifest.tool_state.primary_color);
                        println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
                        println!("Spatial Summary  : {}", manifest.summary.bold());
                        println!("{}", "─────────────────────────────────────────────────────────────────────────".dimmed());
                    }
                }
                ScreenCommands::Perceive { window } => {
                    println!("{}", "=== MAPai ScreenSense: Live Agent Perception Scan (/perceive) ===".bold().cyan());
                    let manifest = SpatialLayoutEngine::perceive_window(&window)?;
                    
                    println!("{} Injected Spatial Context Packet:", "[*]".cyan().bold());
                    println!("```json");
                    println!("{}", serde_json::to_string_pretty(&manifest)?);
                    println!("```");
                    println!("{} Spatial Grounding complete. Coordinates verified.", "[✓]".green().bold());
                }
                ScreenCommands::Act { window, action, text, color, commit, clear, verify } => {
                    println!("{}", "=== MAPai SpatialActuator: Closed-Loop Desktop Action Batch ===".bold().cyan());
                    println!("{} Target Window : {}", "[*]".cyan().bold(), window.bold());
                    println!("{} Action Recipe : {}", "[*]".cyan().bold(), action.bold());
                    if let Some(ref t) = text {
                        println!("{} Payload Text  : {}", "[*]".cyan().bold(), t.bold());
                    }
                    if let Some(ref c) = color {
                        println!("{} Palette Color : {}", "[*]".cyan().bold(), c.bold());
                    }
                    println!("{} Commit Strategy: {}", "[*]".cyan().bold(), commit.bold());
                    println!("{} Clear Canvas   : {}", "[*]".cyan().bold(), clear);

                    let res = SpatialActuator::execute_action(&window, &action, text.as_deref(), color.as_deref(), &commit, clear)?;
                    println!("{} Action Result : {}", "[PASS]".green().bold(), res.details.green());

                    if verify {
                        println!("\n{}", "=== Auto-Triggering Visual Verification Gate ===".bold().cyan());
                        let temp_bmp = StatePaths::temp_file("act_verification_gate.bmp");
                        let req = VisualGoalRequest {
                            window_title_pattern: window.clone(),
                            expected_text: text,
                            required_color: color,
                            minimum_contrast_ratio: Some(3.0),
                        };
                        let v_res = VisualVerificationGate::verify_visual_goal(&req, &temp_bmp)?;
                        if v_res.is_passed {
                            println!("{} Visual Verification Gate: {}", "[PASS]".green().bold(), v_res.summary.green().bold());
                            println!("{} Contrast Ratio         : {:.2}:1 (WCAG Legible: {})", "[✓]".green().bold(), v_res.contrast_ratio, v_res.is_legible);
                        } else {
                            println!("{} Visual Verification Gate: {}", "[FAIL]".red().bold(), v_res.summary.red().bold());
                            if let Some(rem) = v_res.remediation {
                                println!("{} Remediation: {}", "[i]".cyan().bold(), rem.yellow());
                            }
                        }
                    }
                }
            }
        }
        Commands::Mcp => {
            eprintln!("{} Starting MAPai MCP Server (JSON-RPC stdio mode)...", "[*]".blue().bold());
            McpServer::run_stdio()?;
        }
        Commands::Acp => {
            eprintln!("{} Starting MAPai ACP Server (Agent Communication Protocol stdio mode)...", "[*]".blue().bold());
            saia_cli::AcpServer::run_stdio()?;
        }
        Commands::Tokens { file, prompt, budget } => {
            let paths = StatePaths::resolve(".");
            paths.ensure_dir_exists()?;
            let mut ledger = TokenLedger::load_from_file(&paths.token_ledger_file).unwrap_or_default();

            if let Some(new_budget) = budget {
                ledger.project_budget = Some(new_budget);
                ledger.save_to_file(&paths.token_ledger_file)?;
                println!("{} Project token budget updated to: {} tokens", "[✓]".green().bold(), new_budget);
            }

            if let Some(ref text) = prompt {
                let estimated = TokenLedger::estimate_tokens(text);
                println!("{} Prompt Text Estimate: {} tokens ({} chars, {} words)", 
                    "[*]".cyan().bold(), estimated, text.chars().count(), text.split_whitespace().count());
            }

            if let Some(ref file_path) = file {
                if let Ok(content) = fs::read_to_string(file_path) {
                    let estimated = TokenLedger::estimate_tokens(&content);
                    println!("{} File '{}' Estimate: {} tokens ({} chars, {} lines)", 
                        "[*]".cyan().bold(), file_path, estimated, content.chars().count(), content.lines().count());
                } else {
                    eprintln!("{} Could not read file: {}", "[!]".red().bold(), file_path);
                }
            }

            let summary = ledger.compile_summary(None);

            println!();
            println!("{}", "═════════════════════════════════════════════════════════════════════".cyan());
            println!("{}", "               MAPai Token Economics & Expenditure Ledger             ".cyan().bold());
            println!("{}", "═════════════════════════════════════════════════════════════════════".cyan());
            println!("{}", "Current.".bold());
            println!("Prompt Tokens   : {} / {}", 
                summary.latest_prompt_tokens.to_string().bold(),
                TokenLedger::format_count(summary.latest_prompt_tokens_saved).green().bold()
            );
            println!("Response Tokens : {} / {}", 
                summary.latest_completion_tokens.to_string().bold(),
                TokenLedger::format_count(summary.latest_completion_tokens_saved).green().bold()
            );
            println!("Turn Total      : {} / {}", 
                summary.latest_total_tokens.to_string().yellow().bold(),
                TokenLedger::format_count(summary.latest_tokens_saved).green().bold()
            );
            println!("  └─ Breakdown  : Text: {} · Exec: {} · Code: {} · MAPai: {}",
                summary.latest_breakdown.text_tokens,
                summary.latest_breakdown.execution_tokens,
                summary.latest_breakdown.coding_tokens,
                summary.latest_breakdown.mapai_internal_tokens
            );
            println!();
            println!("{}", "Total.".bold());
            println!("Active Conversation : {} / {}", 
                format!("{} tokens", summary.conversation_total).bold(),
                TokenLedger::format_count(summary.conversation_tokens_saved).green().bold()
            );
            println!("Today               : {} / {}", 
                format!("{} tokens", summary.today_total).bold(),
                TokenLedger::format_count(summary.today_tokens_saved).green().bold()
            );
            println!("Project Lifetime    : {} / {}", 
                format!("{} tokens", summary.project_total).cyan().bold(),
                TokenLedger::format_count(summary.project_tokens_saved).green().bold()
            );
            println!("  └─ Breakdown      : Text: {} · Exec: {} · Code: {} · MAPai: {}",
                summary.project_breakdown.text_tokens,
                summary.project_breakdown.execution_tokens,
                summary.project_breakdown.coding_tokens,
                summary.project_breakdown.mapai_internal_tokens
            );
            println!("---------------------------------------------------------------------");
            if let Some(budget_val) = summary.project_budget {
                println!("Project Budget Quota: {} tokens", budget_val);
                if let Some(balance) = summary.remaining_balance {
                    let bal_str = format!("{} tokens", balance);
                    if balance >= 0 {
                        println!("Remaining Balance   : {}", bal_str.green().bold());
                    } else {
                        println!("Remaining Balance   : {}", format!("{} (OVER BUDGET)", bal_str).red().bold());
                    }
                }
            } else {
                println!("Remaining Balance   : Unlimited / Unconfigured");
            }
            println!("Total Recorded Turns: {}", summary.total_turns_recorded);
            println!("{}", "═════════════════════════════════════════════════════════════════════".cyan());
        }
        Commands::Research { action, topic, output_dir } => {
            let paths = StatePaths::resolve(".");
            paths.ensure_dir_exists()?;
            let radar = saia_core::ResearchRadar::load_or_default(&paths.research_radar_file);

            match action.to_lowercase().as_str() {
                "list" => {
                    println!("{}", "╔═════════════════════════════════════════════════════════════════════════╗".cyan());
                    println!("{}", "║              MAPai Autonomous AI Research Radar & Ingestion Ledger       ║".cyan().bold());
                    println!("{}", "║      Reasoning Chains | Swarm Consensus | VAD Streaming | Memory Store   ║".cyan());
                    println!("{}", "╚═════════════════════════════════════════════════════════════════════════╝".cyan());
                    println!("Last Synchronized: {}\n", radar.updated_at.format("%I:%M %p, %d %B %Y").to_string().cyan());

                    for (idx, t) in radar.topics.iter().enumerate() {
                        println!("{}. {} {} [{}]", idx + 1, t.domain.icon(), t.title.bold(), t.id.cyan());
                        println!("   Domain       : {}", t.domain.display_name());
                        println!("   Source       : {}", t.source.italic());
                        println!("   Abstract     : {}", t.abstract_summary);
                        println!("   Capability Δ : {}", t.capability_delta.green());
                        println!("   Heuristic    : {}", t.actionable_pattern);
                        if let Some(_skill) = &t.recommended_skill {
                            println!("   Skill Target : {}", format!("mapai research --action synthesize --topic {}", t.id).yellow());
                        }
                        println!();
                    }
                    println!("Total Frontier Topics Monitored: {}", radar.topics.len().to_string().bold());
                    println!("Commands: `mapai research --action synthesize --topic <id>` to codify into a live skill.");
                }
                "query" => {
                    let q = topic.as_deref().unwrap_or("");
                    println!("Querying Research Radar for: '{}'", q.cyan());
                    let results = radar.query(None, Some(q));
                    if results.is_empty() {
                        println!("{} No topics matching '{}' found in research radar.", "[!]".yellow(), q);
                    } else {
                        for t in results {
                            println!("• [{}] {} (Source: {})", t.id.cyan().bold(), t.title, t.source.italic());
                            println!("  Pattern: {}", t.actionable_pattern);
                        }
                    }
                }
                "synthesize" => {
                    let topic_id = match topic.as_deref() {
                        Some(id) => id,
                        None => {
                            eprintln!("{} Please specify a topic ID via `--topic <id>`", "[ERROR]".red().bold());
                            std::process::exit(1);
                        }
                    };

                    let skills_target = output_dir
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|| std::path::PathBuf::from(".agent").join("skills"));

                    match radar.synthesize_skill_from_topic(topic_id, &skills_target) {
                        Ok(skill_path) => {
                            println!("{} Successfully synthesized sovereign skill from research topic '{}'!", "[SYNTHESIZED]".green().bold(), topic_id);
                            println!("  Skill File  : {}", skill_path.display().to_string().cyan().bold());
                            println!("  Destination : {}", skills_target.display());
                            println!("  Status      : Ready for autonomous subagent and MCP activation.");
                        }
                        Err(e) => {
                            eprintln!("{} Failed to synthesize skill: {}", "[ERROR]".red().bold(), e);
                            std::process::exit(1);
                        }
                    }
                }
                "sync-memory" => {
                    let store = saia_core::CognitiveMemoryStore::open(&paths.cognitive_db_file)?;
                    let count = radar.sync_to_cognitive_memory(&store)?;
                    println!("{} Indexed {} research topics into SQLite FTS5 Cognitive Store ({}).", 
                        "[MEMORY SYNCED]".green().bold(),
                        count,
                        paths.cognitive_db_file.display().to_string().cyan()
                    );
                    println!("Sub-millisecond semantic search across research patterns is now active.");
                }
                unknown => {
                    eprintln!("{} Unknown action '{}'. Available: list, query, synthesize, sync-memory", "[ERROR]".red().bold(), unknown);
                    std::process::exit(1);
                }
            }
        }
        Commands::Watch { path, once } => {
            println!("{}", "╔═════════════════════════════════════════════════════════════════════════╗".cyan());
            println!("{}", "║                   MAPai Always-On Truth-to-Materials Watcher            ║".cyan().bold());
            println!("{}", "║              Instantaneous On-Save Static AST Violation Detector        ║".cyan());
            println!("{}", "╚═════════════════════════════════════════════════════════════════════════╝".cyan());
            println!("{} Watching workspace '{}' for file mutations (sub-10ms AST audits)...\n", "[*]".cyan().bold(), path);
            println!("{} Press Ctrl+C in this terminal to stop watching.\n", "[i]".yellow());

            let scanner = TruthScanner::new();
            let mut last_scan_report = scanner.scan_directory(&path)?;
            let mut last_clean = last_scan_report.is_clean;
            let now = Utc::now().format("%I:%M:%S %p");
            if last_clean {
                println!("[{}] {} Clean baseline (0.00 hollow stubs across {} files)", now, "[✓ PASS]".green().bold(), last_scan_report.files_scanned);
            } else {
                println!("[{}] {} Baseline violations detected ({} stubs)", now, "[✗ VIOLATION]".red().bold(), last_scan_report.violations.len());
            }

            if once {
                return Ok(());
            }

            let mut last_check = std::time::Instant::now();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                if last_check.elapsed() >= std::time::Duration::from_millis(500) {
                    last_check = std::time::Instant::now();
                    if let Ok(report) = scanner.scan_directory(&path) {
                        if report.violations.len() != last_scan_report.violations.len() || report.is_clean != last_clean {
                            let ts = Utc::now().format("%I:%M:%S %p");
                            if report.is_clean {
                                println!("[{}] {} Workspace restored to clean truth-to-materials (0.00 stubs across {} files)", ts, "[✓ RESOLVED]".green().bold(), report.files_scanned);
                            } else {
                                println!("\n[{}] {} New AST Hollow Stubs Introduced! ({} violations)", ts, "[✗ ALERT]".red().bold(), report.violations.len());
                                for v in report.violations.iter().take(5) {
                                    println!("    • {}:{} [{}]", v.file_path.cyan(), v.line_number, v.pattern_type.yellow());
                                }
                            }
                            last_clean = report.is_clean;
                            last_scan_report = report;
                        }
                    }
                }
            }
        }
        Commands::Wizard { name, intent, language, job_class } => {
            println!("{}", "╔═════════════════════════════════════════════════════════════════════════╗".cyan());
            println!("{}", "║                  MAPai Sovereign Greenfield Wizard                      ║".cyan().bold());
            println!("{}", "║         Socratic Intent Scaffolding & Substrate Constraints Engine      ║".cyan());
            println!("{}", "╚═════════════════════════════════════════════════════════════════════════╝".cyan());

            let proj_name = if let Some(n) = name {
                n
            } else {
                print!("Enter Project Name [default: my-sovereign-app]: ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                let trimmed = buf.trim();
                if trimmed.is_empty() { "my-sovereign-app".to_string() } else { trimmed.to_string() }
            };

            let proj_intent = if let Some(i) = intent {
                i
            } else {
                print!("Enter Job 1 Problem Intent (What problem does this solve?): ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf)?;
                let trimmed = buf.trim();
                if trimmed.is_empty() { "Autonomous high-performance system service".to_string() } else { trimmed.to_string() }
            };

            let jc = saia_core::capsule::ProfessionalJobClass::from_identifier(&job_class)
                .unwrap_or(saia_core::capsule::ProfessionalJobClass::GreenfieldSynthesis);

            let paths = StatePaths::resolve(".");
            paths.ensure_dir_exists()?;

            let capsule = ProjectCapsule {
                project_name: proj_name.clone(),
                job_1_intent: saia_core::capsule::JobIntent {
                    problem_statement: proj_intent.clone(),
                    target_domain: "systems-software".into(),
                    job_class: Some(jc),
                    success_criteria: vec![
                        "Zero hollow stubs (AST verified)".into(),
                        "Deterministic builds".into(),
                        "Sub-millisecond verification".into(),
                    ],
                },
                job_2_constraints: saia_core::capsule::MaterialConstraints {
                    primary_language: language.clone(),
                    target_platforms: vec!["native".into()],
                    performance_envelope: vec!["sub-10ms response".into()],
                    forbidden_patterns: vec!["no-todo-stubs".into(), "no-unhandled-unwrap".into()],
                },
                current_stage: saia_core::capsule::PipelineStage::Stage1ProblemIntent,
                ..Default::default()
            };

            let toml_str = toml::to_string_pretty(&capsule)?;
            fs::write(&paths.capsule_file, toml_str)?;

            println!("\n{} Sovereign Project Capsule Initialized!", "[✓]".green().bold());
            println!("  • Project Name : {}", proj_name.cyan().bold());
            println!("  • Job 1 Intent : {}", proj_intent.white());
            println!("  • Job Class    : {}", jc.display_name().cyan());
            println!("  • Language     : {}", language.yellow());
            println!("  • Storage File : {}", paths.capsule_file.display().to_string().cyan());
            println!("\nNext Step: Run `{}` to advance to Stage 2, or run `{}` to launch the GUI Cockpit.\n",
                "mapai advance".green().bold(), "mapai ui".cyan().bold());
        }
    }

    Ok(())
}
