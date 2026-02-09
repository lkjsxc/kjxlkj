//! Session save/load for persisting editor state.

use std::path::{Path, PathBuf};

/// Session data: open buffers, cursor positions, window layout, marks.
#[derive(Debug, Clone, Default)]
pub struct SessionData {
    /// Open file paths with cursor positions.
    pub files: Vec<SessionFile>,
    /// Current working directory.
    pub cwd: Option<PathBuf>,
    /// Window layout descriptor.
    pub layout: SessionLayout,
    /// Global marks.
    pub marks: Vec<(char, usize, usize)>,
    /// Current active buffer index.
    pub active_buffer: usize,
}

/// A file entry in a session.
#[derive(Debug, Clone)]
pub struct SessionFile {
    /// Absolute file path.
    pub path: PathBuf,
    /// Cursor line (0-indexed).
    pub cursor_line: usize,
    /// Cursor column (0-indexed).
    pub cursor_col: usize,
    /// Whether the file was modified when saved.
    pub was_modified: bool,
}

/// Window layout serialization.
#[derive(Debug, Clone, Default)]
pub enum SessionLayout {
    /// Single window.
    #[default]
    Single,
    /// Horizontal split with proportions.
    Hsplit(Vec<SessionLayout>, Vec<f64>),
    /// Vertical split with proportions.
    Vsplit(Vec<SessionLayout>, Vec<f64>),
}

/// Session manager for save/load/list operations.
#[derive(Debug, Clone)]
pub struct SessionManager {
    session_dir: PathBuf,
    current_name: Option<String>,
}

impl SessionManager {
    pub fn new(session_dir: PathBuf) -> Self {
        Self {
            session_dir,
            current_name: None,
        }
    }

    /// Serialize session data to a string format.
    #[rustfmt::skip]
    pub fn serialize(data: &SessionData) -> String {
        let mut out = String::from("# Session file\n");
        if let Some(ref cwd) = data.cwd { out.push_str(&format!("cwd {}\n", cwd.display())); }
        out.push_str(&format!("active {}\n", data.active_buffer));
        for file in &data.files {
            let m = if file.was_modified { "m" } else { "-" };
            out.push_str(&format!("file {} {} {} {m}\n", file.path.display(), file.cursor_line, file.cursor_col));
        }
        for &(name, line, col) in &data.marks { out.push_str(&format!("mark {name} {line} {col}\n")); }
        match &data.layout {
            SessionLayout::Single => out.push_str("layout single\n"),
            SessionLayout::Hsplit(_, w) => { let ws: Vec<String> = w.iter().map(|v| format!("{v:.4}")).collect(); out.push_str(&format!("layout hsplit {}\n", ws.join(","))); }
            SessionLayout::Vsplit(_, w) => { let ws: Vec<String> = w.iter().map(|v| format!("{v:.4}")).collect(); out.push_str(&format!("layout vsplit {}\n", ws.join(","))); }
        }
        out
    }

    /// Deserialize session data from a string.
    pub fn deserialize(input: &str) -> SessionData {
        let mut data = SessionData::default();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(5, ' ').collect();
            match parts.first().copied() {
                Some("cwd") if parts.len() >= 2 => {
                    data.cwd = Some(PathBuf::from(parts[1]));
                }
                Some("active") if parts.len() >= 2 => {
                    data.active_buffer = parts[1].parse().unwrap_or(0);
                }
                Some("file") if parts.len() >= 4 => {
                    data.files.push(SessionFile {
                        path: PathBuf::from(parts[1]),
                        cursor_line: parts[2].parse().unwrap_or(0),
                        cursor_col: parts[3].parse().unwrap_or(0),
                        was_modified: parts.get(4) == Some(&"m"),
                    });
                }
                Some("mark") if parts.len() >= 4 => {
                    if let Some(name) = parts[1].chars().next() {
                        let line = parts[2].parse().unwrap_or(0);
                        let col = parts[3].parse().unwrap_or(0);
                        data.marks.push((name, line, col));
                    }
                }
                Some("layout") if parts.len() >= 2 => {
                    data.layout = match parts[1] {
                        "hsplit" => {
                            let w = parts.get(2).map(|s| parse_weights(s)).unwrap_or_default();
                            SessionLayout::Hsplit(Vec::new(), w)
                        }
                        "vsplit" => {
                            let w = parts.get(2).map(|s| parse_weights(s)).unwrap_or_default();
                            SessionLayout::Vsplit(Vec::new(), w)
                        }
                        _ => SessionLayout::Single,
                    };
                }
                _ => {}
            }
        }

        data
    }

    /// Get session file path for a given name.
    pub fn session_path(&self, name: &str) -> PathBuf {
        self.session_dir.join(format!("{name}.session"))
    }

    /// Set current session name.
    pub fn set_name(&mut self, name: String) {
        self.current_name = Some(name);
    }

    /// Get current session name.
    pub fn name(&self) -> Option<&str> {
        self.current_name.as_deref()
    }

    /// List available sessions.
    pub fn list_sessions(&self) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(&self.session_dir) else { return Vec::new(); };
        entries.filter_map(|e| e.ok()).filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.strip_suffix(".session").map(|s| s.to_string())
        }).collect()
    }

    /// Save session data to disk.
    pub fn save(&self, name: &str, data: &SessionData) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.session_dir)?;
        let path = self.session_path(name);
        let content = Self::serialize(data);
        std::fs::write(path, content)
    }

    /// Load session data from disk.
    pub fn load(&self, name: &str) -> std::io::Result<SessionData> {
        let path = self.session_path(name);
        let content = std::fs::read_to_string(path)?;
        Ok(Self::deserialize(&content))
    }

    /// Delete a session file.
    pub fn delete(&self, name: &str) -> std::io::Result<()> {
        let path = self.session_path(name);
        if path.exists() {
            std::fs::remove_file(path)
        } else {
            Ok(())
        }
    }

    /// Get session directory.
    pub fn session_dir(&self) -> &Path {
        &self.session_dir
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(PathBuf::from(".sessions"))
    }
}

fn parse_weights(s: &str) -> Vec<f64> {
    s.split(',').filter_map(|w| w.trim().parse().ok()).collect()
}
