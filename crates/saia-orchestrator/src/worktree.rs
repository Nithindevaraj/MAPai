use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents an isolated ephemeral Git worktree for background subagent execution.
#[derive(Debug)]
pub struct EphemeralWorktree {
    pub subagent_id: String,
    pub path: PathBuf,
    pub original_branch: String,
}

/// Manager for creating and tearing down isolated Git worktrees.
pub struct WorktreeManager {
    worktrees_root: PathBuf,
}

impl WorktreeManager {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            worktrees_root: repo_root.as_ref().join(".mapai").join("worktrees"),
        }
    }

    /// Creates an isolated ephemeral worktree for a subagent so the developer's active
    /// IDE workspace is never dirtied or clobbered during long-running tasks.
    pub fn spawn_isolated_worktree(&self, subagent_id: &str) -> Result<EphemeralWorktree> {
        fs::create_dir_all(&self.worktrees_root).context("Failed to create worktrees directory")?;
        let target_dir = self.worktrees_root.join(subagent_id);

        if target_dir.exists() {
            let _ = Self::remove_worktree_dir(&target_dir);
        }

        // Run git worktree add --detach <target_dir> HEAD
        let output = Command::new("git")
            .args(["worktree", "add", "--detach", target_dir.to_str().unwrap(), "HEAD"])
            .output()
            .context("Failed to execute `git worktree add`")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Fallback for non-git workspaces: simply create directory mirror
            fs::create_dir_all(&target_dir)?;
            return Ok(EphemeralWorktree {
                subagent_id: subagent_id.to_string(),
                path: target_dir,
                original_branch: format!("fallback-detached-{}", stderr.trim()),
            });
        }

        Ok(EphemeralWorktree {
            subagent_id: subagent_id.to_string(),
            path: target_dir,
            original_branch: "HEAD".to_string(),
        })
    }

    /// Safely cleans up and tears down an ephemeral worktree.
    pub fn cleanup(&self, worktree: &EphemeralWorktree) -> Result<()> {
        let _ = Command::new("git")
            .args(["worktree", "remove", "--force", worktree.path.to_str().unwrap()])
            .output();

        if worktree.path.exists() {
            fs::remove_dir_all(&worktree.path).ok();
        }

        let _ = Command::new("git")
            .args(["worktree", "prune"])
            .output();

        Ok(())
    }

    /// Lists all currently active ephemeral subagent worktree directories.
    pub fn list_active_worktrees(&self) -> Vec<String> {
        let mut active = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.worktrees_root) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    active.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
        active
    }

    /// Prunes and cleans all orphaned ephemeral worktrees.
    pub fn prune_all_ephemeral(&self) -> Result<()> {
        for agent_id in self.list_active_worktrees() {
            let target_dir = self.worktrees_root.join(&agent_id);
            let _ = Self::remove_worktree_dir(&target_dir);
        }
        let _ = Command::new("git").args(["worktree", "prune"]).output();
        Ok(())
    }

    fn remove_worktree_dir(path: &Path) -> Result<()> {
        let _ = Command::new("git")
            .args(["worktree", "remove", "--force", path.to_str().unwrap()])
            .output();
        if path.exists() {
            fs::remove_dir_all(path).ok();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worktree_manager_path_resolution() {
        let mgr = WorktreeManager::new(".");
        assert!(mgr.worktrees_root.ends_with(".mapai\\worktrees") || mgr.worktrees_root.ends_with(".mapai/worktrees"));
    }

    #[test]
    fn test_worktree_listing_and_pruning() {
        let temp_dir = std::env::temp_dir().join("saia_wt_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mgr = WorktreeManager::new(&temp_dir);
        let mock_wt_dir = mgr.worktrees_root.join("subagent_alpha");
        fs::create_dir_all(&mock_wt_dir).unwrap();

        let active = mgr.list_active_worktrees();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], "subagent_alpha");

        mgr.prune_all_ephemeral().unwrap();
        assert!(mgr.list_active_worktrees().is_empty());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
