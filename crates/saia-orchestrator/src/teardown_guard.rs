use std::process::Child;
use std::time::{Duration, Instant};
use std::io::Write;
use anyhow::{Context, Result};

/// Represents the termination outcome of a managed process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeardownOutcome {
    /// Exited cleanly upon EOF transmission to stdin (Tier 1)
    GracefulEof { status_code: Option<i32> },
    /// Settled within the SIGTERM/grace window (Tier 2)
    GracefulTermination { elapsed_ms: u64 },
    /// Required hard kill to prevent zombie process leakage (Tier 3)
    ForceKilled { elapsed_ms: u64 },
}

/// The 3-Tier Deterministic Process Teardown Manager.
/// SLA Lifecycle:
/// 1. Tier 1 (Graceful): Closes stdin / transmits EOF.
/// 2. Tier 2 (Grace Window): Waits for in-flight buffers to flush cleanly.
/// 3. Tier 3 (Force Teardown): Sends hard kill signal and reaps exit state.
pub struct ProcessTeardownGuard {
    pub grace_period: Duration,
}

impl Default for ProcessTeardownGuard {
    fn default() -> Self {
        Self {
            grace_period: Duration::from_millis(500),
        }
    }
}

impl ProcessTeardownGuard {
    pub fn new(grace_period: Duration) -> Self {
        Self { grace_period }
    }

    /// Executes the 3-Tier teardown lifecycle on a running child process.
    pub fn teardown(&self, mut child: Child) -> Result<TeardownOutcome> {
        let start = Instant::now();

        // Tier 1: Graceful EOF via stdin close
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.flush();
            drop(stdin); // Drops and closes stdin handle
        }

        // Check if process settled immediately on EOF
        if let Some(status) = child.try_wait().context("Failed to poll child status")? {
            return Ok(TeardownOutcome::GracefulEof {
                status_code: status.code(),
            });
        }

        // Tier 2: Wait within the bounded grace period
        let poll_interval = Duration::from_millis(25);
        while start.elapsed() < self.grace_period {
            if let Some(_status) = child.try_wait().context("Failed to poll child status")? {
                return Ok(TeardownOutcome::GracefulTermination {
                    elapsed_ms: start.elapsed().as_millis() as u64,
                });
            }
            std::thread::sleep(poll_interval);
        }

        // Tier 3: Hard kill fallback to guarantee zero zombie leakage
        let _ = child.kill();
        let _ = child.wait(); // Reap exit state to avoid defunct zombie entry

        Ok(TeardownOutcome::ForceKilled {
            elapsed_ms: start.elapsed().as_millis() as u64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_tier_1_graceful_eof_on_fast_process() {
        let guard = ProcessTeardownGuard::default();
        let child = if cfg!(windows) {
            Command::new("cmd")
                .args(["/c", "exit 0"])
                .spawn()
                .unwrap()
        } else {
            Command::new("sh")
                .args(["-c", "exit 0"])
                .spawn()
                .unwrap()
        };

        let outcome = guard.teardown(child).unwrap();
        match outcome {
            TeardownOutcome::GracefulEof { status_code } => {
                assert_eq!(status_code, Some(0));
            }
            TeardownOutcome::GracefulTermination { .. } => {}
            TeardownOutcome::ForceKilled { .. } => {
                panic!("Fast exit process should not require force kill");
            }
        }
    }

    #[test]
    fn test_tier_3_force_kills_hanging_process() {
        let guard = ProcessTeardownGuard::new(Duration::from_millis(100));
        let child = if cfg!(windows) {
            Command::new("powershell")
                .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
                .spawn()
                .unwrap()
        } else {
            Command::new("sh")
                .args(["-c", "sleep 30"])
                .spawn()
                .unwrap()
        };

        let outcome = guard.teardown(child).unwrap();
        match outcome {
            TeardownOutcome::ForceKilled { elapsed_ms } => {
                assert!(elapsed_ms >= 100);
            }
            _ => panic!("Hanging process must reach Tier 3 ForceKilled outcome"),
        }
    }
}
