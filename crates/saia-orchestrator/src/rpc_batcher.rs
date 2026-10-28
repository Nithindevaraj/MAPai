use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/// Represents a single tool call invocation inside a batched script execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCallInvocation {
    pub call_id: String,
    pub tool_name: String,
    pub parameters: HashMap<String, String>,
}

/// Execution outcome for a single tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolExecutionResult {
    pub call_id: String,
    pub is_success: bool,
    pub output: String,
    pub duration_ms: u64,
}

/// Summary report returned after executing an RPC tool batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcBatchSummary {
    pub batch_id: String,
    pub total_tools_executed: usize,
    pub successful_tools: usize,
    pub failed_tools: usize,
    /// Round trips to the LLM saved by executing locally in one batch
    pub llm_round_trips_saved: usize,
    pub results: Vec<ToolExecutionResult>,
    pub total_elapsed_ms: u64,
}

/// The local RPC batch dispatcher that collapses multi-turn tool loops
/// into a single fast execution turn.
#[derive(Default)]
pub struct RpcBatchDispatcher {
    /// Handlers registered for simulated or native tool executions
    tool_handlers: HashMap<String, Box<dyn Fn(&HashMap<String, String>) -> Result<String, String> + Send + Sync>>,
}

impl RpcBatchDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an execution handler for a named tool.
    pub fn register_handler<F>(&mut self, tool_name: &str, handler: F)
    where
        F: Fn(&HashMap<String, String>) -> Result<String, String> + Send + Sync + 'static,
    {
        self.tool_handlers.insert(tool_name.to_string(), Box::new(handler));
    }

    /// Executes an entire batch of tool calls atomically in a single pass.
    pub fn execute_batch(&self, batch_id: &str, invocations: &[ToolCallInvocation]) -> RpcBatchSummary {
        let start_time = Instant::now();
        let mut results = Vec::new();
        let mut success_count = 0;
        let mut fail_count = 0;

        for inv in invocations {
            let tool_start = Instant::now();
            let (success, out) = if let Some(handler) = self.tool_handlers.get(&inv.tool_name) {
                match handler(&inv.parameters) {
                    Ok(res) => (true, res),
                    Err(err) => (false, err),
                }
            } else {
                (false, format!("Unrecognized tool '{}' in batch", inv.tool_name))
            };

            let duration_ms = tool_start.elapsed().as_millis() as u64;
            if success {
                success_count += 1;
            } else {
                fail_count += 1;
            }

            results.push(ToolExecutionResult {
                call_id: inv.call_id.clone(),
                is_success: success,
                output: out,
                duration_ms,
            });
        }

        let total = invocations.len();
        // If 5 tools ran in 1 batch, we saved (5 - 1) = 4 full LLM round trips
        let round_trips_saved = if total > 1 { total - 1 } else { 0 };

        RpcBatchSummary {
            batch_id: batch_id.to_string(),
            total_tools_executed: total,
            successful_tools: success_count,
            failed_tools: fail_count,
            llm_round_trips_saved: round_trips_saved,
            results,
            total_elapsed_ms: start_time.elapsed().as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_tool_collapsing_and_round_trip_savings() {
        let mut dispatcher = RpcBatchDispatcher::new();

        // Register sample tool handlers
        dispatcher.register_handler("read_file", |params| {
            let path = params.get("path").cloned().unwrap_or_default();
            Ok(format!("Contents of {}", path))
        });

        dispatcher.register_handler("grep_search", |params| {
            let pattern = params.get("pattern").cloned().unwrap_or_default();
            Ok(format!("Matches for '{}'", pattern))
        });

        // Batch 4 sequential tool invocations
        let mut params1 = HashMap::new();
        params1.insert("path".into(), "Cargo.toml".into());

        let mut params2 = HashMap::new();
        params2.insert("pattern".into(), "serde".into());

        let mut params3 = HashMap::new();
        params3.insert("path".into(), "src/lib.rs".into());

        let mut params4 = HashMap::new();
        params4.insert("pattern".into(), "tokio".into());

        let invocations = vec![
            ToolCallInvocation {
                call_id: "call_1".into(),
                tool_name: "read_file".into(),
                parameters: params1,
            },
            ToolCallInvocation {
                call_id: "call_2".into(),
                tool_name: "grep_search".into(),
                parameters: params2,
            },
            ToolCallInvocation {
                call_id: "call_3".into(),
                tool_name: "read_file".into(),
                parameters: params3,
            },
            ToolCallInvocation {
                call_id: "call_4".into(),
                tool_name: "grep_search".into(),
                parameters: params4,
            },
        ];

        let summary = dispatcher.execute_batch("batch_test_01", &invocations);

        assert_eq!(summary.total_tools_executed, 4);
        assert_eq!(summary.successful_tools, 4);
        assert_eq!(summary.failed_tools, 0);
        // 4 tools collapsed into 1 turn = 3 LLM round trips saved!
        assert_eq!(summary.llm_round_trips_saved, 3);
        assert_eq!(summary.results.len(), 4);
    }
}
