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
    /// Number of tab pages.
    pub tab_count: usize,
    /// Active tab index (0-based).
    pub active_tab: usize,
    /// Per-tab window layouts.
    pub tab_layouts: Vec<SessionLayout>,
    /// Per-tab buffer associations (indices into files vec).
    pub tab_buffers: Vec<Vec<usize>>,
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
    /// Local marks (a-z) for this buffer: (name, line, col).
    pub local_marks: Vec<(char, usize, usize)>,
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
    pub fn new(session_dir: PathBuf) -> Self { Self { session_dir, current_name: None } }

    /// Serialize session data to a string format.
    #[rustfmt::skip]
    pub fn serialize(data: &SessionData) -> String {
        let mut out = String::from("# Session file\n");
        if let Some(ref cwd) = data.cwd { out.push_str(&format!("cwd {}\n", cwd.display())); }
        out.push_str(&format!("active {}\n", data.active_buffer));
        if data.tab_count > 1 { out.push_str(&format!("tabs {} {}\n", data.tab_count, data.active_tab)); }
        for (ti, tl) in data.tab_layouts.iter().enumerate() { out.push_str(&format!("tablayout {} {}\n", ti, serialize_layout(tl))); }
        for (ti, bufs) in data.tab_buffers.iter().enumerate() { let bs: Vec<String> = bufs.iter().map(|b| b.to_string()).collect(); out.push_str(&format!("tabbuf {} {}\n", ti, bs.join(","))); }
        for file in &data.files {
            let m = if file.was_modified { "m" } else { "-" };
            out.push_str(&format!("file {} {} {} {m}\n", file.path.display(), file.cursor_line, file.cursor_col));
            for &(name, line, col) in &file.local_marks { out.push_str(&format!("localmark {name} {line} {col}\n")); }
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
            if line.is_empty() || line.starts_with('#') { continue; }

            let parts: Vec<&str> = line.splitn(5, ' ').collect();
            match parts.first().copied() {
                Some("cwd") if parts.len() >= 2 => { data.cwd = Some(PathBuf::from(parts[1])); }
                Some("active") if parts.len() >= 2 => { data.active_buffer = parts[1].parse().unwrap_or(0); }
                Some("tabs") if parts.len() >= 3 => {
                    data.tab_count = parts[1].parse().unwrap_or(1);
                    data.active_tab = parts[2].parse().unwrap_or(0);
                }
                Some("tablayout") if parts.len() >= 3 => {
                    let layout = parse_layout_str(parts[2]);
                    let idx: usize = parts[1].parse().unwrap_or(0);
                    while data.tab_layouts.len() <= idx { data.tab_layouts.push(SessionLayout::Single); }
                    data.tab_layouts[idx] = layout;
                }
                Some("tabbuf") if parts.len() >= 3 => {
                    let idx: usize = parts[1].parse().unwrap_or(0);
                    let bufs: Vec<usize> = parts[2].split(',').filter_map(|s| s.trim().parse().ok()).collect();
                    while data.tab_buffers.len() <= idx { data.tab_buffers.push(Vec::new()); }
                    data.tab_buffers[idx] = bufs;
                }
                Some("file") if parts.len() >= 4 => {
                    data.files.push(SessionFile {
                        path: PathBuf::from(parts[1]),
                        cursor_line: parts[2].parse().unwrap_or(0),
                        cursor_col: parts[3].parse().unwrap_or(0),
                        was_modified: parts.get(4) == Some(&"m"),
                        local_marks: Vec::new(),
                    });
                }
                Some("localmark") if parts.len() >= 4 => {
                    if let (Some(f), Some(name)) = (data.files.last_mut(), parts[1].chars().next()) {
                        f.local_marks.push((name, parts[2].parse().unwrap_or(0), parts[3].parse().unwrap_or(0)));
                    }
                }
                Some("mark") if parts.len() >= 4 => {
                    if let Some(name) = parts[1].chars().next() { data.marks.push((name, parts[2].parse().unwrap_or(0), parts[3].parse().unwrap_or(0))); }
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
    pub fn session_path(&self, name: &str) -> PathBuf { self.session_dir.join(format!("{name}.session")) }
    /// Set current session name.
    pub fn set_name(&mut self, name: String) { self.current_name = Some(name); }
    /// Get current session name.
    pub fn name(&self) -> Option<&str> { self.current_name.as_deref() }

    /// List available sessions.
    #[rustfmt::skip]
    pub fn list_sessions(&self) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(&self.session_dir) else { return Vec::new(); };
        entries.filter_map(|e| e.ok()).filter_map(|e| { let n = e.file_name().to_string_lossy().to_string(); n.strip_suffix(".session").map(|s| s.to_string()) }).collect()
    }

    /// Save session data to disk.
    #[rustfmt::skip]
    pub fn save(&self, name: &str, data: &SessionData) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.session_dir)?;
        std::fs::write(self.session_path(name), Self::serialize(data))
    }

    /// Load session data from disk.
    pub fn load(&self, name: &str) -> std::io::Result<SessionData> { Ok(Self::deserialize(&std::fs::read_to_string(self.session_path(name))?)) }

    /// Delete a session file.
    pub fn delete(&self, name: &str) -> std::io::Result<()> { let p = self.session_path(name); if p.exists() { std::fs::remove_file(p) } else { Ok(()) } }

    /// Get session directory.
    pub fn session_dir(&self) -> &Path { &self.session_dir }
}

impl Default for SessionManager { fn default() -> Self { Self::new(PathBuf::from(".sessions")) } }

fn parse_weights(s: &str) -> Vec<f64> { s.split(',').filter_map(|w| w.trim().parse().ok()).collect() }

fn serialize_layout(l: &SessionLayout) -> String {
    match l {
        SessionLayout::Single => "single".into(),
        SessionLayout::Hsplit(_, w) => { let ws: Vec<String> = w.iter().map(|v| format!("{v:.4}")).collect(); format!("hsplit:{}", ws.join(",")) }
        SessionLayout::Vsplit(_, w) => { let ws: Vec<String> = w.iter().map(|v| format!("{v:.4}")).collect(); format!("vsplit:{}", ws.join(",")) }
    }
}

fn parse_layout_str(s: &str) -> SessionLayout {
    if let Some(w) = s.strip_prefix("hsplit:") { SessionLayout::Hsplit(Vec::new(), parse_weights(w)) }
    else if let Some(w) = s.strip_prefix("vsplit:") { SessionLayout::Vsplit(Vec::new(), parse_weights(w)) }
    else { SessionLayout::Single }
}
