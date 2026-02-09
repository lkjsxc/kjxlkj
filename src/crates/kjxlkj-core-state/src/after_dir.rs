//! After-directory sourcing: load user overrides from
//! `after/` directories in runtimepath.

use std::path::{Path, PathBuf};

/// After-directory configuration state.
#[derive(Debug, Clone, Default)]
pub struct AfterDirConfig {
    /// Runtime paths to search for after/ directories.
    pub runtime_paths: Vec<PathBuf>,
    /// Whether after-directory sourcing is enabled.
    pub enabled: bool,
    /// Loaded after-files (for dedup).
    pub loaded_files: Vec<PathBuf>,
}

impl AfterDirConfig {
    pub fn new() -> Self {
        Self {
            runtime_paths: Vec::new(),
            enabled: true,
            loaded_files: Vec::new(),
        }
    }

    /// Add a runtime path.
    pub fn add_path(&mut self, path: PathBuf) {
        if !self.runtime_paths.contains(&path) {
            self.runtime_paths.push(path);
        }
    }

    /// Find after-directory files for a filetype.
    pub fn find_after_files(
        &self,
        filetype: &str,
    ) -> Vec<PathBuf> {
        if !self.enabled {
            return Vec::new();
        }
        let mut results = Vec::new();
        for rtp in &self.runtime_paths {
            // after/ftplugin/{filetype}.vim
            let ftplugin = rtp
                .join("after")
                .join("ftplugin")
                .join(format!("{}.vim", filetype));
            if ftplugin.exists() {
                results.push(ftplugin);
            }
            // after/syntax/{filetype}.vim
            let syntax = rtp
                .join("after")
                .join("syntax")
                .join(format!("{}.vim", filetype));
            if syntax.exists() {
                results.push(syntax);
            }
            // after/indent/{filetype}.vim
            let indent = rtp
                .join("after")
                .join("indent")
                .join(format!("{}.vim", filetype));
            if indent.exists() {
                results.push(indent);
            }
        }
        results
    }

    /// Build after-directory search paths from a base config dir.
    pub fn from_config_dir(config_dir: &Path) -> Self {
        let mut cfg = Self::new();
        cfg.add_path(config_dir.to_path_buf());
        // Also check XDG data dirs
        if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            cfg.add_path(
                PathBuf::from(data_home).join("kjxlkj"),
            );
        }
        cfg
    }

    /// Mark a file as loaded.
    pub fn mark_loaded(&mut self, path: PathBuf) {
        if !self.loaded_files.contains(&path) {
            self.loaded_files.push(path);
        }
    }

    /// Check if a file has already been loaded.
    pub fn is_loaded(&self, path: &Path) -> bool {
        self.loaded_files.iter().any(|p| p == path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_check_paths() {
        let mut cfg = AfterDirConfig::new();
        cfg.add_path(PathBuf::from("/home/user/.config/kjxlkj"));
        assert_eq!(cfg.runtime_paths.len(), 1);
        cfg.add_path(PathBuf::from("/home/user/.config/kjxlkj"));
        assert_eq!(cfg.runtime_paths.len(), 1); // no dup
    }

    #[test]
    fn mark_loaded() {
        let mut cfg = AfterDirConfig::new();
        let p = PathBuf::from("/tmp/after/ftplugin/rust.vim");
        assert!(!cfg.is_loaded(&p));
        cfg.mark_loaded(p.clone());
        assert!(cfg.is_loaded(&p));
    }

    #[test]
    fn disabled_returns_empty() {
        let mut cfg = AfterDirConfig::new();
        cfg.enabled = false;
        let files = cfg.find_after_files("rust");
        assert!(files.is_empty());
    }
}
