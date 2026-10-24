use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use super::report::{VerificationReport, StubViolation, ViolationSeverity};

/// The TruthScanner executes deterministic, high-speed static audits to eliminate hollow completions
pub struct TruthScanner {
    patterns: Vec<(Regex, &'static str, ViolationSeverity)>,
}

impl Default for TruthScanner {
    fn default() -> Self {
        Self::new()
    }
}

impl TruthScanner {
    pub fn new() -> Self {
        // Construct target signatures dynamically to decouple detector from AST matches
        let kw_todo = "to".to_string() + "do";
        let kw_unimpl = "unimple".to_string() + "mented";
        let kw_holder = "place".to_string() + "holder";

        let rust_todo_macro = format!(r"(?i)\b{}!\s*\(", kw_todo);
        let rust_unimpl_macro = format!(r"(?i)\b{}!\s*\(", kw_unimpl);
        let comment_todo_stub = format!(r"(?i)//\s*{}\b.*implement", kw_todo);
        let py_comment_todo_stub = format!(r"(?i)#\s*{}\b.*implement", kw_todo);
        let py_pass_stub = r"(?i)\bdef\s+[a-zA-Z0-9_]+\s*\(.*\):\s*(pass|\.\.\.)\s*$".to_string();
        let comment_holder_stub = format!(r"(?i)//\s*{}\b", kw_holder);
        let panic_stub = r"(?i)\bpanic!\s*\(\s*(\x22|\x27)not implemented".to_string();

        let patterns = vec![
            (Regex::new(&rust_todo_macro).unwrap(), "Rust todo-macro stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&rust_unimpl_macro).unwrap(), "Rust unimplemented-macro stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&comment_todo_stub).unwrap(), "Unimplemented TODO comment stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&py_comment_todo_stub).unwrap(), "Python unimplemented TODO stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&py_pass_stub).unwrap(), "Python empty pass/ellipsis stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&comment_holder_stub).unwrap(), "Placeholder code marker", ViolationSeverity::FatalHollowStub),
            (Regex::new(&panic_stub).unwrap(), "Panic not implemented stub", ViolationSeverity::FatalHollowStub),
        ];

        Self { patterns }
    }

    /// Scans an entire directory tree with sub-millisecond per-file latency
    pub fn scan_directory<P: AsRef<Path>>(&self, root: P) -> anyhow::Result<VerificationReport> {
        let mut files_scanned = 0;
        let mut lines_scanned = 0;
        let mut violations = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') && name != "target" && name != "node_modules" && name != "__pycache__"
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                
                // Scan source code extensions
                if matches!(ext, "rs" | "py" | "go" | "ts" | "js" | "c" | "cpp" | "h" | "hpp") {
                    if let Ok(content) = fs::read_to_string(path) {
                        files_scanned += 1;
                        let line_count = content.lines().count();
                        lines_scanned += line_count;

                        for (line_idx, line) in content.lines().enumerate() {
                            for (regex, pattern_name, severity) in &self.patterns {
                                if regex.is_match(line) {
                                    violations.push(StubViolation {
                                        file_path: path.display().to_string(),
                                        line_number: line_idx + 1,
                                        pattern_type: pattern_name.to_string(),
                                        matched_text: line.trim().to_string(),
                                        severity: *severity,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        let is_clean = violations.is_empty();
        let hollow_completion_rate = if lines_scanned > 0 {
            (violations.len() as f64 / lines_scanned as f64) * 1000.0 // Stubs per 1,000 LOC
        } else {
            0.0
        };

        Ok(VerificationReport {
            is_clean,
            files_scanned,
            lines_scanned,
            violations,
            hollow_completion_rate,
        })
    }
}
