//! Auto-session restore and init file sourcing.
//!
//! Handles automatic session restoration on startup
//! and sourcing of init configuration files.

use std::path::{Path, PathBuf};

/// Auto-session restore configuration.
#[derive(Debug, Clone)]
pub struct AutoSessionConfig {
    /// Whether to auto-restore last session.
    pub auto_restore: bool,
    /// Whether to auto-save session on quit.
    pub auto_save_on_quit: bool,
    /// Session directory path.
    pub session_dir: PathBuf,
    /// Current session file name.
    pub current_session: Option<String>,
    /// Directories to exclude from auto-session.
    pub exclude_dirs: Vec<PathBuf>,
}

impl AutoSessionConfig {
    pub fn new() -> Self {
        let session_dir = dirs_hint()
            .map(|d| d.join("sessions"))
            .unwrap_or_else(|| PathBuf::from("/tmp/kjxlkj/sessions"));
        Self {
            auto_restore: false,
            auto_save_on_quit: false,
            session_dir,
            current_session: None,
            exclude_dirs: Vec::new(),
        }
    }

    /// Get the session file for a working directory.
    pub fn session_file_for(&self, cwd: &Path) -> PathBuf {
        let hash = simple_hash(
            &cwd.to_string_lossy(),
        );
        self.session_dir.join(format!("{}.json", hash))
    }

    /// Whether a directory is excluded.
    pub fn is_excluded(&self, dir: &Path) -> bool {
        self.exclude_dirs.iter().any(|e| dir.starts_with(e))
    }

    /// List available sessions.
    pub fn list_sessions(&self) -> Vec<String> {
        if !self.session_dir.exists() {
            return Vec::new();
        }
        std::fs::read_dir(&self.session_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "json")
                    .unwrap_or(false)
            })
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .collect()
    }
}

impl Default for AutoSessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Init file sourcing order and state.
#[derive(Debug, Clone, Default)]
pub struct InitFileState {
    /// Init files to source in order.
    pub init_files: Vec<PathBuf>,
    /// Files already sourced.
    pub sourced: Vec<PathBuf>,
    /// Whether init file sourcing is complete.
    pub complete: bool,
}

impl InitFileState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Discover init files in standard locations.
    pub fn discover(&mut self) {
        self.init_files.clear();
        // XDG config
        if let Some(config_dir) = dirs_hint() {
            let init = config_dir.join("init.vim");
            if init.exists() {
                self.init_files.push(init);
            }
            let init_lua = config_dir.join("init.lua");
            if init_lua.exists() {
                self.init_files.push(init_lua);
            }
        }
        // Legacy location
        let home_rc = home_dir_hint()
            .map(|h| h.join(".kjxlkjrc"));
        if let Some(rc) = home_rc {
            if rc.exists() {
                self.init_files.push(rc);
            }
        }
    }

    /// Get the next init file to source.
    pub fn next_file(&mut self) -> Option<PathBuf> {
        for f in &self.init_files {
            if !self.sourced.contains(f) {
                let path = f.clone();
                self.sourced.push(path.clone());
                return Some(path);
            }
        }
        self.complete = true;
        None
    }

    /// Whether all init files have been sourced.
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

/// Simple string hash for session file naming.
fn simple_hash(s: &str) -> String {
    let mut hash: u64 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    format!("{:016x}", hash)
}

/// Hint for config directory.
fn dirs_hint() -> Option<PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(|d| PathBuf::from(d).join("kjxlkj"))
        .or_else(|| {
            home_dir_hint()
                .map(|h| h.join(".config").join("kjxlkj"))
        })
}

/// Hint for home directory.
fn home_dir_hint() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_file_deterministic() {
        let cfg = AutoSessionConfig::new();
        let f1 = cfg.session_file_for(Path::new("/tmp/project"));
        let f2 = cfg.session_file_for(Path::new("/tmp/project"));
        assert_eq!(f1, f2);
    }

    #[test]
    fn exclude_dir() {
        let mut cfg = AutoSessionConfig::new();
        cfg.exclude_dirs
            .push(PathBuf::from("/tmp"));
        assert!(cfg.is_excluded(Path::new("/tmp/foo")));
        assert!(!cfg.is_excluded(Path::new("/home/foo")));
    }

    #[test]
    fn init_file_state() {
        let mut state = InitFileState::new();
        assert!(!state.is_complete());
        assert!(state.next_file().is_none()); // no files
        assert!(state.is_complete());
    }

    #[test]
    fn simple_hash_consistency() {
        let h1 = simple_hash("test");
        let h2 = simple_hash("test");
        assert_eq!(h1, h2);
        let h3 = simple_hash("other");
        assert_ne!(h1, h3);
    }
}
