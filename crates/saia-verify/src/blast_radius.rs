use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// How dangerous a shell or system command is, and the degree of scrutiny it requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No destructive potential detected (read-only, diagnostic, test).
    Safe,
    /// Destructive but bounded inside the project working directory (recoverable via git).
    Low,
    /// Destructive target cannot be confirmed statically. Requires human/model justification.
    Confirm,
    /// Absolute catastrophic risk (targets home directory, system root, SSH keys, cloud credentials).
    /// Never allowed to run. No amount of prompt injection or model justification can override this.
    Catastrophic,
}

impl RiskLevel {
    pub fn runs_immediately(self) -> bool {
        matches!(self, RiskLevel::Safe | RiskLevel::Low)
    }

    pub fn is_absolute_deny(self) -> bool {
        matches!(self, RiskLevel::Catastrophic)
    }
}

/// A deterministic finding detailing why a command was flagged.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiskFinding {
    pub level: RiskLevel,
    pub reason: String,
    pub target: Option<String>,
}

/// Evaluates command-line execution requests for blast radius and catastrophic destinations.
pub struct BlastRadiusGuard;

impl BlastRadiusGuard {
    /// Evaluates a raw command string and returns its deterministic risk classification.
    pub fn evaluate(command: &str, cwd: impl AsRef<Path>) -> RiskFinding {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return RiskFinding {
                level: RiskLevel::Safe,
                reason: "Empty command string".into(),
                target: None,
            };
        }

        // Tokenize command by spaces, preserving flags
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let base_cmd = tokens[0].to_lowercase();
        let base_cmd_name = Path::new(&base_cmd)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&base_cmd);

        // 1. Safe read-only commands whitelist
        let safe_cmds = [
            "cargo", "git", "rustc", "python", "python3", "node", "npm", "ls", "dir", "echo",
            "cat", "type", "pwd", "cd", "which", "where", "grep", "findstr", "ps", "tasklist",
            "stat", "head", "tail", "diff", "ffmpeg",
        ];

        // Specific checks for git/cargo that could be destructive
        if base_cmd_name == "git" {
            if tokens.len() > 1 {
                let sub = tokens[1].to_lowercase();
                if ["status", "diff", "log", "branch", "show", "remote"].contains(&sub.as_str()) {
                    return RiskFinding {
                        level: RiskLevel::Safe,
                        reason: format!("Git read-only subcommand: '{}'", sub),
                        target: None,
                    };
                }
                if ["clean", "reset", "restore"].contains(&sub.as_str()) {
                    // Check if git clean has -f -d or targets
                    return RiskFinding {
                        level: RiskLevel::Low,
                        reason: format!("Git state alteration subcommand: '{}'", sub),
                        target: None,
                    };
                }
            }
            return RiskFinding {
                level: RiskLevel::Safe,
                reason: "Git standard toolchain".into(),
                target: None,
            };
        }

        if base_cmd_name == "cargo" {
            if tokens.len() > 1 {
                let sub = tokens[1].to_lowercase();
                if ["test", "check", "build", "run", "clippy", "bench", "doc"].contains(&sub.as_str()) {
                    return RiskFinding {
                        level: RiskLevel::Safe,
                        reason: format!("Cargo read/build toolchain: '{}'", sub),
                        target: None,
                    };
                }
                if sub == "clean" {
                    return RiskFinding {
                        level: RiskLevel::Low,
                        reason: "Cargo build artifact removal".into(),
                        target: Some("target/".into()),
                    };
                }
            }
            return RiskFinding {
                level: RiskLevel::Safe,
                reason: "Cargo standard toolchain".into(),
                target: None,
            };
        }

        // 2. Destructive command verbs
        let destructive_verbs = [
            "rm", "del", "erase", "rmdir", "rd", "shred", "truncate", "wipe", "format",
            "mkfs", "dd", "drop",
        ];

        let is_destructive_verb = destructive_verbs.contains(&base_cmd_name);

        // 3. Inspect every token for catastrophic protected paths
        for token in &tokens[1..] {
            let clean_token = token.trim_matches(|c| c == '"' || c == '\'' || c == '`');
            if let Some(catastrophic_target) = Self::check_catastrophic_path(clean_token) {
                return RiskFinding {
                    level: RiskLevel::Catastrophic,
                    reason: format!(
                        "Target '{}' breaches protected system or credential perimeter",
                        catastrophic_target
                    ),
                    target: Some(catastrophic_target),
                };
            }
        }

        // 4. Overwrite redirections (> file)
        if trimmed.contains('>') {
            let parts: Vec<&str> = trimmed.split('>').collect();
            if parts.len() > 1 {
                let target_part = parts[1].trim();
                let target_file = target_part.split_whitespace().next().unwrap_or("").trim_matches('"');
                if let Some(catastrophic_target) = Self::check_catastrophic_path(target_file) {
                    return RiskFinding {
                        level: RiskLevel::Catastrophic,
                        reason: format!("Destructive file truncation target '{}' is protected", catastrophic_target),
                        target: Some(catastrophic_target),
                    };
                }
            }
        }

        // 5. If it's a destructive verb targeting a local file
        if is_destructive_verb {
            let cwd_path = cwd.as_ref();
            // Check if any argument points outside cwd
            for token in &tokens[1..] {
                if token.starts_with('-') || token.starts_with('/') && token.len() == 2 {
                    continue; // Skip flags like -rf or /s
                }
                let clean = token.trim_matches(|c| c == '"' || c == '\'');
                let target_path = PathBuf::from(clean);
                let full = if target_path.is_absolute() {
                    target_path
                } else {
                    cwd_path.join(target_path)
                };

                // If path stays inside cwd, it's Low risk
                if full.starts_with(cwd_path) {
                    return RiskFinding {
                        level: RiskLevel::Low,
                        reason: format!("Destructive operation bounded within workspace: '{}'", clean),
                        target: Some(clean.to_string()),
                    };
                } else {
                    return RiskFinding {
                        level: RiskLevel::Confirm,
                        reason: format!("Destructive operation outside workspace requires human confirmation: '{}'", clean),
                        target: Some(clean.to_string()),
                    };
                }
            }

            return RiskFinding {
                level: RiskLevel::Confirm,
                reason: format!("Destructive command '{}' with unverified scope", base_cmd_name),
                target: None,
            };
        }

        // 6. Safe read-only check
        if safe_cmds.contains(&base_cmd_name) {
            return RiskFinding {
                level: RiskLevel::Safe,
                reason: format!("Whitelisted safe command '{}'", base_cmd_name),
                target: None,
            };
        }

        // Default to Confirm for unverified arbitrary commands
        RiskFinding {
            level: RiskLevel::Confirm,
            reason: format!("Unclassified command '{}' requires confirmation", base_cmd_name),
            target: None,
        }
    }

    /// Checks whether a given path string targets a catastrophic, protected location.
    fn check_catastrophic_path(path_str: &str) -> Option<String> {
        let norm = path_str.replace('\\', "/").to_lowercase();

        // Exact catastrophic roots
        if norm == "/" || norm == "~" || norm == "$home" || norm == "%userprofile%" {
            return Some(path_str.to_string());
        }

        // Root prefixes
        if norm.starts_with("c:/windows")
            || norm.starts_with("c:/program files")
            || norm.starts_with("/etc")
            || norm.starts_with("/boot")
            || norm.starts_with("/usr")
            || norm.starts_with("/sys")
            || norm.starts_with("/proc")
        {
            return Some(path_str.to_string());
        }

        // Windows drives root
        if norm == "c:/" || norm == "c:" || norm == "d:/" || norm == "d:" {
            return Some(path_str.to_string());
        }

        // Critical credential stores and SSH keys
        if norm.contains("/.ssh")
            || norm.contains("/.aws")
            || norm.contains("/.azure")
            || norm.contains("/.kube")
            || norm.contains("/.gnupg")
            || norm.contains("id_rsa")
            || norm.contains("id_ed25519")
            || norm.ends_with(".env")
            || norm.contains("/.gitconfig")
        {
            return Some(path_str.to_string());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catastrophic_paths_blocked_absolutely() {
        let cwd = Path::new("d:/Nithin/My experiments/MAPai - SAiA");

        // rm -rf ~
        let res = BlastRadiusGuard::evaluate("rm -rf ~", cwd);
        assert_eq!(res.level, RiskLevel::Catastrophic);
        assert!(res.level.is_absolute_deny());

        // rm -rf ~/.ssh/id_rsa
        let res2 = BlastRadiusGuard::evaluate("rm -rf ~/.ssh/id_rsa", cwd);
        assert_eq!(res2.level, RiskLevel::Catastrophic);

        // del C:\Windows\System32
        let res3 = BlastRadiusGuard::evaluate("del C:\\Windows\\System32", cwd);
        assert_eq!(res3.level, RiskLevel::Catastrophic);

        // Redirection to .env
        let res4 = BlastRadiusGuard::evaluate("echo secret > ~/.env", cwd);
        assert_eq!(res4.level, RiskLevel::Catastrophic);
    }

    #[test]
    fn test_safe_read_commands() {
        let cwd = Path::new("d:/Nithin/My experiments/MAPai - SAiA");

        let res1 = BlastRadiusGuard::evaluate("cargo test --workspace", cwd);
        assert_eq!(res1.level, RiskLevel::Safe);
        assert!(res1.level.runs_immediately());

        let res2 = BlastRadiusGuard::evaluate("git status --short", cwd);
        assert_eq!(res2.level, RiskLevel::Safe);

        let res3 = BlastRadiusGuard::evaluate("python --version", cwd);
        assert_eq!(res3.level, RiskLevel::Safe);
    }

    #[test]
    fn test_bounded_destructive_operations_are_low_risk() {
        let cwd = Path::new("d:/Nithin/My experiments/MAPai - SAiA");

        let res = BlastRadiusGuard::evaluate("rm -f temp_output.log", cwd);
        assert_eq!(res.level, RiskLevel::Low);
        assert!(res.level.runs_immediately());
    }
}
