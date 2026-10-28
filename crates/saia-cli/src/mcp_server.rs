use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use std::fs;
use std::path::Path;
use saia_verify::TruthScanner;
use saia_core::capsule::ProjectCapsule;
use saia_core::cawo::{ContinuityJournal, CawoCheckpoint, GateStatus};
use saia_core::chrono::Utc;
use saia_core::protocol_standard::MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION;

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

pub struct McpServer;

impl McpServer {
    /// Builds a formatted sovereign badge reflecting live MAPai sidecar state
    pub fn generate_badge() -> String {
        let capsule_path = Path::new(".mapai/capsule.toml");
        let journal_path = Path::new(".mapai/journal.json");

        let (project, stage, ver_str) = if capsule_path.exists() {
            if let Ok(c) = fs::read_to_string(capsule_path) {
                if let Ok(capsule) = toml::from_str::<ProjectCapsule>(&c) {
                    let v = match capsule.check_version_compatibility() {
                        saia_core::capsule::VersionStatus::UpToDate => "Aligned".to_string(),
                        saia_core::capsule::VersionStatus::OutdatedCapsule { .. } => "Update Pending".to_string(),
                        saia_core::capsule::VersionStatus::NewerCapsule { .. } => "Newer Capsule".to_string(),
                    };
                    (capsule.project_name, format!("{:?}", capsule.current_stage), v)
                } else {
                    ("MAPai".into(), "Unknown".into(), "Unknown".into())
                }
            } else {
                ("MAPai".into(), "Uninitialized".into(), "Uninitialized".into())
            }
        } else {
            ("MAPai".into(), "Uninitialized".into(), "Uninitialized".into())
        };

        let scanner = TruthScanner::new();
        let (audit_str, hcr) = if let Ok(report) = scanner.scan_directory(".") {
            if report.is_clean {
                ("VERIFIED (Zero Stubs)".to_string(), report.hollow_completion_rate)
            } else {
                (format!("FAILED ({} stubs)", report.violations.len()), report.hollow_completion_rate)
            }
        } else {
            ("Unchecked".into(), 0.0)
        };

        let last_sha = if let Ok(journal) = ContinuityJournal::load_from_file(journal_path) {
            journal.last_valid_checkpoint()
                .map(|c| if c.codebase_ast_hash.len() >= 12 { c.codebase_ast_hash[..12].to_string() } else { c.codebase_ast_hash.clone() })
                .unwrap_or_else(|| "none".into())
        } else {
            "none".into()
        };

        format!(
            "🛡️ [MAPai-SAiA ACTIVE SOVEREIGN SIDECAR]\n  • Project: {}\n  • Stage: {}\n  • Version Status: {}\n  • Truth-to-Materials: {} (HCR: {:.2}/KLOC)\n  • Anchored Tree SHA: {}\n  • Model Protocol: Epistemic Dissent Gated | Zero Telemetry",
            project, stage, ver_str, audit_str, hcr, last_sha
        )
    }

    /// Runs a compliant Model Context Protocol (MCP) JSON-RPC loop over standard input/output
    pub fn run_stdio() -> anyhow::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
                let response = match req.method.as_str() {
                    "initialize" => {
                        let result = serde_json::json!({
                            "protocolVersion": "2024-11-05",
                            "capabilities": {
                                "tools": {},
                                "resources": {}
                            },
                            "serverInfo": {
                                "name": "mapai-saia",
                                "version": env!("CARGO_PKG_VERSION")
                            }
                        });
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(result),
                            error: None,
                        }
                    }
                    "resources/list" => {
                        let resources = serde_json::json!({
                            "resources": [
                                {
                                    "uri": "mapai://status/badge",
                                    "name": "MAPai Sidecar Active Badge",
                                    "mimeType": "text/plain",
                                    "description": "Visual sovereign badge proving MAPai sidecar is active over vanilla AI"
                                },
                                {
                                    "uri": "mapai://protocol/instructions",
                                    "name": "Universal MAPai Operational Protocol",
                                    "mimeType": "text/markdown",
                                    "description": "Mandatory response header, 12-hour AM/PM timestamp, SemVer, and sovereign identity"
                                }
                            ]
                        });
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(resources),
                            error: None,
                        }
                    }
                    "resources/read" => {
                        let uri = req.params.as_ref()
                            .and_then(|p| p.get("uri"))
                            .and_then(|u| u.as_str())
                            .unwrap_or("mapai://status/badge");

                        let text_content = if uri == "mapai://protocol/instructions" {
                            MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION.to_string()
                        } else {
                            Self::generate_badge()
                        };

                        let result = serde_json::json!({
                            "contents": [
                                {
                                    "uri": uri,
                                    "mimeType": "text/plain",
                                    "text": text_content
                                }
                            ]
                        });
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(result),
                            error: None,
                        }
                    }
                    "tools/list" => {
                        let tools = serde_json::json!({
                            "tools": [
                                {
                                    "name": "get_active_badge",
                                    "description": "Returns the visual sovereign badge confirming MAPai is actively supervising the session rather than vanilla AI",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {}
                                    }
                                },
                                {
                                    "name": "verify_deliverables",
                                    "description": "Performs deterministic AST and stub audit ensuring Truth-to-Materials (Zero Hollow Stubs)",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "path": {
                                                "type": "string",
                                                "description": "Directory to scan (default: .)"
                                            }
                                        }
                                    }
                                },
                                {
                                    "name": "get_cawo_context",
                                    "description": "Retrieves the compact token-dense cognitive context packet (< 1,000 tokens) with active non-negotiables",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {}
                                    }
                                },
                                {
                                    "name": "get_version_status",
                                    "description": "Inspects MAPai protocol version alignment and reports any required schema migrations",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {}
                                    }
                                },
                                {
                                    "name": "record_cawo_checkpoint",
                                    "description": "Records an immutable CAWO state checkpoint anchored to the repository tree SHA-256",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "intent": {
                                                "type": "string",
                                                "description": "Formal description of changes"
                                            },
                                            "rationale": {
                                                "type": "string",
                                                "description": "Architectural justification"
                                            }
                                        },
                                        "required": ["intent"]
                                    }
                                },
                                {
                                    "name": "get_protocol_instructions",
                                    "description": "Returns the mandatory MAPai operational protocol instructions, 12-hour AM/PM header format, and sovereign persona",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {}
                                    }
                                }
                            ]
                        });
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(tools),
                            error: None,
                        }
                    }
                    "tools/call" => {
                        let tool_name = req.params.as_ref()
                            .and_then(|p| p.get("name"))
                            .and_then(|n| n.as_str())
                            .unwrap_or("");

                        let response_text = match tool_name {
                            "get_protocol_instructions" => MAPAI_UNIVERSAL_SYSTEM_INSTRUCTION.to_string(),
                            "get_active_badge" => Self::generate_badge(),
                            "verify_deliverables" => {
                                let scan_path = req.params.as_ref()
                                    .and_then(|p| p.get("arguments"))
                                    .and_then(|a| a.get("path"))
                                    .and_then(|p| p.as_str())
                                    .unwrap_or(".");
                                let scanner = TruthScanner::new();
                                match scanner.scan_directory(scan_path) {
                                    Ok(report) => format!(
                                        "MAPai Audit: is_clean={}, violations={}, hollow_rate={:.2} stubs/KLOC",
                                        report.is_clean, report.violations.len(), report.hollow_completion_rate
                                    ),
                                    Err(e) => format!("Audit error: {}", e),
                                }
                            }
                            "get_cawo_context" => {
                                let capsule_path = Path::new(".mapai/capsule.toml");
                                if capsule_path.exists() {
                                    fs::read_to_string(capsule_path).unwrap_or_else(|_| "Error reading capsule".into())
                                } else {
                                    "No capsule.toml found. Run mapai init.".into()
                                }
                            }
                            "get_version_status" => {
                                match saia_orchestrator::version_adapter::VersionAdaptiveEngine::inspect_project(".") {
                                    Ok(report) => format!(
                                        "MAPai Version Status: status={:?}, migration_required={}, changes={:?}, actions={:?}",
                                        report.status, report.migration_required, report.changes_summary, report.prescribed_actions
                                    ),
                                    Err(e) => format!("Version inspection error: {}", e),
                                }
                            }
                            "record_cawo_checkpoint" => {
                                let intent = req.params.as_ref()
                                    .and_then(|p| p.get("arguments"))
                                    .and_then(|a| a.get("intent"))
                                    .and_then(|i| i.as_str())
                                    .unwrap_or("Autonomous checkpoint");
                                let rationale = req.params.as_ref()
                                    .and_then(|p| p.get("arguments"))
                                    .and_then(|a| a.get("rationale"))
                                    .and_then(|r| r.as_str())
                                    .map(|s| s.to_string());

                                let journal_path = Path::new(".mapai/journal.json");
                                match ContinuityJournal::load_from_file(journal_path) {
                                    Ok(mut journal) => {
                                        let ast_hash = ContinuityJournal::compute_workspace_tree_hash(".").unwrap_or_else(|_| "unknown".into());
                                        let parent_step_id = journal.checkpoints.last().map(|c| c.step_id);
                                        let next_step_id = parent_step_id.unwrap_or(0) + 1;
                                        let cp = CawoCheckpoint {
                                            step_id: next_step_id,
                                            parent_step_id,
                                            timestamp: Utc::now(),
                                            stage: saia_core::capsule::PipelineStage::Stage4Implementation,
                                            intent: intent.to_string(),
                                            codebase_ast_hash: ast_hash.clone(),
                                            reality_status: GateStatus::Passed,
                                            rationale,
                                        };
                                        journal.append(cp);
                                        let _ = journal.save_to_file(journal_path);
                                        format!("Checkpoint recorded: Step #{} with SHA {}", next_step_id, &ast_hash[..12])
                                    }
                                    Err(e) => format!("Failed to record checkpoint: {}", e),
                                }
                            }
                            _ => format!("Unknown tool: {}", tool_name),
                        };

                        let result = serde_json::json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": response_text
                                }
                            ]
                        });
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: Some(result),
                            error: None,
                        }
                    }
                    _ => {
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: req.id,
                            result: None,
                            error: Some(serde_json::json!({
                                "code": -32601,
                                "message": "Method not found"
                            })),
                        }
                    }
                };

                let resp_str = serde_json::to_string(&response)?;
                writeln!(stdout, "{}", resp_str)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }
}
