use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};
use saia_verify::TruthScanner;

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
                                "tools": {}
                            },
                            "serverInfo": {
                                "name": "mapai-saia",
                                "version": "0.1.0"
                            }
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
                                    "name": "verify_deliverables",
                                    "description": "Performs deterministic AST and stub audit ensuring Truth-to-Materials",
                                    "inputSchema": {
                                        "type": "object",
                                        "properties": {
                                            "path": {
                                                "type": "string",
                                                "description": "Directory to scan (default: .)"
                                            }
                                        }
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
                        let scanner = TruthScanner::new();
                        let report = scanner.scan_directory(".")?;
                        let result = serde_json::json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": format!("Audit Result: is_clean={}, violations={}, hollow_rate={:.2} stubs/KLOC", 
                                        report.is_clean, report.violations.len(), report.hollow_completion_rate)
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
