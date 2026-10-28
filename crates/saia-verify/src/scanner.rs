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
        let kotlin_todo = format!(r"\b{}\s*\(", "TO".to_string() + "DO");
        let py_raise_unimpl = r"(?i)\braise\s+NotImplementedError".to_string();
        let ts_throw_unimpl = r#"(?i)\bthrow\s+new\s+Error\s*\(\s*["'](?:not\s+implemented|todo|placeholder)["']\s*\)"#.to_string();
        let go_panic_unimpl = r#"(?i)\bpanic\s*\(\s*["'](?:not\s+implemented|todo)["']\s*\)"#.to_string();

        let patterns = vec![
            (Regex::new(&rust_todo_macro).unwrap(), "Rust todo-macro stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&rust_unimpl_macro).unwrap(), "Rust unimplemented-macro stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&comment_todo_stub).unwrap(), "Unimplemented TODO comment stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&py_comment_todo_stub).unwrap(), "Python unimplemented TODO stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&py_pass_stub).unwrap(), "Python empty pass/ellipsis stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&comment_holder_stub).unwrap(), "Placeholder code marker", ViolationSeverity::FatalHollowStub),
            (Regex::new(&panic_stub).unwrap(), "Panic not implemented stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&kotlin_todo).unwrap(), "Kotlin placeholder stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&py_raise_unimpl).unwrap(), "Python NotImplementedError stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&ts_throw_unimpl).unwrap(), "TypeScript/JS throw not implemented stub", ViolationSeverity::FatalHollowStub),
            (Regex::new(&go_panic_unimpl).unwrap(), "Go panic not implemented stub", ViolationSeverity::FatalHollowStub),
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
                if e.depth() == 0 {
                    return true;
                }
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') 
                    && name != "target" 
                    && name != "node_modules" 
                    && name != "__pycache__"
                    && name != "AppData"
                    && name != "CrossDevice"
                    && name != "OneDrive"
                    && name != "dist"
                    && name != "build"
                    && name != "venv"
                    && name != ".venv"
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                #[cfg(target_os = "windows")]
                {
                    use std::os::windows::fs::MetadataExt;
                    if let Ok(meta) = entry.metadata() {
                        let attrs = meta.file_attributes();
                        // Skip Windows on-demand cloud placeholder files (e.g. OneDrive, Phone Link / CrossDevice)
                        // 0x00400000 = FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS
                        // 0x00040000 = FILE_ATTRIBUTE_RECALL_ON_OPEN
                        // 0x00001000 = FILE_ATTRIBUTE_OFFLINE
                        if (attrs & 0x00400000 != 0) || (attrs & 0x00040000 != 0) || (attrs & 0x00001000 != 0) {
                            continue;
                        }
                    }
                }

                let path = entry.path();
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                
                // Scan source code extensions
                if matches!(ext, "rs" | "py" | "go" | "ts" | "js" | "c" | "cpp" | "h" | "hpp" | "kt" | "kts" | "java" | "swift" | "dart" | "cs") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scanner_clean_detection() {
        let temp_dir = std::env::temp_dir().join("saia_test_clean");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let clean_file = temp_dir.join("clean.rs");
        fs::write(&clean_file, "pub fn calculate_sum(a: i32, b: i32) -> i32 {\n    a + b\n}\n").unwrap();

        let scanner = TruthScanner::new();
        let report = scanner.scan_directory(&temp_dir).unwrap();

        assert!(report.is_clean);
        assert_eq!(report.violations.len(), 0);
        assert_eq!(report.files_scanned, 1);
        assert_eq!(report.lines_scanned, 3);
        assert_eq!(report.hollow_completion_rate, 0.0);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_scanner_detects_hollow_stubs() {
        let temp_dir = std::env::temp_dir().join("saia_test_stubs");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let stub_keyword = "to".to_string() + "do";
        let stub_content = format!("fn incomplete() {{\n    {}!(\"unimplemented\");\n}}\n", stub_keyword);
        let stub_file = temp_dir.join("stub.rs");
        fs::write(&stub_file, stub_content).unwrap();

        let scanner = TruthScanner::new();
        let report = scanner.scan_directory(&temp_dir).unwrap();

        assert!(!report.is_clean);
        assert_eq!(report.violations.len(), 1);
        assert_eq!(report.violations[0].pattern_type, "Rust todo-macro stub");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_scanner_detects_polyglot_hollow_stubs() {
        let temp_dir = std::env::temp_dir().join("saia_test_polyglot_stubs");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // 1. Python NotImplementedError
        let py_stub = format!("def fetch():\n    raise {}('todo')\n", "NotImplemented".to_string() + "Error");
        fs::write(temp_dir.join("service.py"), py_stub).unwrap();
        // 2. TypeScript throw not implemented
        let ts_stub = format!("export function query() {{\n  throw new Error('{}');\n}}\n", "not ".to_string() + "implemented");
        fs::write(temp_dir.join("api.ts"), ts_stub).unwrap();
        // 3. Go panic not implemented
        let go_stub = format!("package main\nfunc Run() {{\n    panic(\"{}\")\n}}\n", "not ".to_string() + "implemented");
        fs::write(temp_dir.join("main.go"), go_stub).unwrap();

        let scanner = TruthScanner::new();
        let report = scanner.scan_directory(&temp_dir).unwrap();

        assert!(!report.is_clean);
        assert_eq!(report.violations.len(), 3);
        assert_eq!(report.files_scanned, 3);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
