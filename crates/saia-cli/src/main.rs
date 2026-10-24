use clap::{Parser, Subcommand};
use colored::*;
use saia_core::capsule::ProjectCapsule;
use saia_verify::TruthScanner;
use saia_orchestrator::mediator::HumanVerificationMediator;
use saia_cli::mcp_server::McpServer;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "mapai")]
#[command(author = "MAPai & SAiA Contributors")]
#[command(version = "0.1.0")]
#[command(about = "MAPai: Sovereign Reference Implementation of the SAiA Protocol", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a sovereign .mapai working environment in the current directory
    Init {
        #[arg(short, long, default_value = "unnamed-project")]
        name: String,
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
    /// Launch the Model Context Protocol (MCP) server for Cursor, Claude Desktop, and Antigravity
    Mcp,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            let mapai_dir = Path::new(".mapai");
            if !mapai_dir.exists() {
                fs::create_dir_all(mapai_dir)?;
            }

            let capsule_path = mapai_dir.join("capsule.toml");
            if !capsule_path.exists() {
                let mut capsule = ProjectCapsule::default();
                capsule.project_name = name;
                let toml_str = toml::to_string_pretty(&capsule)?;
                fs::write(&capsule_path, toml_str)?;
                println!("{} Sovereign SAiA project capsule initialized at .mapai/capsule.toml", "[+]".green().bold());
            } else {
                println!("{} Project capsule already exists at .mapai/capsule.toml", "[!]".yellow().bold());
            }
        }
        Commands::Check { path } => {
            println!("{}", "=== MAPai Truth-to-Materials Verification Gate ===".bold());
            let scanner = TruthScanner::new();
            let report = scanner.scan_directory(&path)?;

            println!("Files audited : {}", report.files_scanned);
            println!("Lines audited : {}", report.lines_scanned);
            println!("Hollow Rate   : {:.2} stubs/KLOC", report.hollow_completion_rate);

            if report.is_clean {
                println!("\n{} All deliverables satisfy Truth-to-Materials (Zero Hollow Stubs).", "[PASS]".green().bold());
                std::process::exit(0);
            } else {
                println!("\n{} Hollow stubs or unverified placeholders detected:", "[FAIL]".red().bold());
                for v in report.violations {
                    println!("  - {}:{} [{}] {}", v.file_path.cyan(), v.line_number, v.pattern_type.red(), v.matched_text);
                }
                std::process::exit(1);
            }
        }
        Commands::Context => {
            let capsule_path = Path::new(".mapai/capsule.toml");
            if capsule_path.exists() {
                let content = fs::read_to_string(capsule_path)?;
                let capsule: ProjectCapsule = toml::from_str(&content)?;
                
                println!("--- MAPai COGNITIVE CONTEXT PACKET (SAiA v1) ---");
                println!("Project     : {}", capsule.project_name);
                println!("Active Stage: {:?}", capsule.current_stage);
                println!("Job 1 Goal  : {}", capsule.job_1_intent.problem_statement);
                println!("Job 2 Bounds: Language: {}, Constraints: {:?}", capsule.job_2_constraints.primary_language, capsule.job_2_constraints.performance_envelope);
                println!("Forbidden   : {:?}", capsule.job_2_constraints.forbidden_patterns);
                println!("------------------------------------------------");
            } else {
                eprintln!("{} No .mapai/capsule.toml found. Run `mapai init` first.", "[ERROR]".red().bold());
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
        Commands::Mcp => {
            eprintln!("{} Starting MAPai MCP Server (JSON-RPC stdio mode)...", "[*]".blue().bold());
            McpServer::run_stdio()?;
        }
    }

    Ok(())
}
