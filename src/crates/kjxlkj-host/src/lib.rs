//! Terminal host and lifecycle management for kjxlkj editor.
//!
//! This crate manages the terminal lifecycle and event loop.

mod event_loop;

pub use event_loop::EventLoop;

use anyhow::Result;
use crossterm::terminal;
use kjxlkj_core_state::EditorState;
use std::path::PathBuf;

/// Host configuration.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HostConfig {
    /// Initial file to open.
    pub file: Option<PathBuf>,
    /// Initial content (if no file).
    pub content: Option<String>,
}


/// Terminal host.
pub struct Host {
    config: HostConfig,
}

impl Host {
    /// Create a new host.
    pub fn new(config: HostConfig) -> Self {
        Self { config }
    }

    /// Run the editor.
    pub fn run(self) -> Result<()> {
        let mut state = EditorState::new();

        // Load initial content
        if let Some(path) = &self.config.file {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                state.load_file(path.clone(), &content);
            } else {
                state.load_file(path.clone(), "");
            }
        } else if let Some(content) = &self.config.content {
            state.load_content(content);
        }

        // Get initial terminal size
        let (width, height) = terminal::size()?;
        state.resize(width, height);

        // Create event loop
        let mut event_loop = EventLoop::new(state)?;
        event_loop.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_config_default() {
        let config = HostConfig::default();
        assert!(config.file.is_none());
        assert!(config.content.is_none());
    }

    #[test]
    fn test_host_config_with_file() {
        let config = HostConfig {
            file: Some(PathBuf::from("/tmp/test.txt")),
            content: None,
        };
        assert!(config.file.is_some());
        assert!(config.content.is_none());
    }

    #[test]
    fn test_host_config_with_content() {
        let config = HostConfig {
            file: None,
            content: Some("hello world".to_string()),
        };
        assert!(config.file.is_none());
        assert!(config.content.is_some());
    }

    #[test]
    fn test_host_config_clone() {
        let config = HostConfig {
            file: Some(PathBuf::from("/tmp/test.txt")),
            content: Some("content".to_string()),
        };
        let cloned = config.clone();
        assert_eq!(config.file, cloned.file);
        assert_eq!(config.content, cloned.content);
    }

    #[test]
    fn test_host_config_debug() {
        let config = HostConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("HostConfig"));
    }

    #[test]
    fn test_host_new() {
        let config = HostConfig::default();
        let _host = Host::new(config);
        // Host is created successfully
    }

    #[test]
    fn test_host_config_both_file_and_content() {
        let config = HostConfig {
            file: Some(PathBuf::from("/tmp/test.txt")),
            content: Some("hello".to_string()),
        };
        // Both can be set (file takes priority in run())
        assert!(config.file.is_some());
        assert!(config.content.is_some());
    }

    #[test]
    fn test_host_config_empty_content() {
        let config = HostConfig {
            file: None,
            content: Some(String::new()),
        };
        assert_eq!(config.content, Some(String::new()));
    }

    #[test]
    fn test_host_config_path_extensions() {
        let config = HostConfig {
            file: Some(PathBuf::from("/home/user/code/file.rs")),
            content: None,
        };
        assert!(config.file.as_ref().unwrap().to_string_lossy().ends_with(".rs"));
    }

    #[test]
    fn test_host_config_relative_path() {
        let config = HostConfig {
            file: Some(PathBuf::from("relative/path/file.txt")),
            content: None,
        };
        assert!(config.file.is_some());
    }

    #[test]
    fn test_host_config_content_with_newlines() {
        let config = HostConfig {
            file: None,
            content: Some("line1\nline2\nline3".to_string()),
        };
        assert!(config.content.as_ref().unwrap().contains('\n'));
    }

    #[test]
    fn test_host_config_unicode_content() {
        let config = HostConfig {
            file: None,
            content: Some("你好世界 🌍".to_string()),
        };
        assert!(config.content.is_some());
    }

    #[test]
    fn test_host_config_windows_path() {
        let config = HostConfig {
            file: Some(PathBuf::from("C:\\Users\\test\\file.txt")),
            content: None,
        };
        assert!(config.file.is_some());
    }

    #[test]
    fn test_host_config_long_content() {
        let config = HostConfig {
            file: None,
            content: Some("x".repeat(10000)),
        };
        assert_eq!(config.content.as_ref().unwrap().len(), 10000);
    }

    #[test]
    fn test_host_with_file_config() {
        let config = HostConfig {
            file: Some(PathBuf::from("test.rs")),
            content: None,
        };
        let host = Host::new(config);
        assert!(host.config.file.is_some());
    }

    #[test]
    fn test_host_with_content_config() {
        let config = HostConfig {
            file: None,
            content: Some("hello".to_string()),
        };
        let host = Host::new(config);
        assert!(host.config.content.is_some());
    }

    #[test]
    fn test_host_config_path_with_spaces() {
        let config = HostConfig {
            file: Some(PathBuf::from("/home/user/my project/file name.txt")),
            content: None,
        };
        assert!(config.file.is_some());
    }

    #[test]
    fn test_host_config_hidden_file() {
        let config = HostConfig {
            file: Some(PathBuf::from("/home/user/.config/.hidden")),
            content: None,
        };
        assert!(config.file.is_some());
    }

    #[test]
    fn test_host_config_content_tabs() {
        let config = HostConfig {
            file: None,
            content: Some("col1\tcol2\tcol3".to_string()),
        };
        assert!(config.content.as_ref().unwrap().contains('\t'));
    }

    #[test]
    fn test_host_config_symlink_path() {
        let config = HostConfig {
            file: Some(PathBuf::from("/var/run/../log/test.log")),
            content: None,
        };
        assert!(config.file.is_some());
    }

    #[test]
    fn test_host_config_no_extension() {
        let config = HostConfig {
            file: Some(PathBuf::from("/usr/bin/bash")),
            content: None,
        };
        assert!(config.file.is_some());
    }
}
