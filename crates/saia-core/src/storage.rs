use std::path::{Path, PathBuf};
use sha2::{Digest, Sha256};
use std::fs;

/// Manages resolution and isolation of MAPai state storage.
/// In Zero-Pollution mode:
/// - If a local `.mapai` directory already exists in the project root, it is respected.
/// - Otherwise, all state (capsules, journals, heuristics) is stored centrally in
///   `~/.mapai/projects/<project_slug>-<hash>/`, keeping working project directories 100% clean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatePaths {
    pub dir: PathBuf,
    pub capsule_file: PathBuf,
    pub journal_file: PathBuf,
    pub token_ledger_file: PathBuf,
    pub settings_file: PathBuf,
    pub heuristics_dir: PathBuf,
    pub research_radar_file: PathBuf,
    pub cognitive_db_file: PathBuf,
    pub is_local: bool,
}

impl StatePaths {
    /// Resolves the storage location for a given workspace path.
    /// Prefers local `.mapai` only if it already exists, otherwise defaults to centralized shadow storage.
    pub fn resolve<P: AsRef<Path>>(workspace_root: P) -> Self {
        let ws = workspace_root.as_ref();
        let local_dir = ws.join(".mapai");
        if local_dir.is_dir() {
            return Self::from_dir(local_dir, true);
        }

        Self::shadow(ws)
    }

    /// Explicitly get the centralized shadow storage paths for a workspace.
    pub fn shadow<P: AsRef<Path>>(workspace_root: P) -> Self {
        let ws = workspace_root.as_ref();
        let canonical = ws.canonicalize().unwrap_or_else(|_| ws.to_path_buf());
        let norm_str = canonical.to_string_lossy().to_lowercase().replace('\\', "/");

        let mut hasher = Sha256::new();
        hasher.update(norm_str.as_bytes());
        let hash_hex = format!("{:x}", hasher.finalize());
        let short_hash = &hash_hex[..8];

        let folder_name = canonical
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project");
        let slug: String = folder_name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect();
        let project_key = format!("{}-{}", slug, short_hash);

        let home_dir = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());

        let shadow_dir = home_dir.join(".mapai").join("projects").join(project_key);
        Self::from_dir(shadow_dir, false)
    }

    /// Explicitly get the local storage paths (`<workspace>/.mapai`)
    pub fn local<P: AsRef<Path>>(workspace_root: P) -> Self {
        let local_dir = workspace_root.as_ref().join(".mapai");
        Self::from_dir(local_dir, true)
    }

    fn from_dir(dir: PathBuf, is_local: bool) -> Self {
        let capsule_file = dir.join("capsule.toml");
        let journal_file = dir.join("journal.json");
        let token_ledger_file = dir.join("token_ledger.json");
        let settings_file = dir.join("settings.json");
        let heuristics_dir = dir.join("heuristics");
        let research_radar_file = dir.join("research_radar.json");
        let cognitive_db_file = dir.join("cognitive_memory.db");

        Self {
            dir,
            capsule_file,
            journal_file,
            token_ledger_file,
            settings_file,
            heuristics_dir,
            research_radar_file,
            cognitive_db_file,
            is_local,
        }
    }

    /// Ensures the state directory and heuristics subdirectory exist.
    pub fn ensure_dir_exists(&self) -> std::io::Result<()> {
        if !self.dir.exists() {
            fs::create_dir_all(&self.dir)?;
        }
        if !self.heuristics_dir.exists() {
            fs::create_dir_all(&self.heuristics_dir)?;
        }
        Ok(())
    }

    /// Returns a temporary file path inside the OS temp directory
    pub fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("mapai_{}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_resolution() {
        let temp_dir = std::env::temp_dir().join("mapai_test_clean_workspace");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // Workspace has no .mapai, so resolve() must return shadow storage
        let paths = StatePaths::resolve(&temp_dir);
        assert!(!paths.is_local);
        assert!(paths.dir.to_string_lossy().contains(".mapai"));
        assert!(paths.dir.to_string_lossy().contains("projects"));

        // If .mapai directory is created locally, resolve() must return local
        let local_mapai = temp_dir.join(".mapai");
        fs::create_dir_all(&local_mapai).unwrap();
        let local_paths = StatePaths::resolve(&temp_dir);
        assert!(local_paths.is_local);
        assert_eq!(local_paths.dir, local_mapai);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
