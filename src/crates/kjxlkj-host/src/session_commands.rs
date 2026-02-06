//! Session save/restore — mksession, source, session layout.

/// Session data that can be saved and restored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionData {
    pub working_dir: String,
    pub open_files: Vec<String>,
    pub active_file: Option<String>,
    pub cursor_positions: Vec<(String, usize, usize)>,
    pub window_layout: WindowLayout,
}

/// Window layout descriptor for session save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowLayout {
    Single,
    Horizontal(Vec<WindowLayout>),
    Vertical(Vec<WindowLayout>),
}

/// Serialize session data to a script-like format.
pub fn serialize_session(session: &SessionData) -> String {
    let mut lines = Vec::new();
    lines.push(format!("cd {}", session.working_dir));
    for file in &session.open_files {
        lines.push(format!("edit {}", file));
    }
    for (file, row, col) in &session.cursor_positions {
        lines.push(format!("cursor {} {} {}", file, row, col));
    }
    if let Some(ref active) = session.active_file {
        lines.push(format!("buffer {}", active));
    }
    lines.push(serialize_layout(&session.window_layout));
    lines.join("\n")
}

fn serialize_layout(layout: &WindowLayout) -> String {
    match layout {
        WindowLayout::Single => "layout single".into(),
        WindowLayout::Horizontal(children) => {
            let parts: Vec<String> = children.iter().map(serialize_layout).collect();
            format!("layout hsplit {}", parts.join(" | "))
        }
        WindowLayout::Vertical(children) => {
            let parts: Vec<String> = children.iter().map(serialize_layout).collect();
            format!("layout vsplit {}", parts.join(" | "))
        }
    }
}

/// Parse a session script back into SessionData.
pub fn parse_session(script: &str) -> SessionData {
    let mut data = SessionData {
        working_dir: String::new(), open_files: Vec::new(),
        active_file: None, cursor_positions: Vec::new(),
        window_layout: WindowLayout::Single,
    };
    for line in script.lines() {
        let line = line.trim();
        if let Some(dir) = line.strip_prefix("cd ") {
            data.working_dir = dir.trim().to_string();
        } else if let Some(file) = line.strip_prefix("edit ") {
            data.open_files.push(file.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("cursor ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() == 3 {
                let file = parts[0].to_string();
                let row: usize = parts[1].parse().unwrap_or(0);
                let col: usize = parts[2].parse().unwrap_or(0);
                data.cursor_positions.push((file, row, col));
            }
        } else if let Some(buf) = line.strip_prefix("buffer ") {
            data.active_file = Some(buf.trim().to_string());
        }
    }
    data
}

/// Check if a session file exists at the given path (stub).
pub fn session_file_exists(path: &str) -> bool { !path.is_empty() }

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_session() -> SessionData {
        SessionData {
            working_dir: "/home/user/project".into(),
            open_files: vec!["main.rs".into(), "lib.rs".into()],
            active_file: Some("main.rs".into()),
            cursor_positions: vec![("main.rs".into(), 10, 5), ("lib.rs".into(), 1, 0)],
            window_layout: WindowLayout::Single,
        }
    }

    #[test]
    fn serialize_basic() {
        let s = serialize_session(&sample_session());
        assert!(s.contains("cd /home/user/project"));
        assert!(s.contains("edit main.rs"));
        assert!(s.contains("cursor main.rs 10 5"));
        assert!(s.contains("buffer main.rs"));
    }

    #[test]
    fn roundtrip() {
        let original = sample_session();
        let script = serialize_session(&original);
        let parsed = parse_session(&script);
        assert_eq!(parsed.working_dir, original.working_dir);
        assert_eq!(parsed.open_files, original.open_files);
        assert_eq!(parsed.active_file, original.active_file);
        assert_eq!(parsed.cursor_positions, original.cursor_positions);
    }

    #[test]
    fn parse_empty() {
        let data = parse_session("");
        assert!(data.open_files.is_empty());
        assert!(data.working_dir.is_empty());
    }

    #[test]
    fn layout_single() {
        assert_eq!(serialize_layout(&WindowLayout::Single), "layout single");
    }

    #[test]
    fn layout_hsplit() {
        let layout = WindowLayout::Horizontal(vec![WindowLayout::Single, WindowLayout::Single]);
        let s = serialize_layout(&layout);
        assert!(s.contains("hsplit"));
    }

    #[test]
    fn layout_vsplit() {
        let layout = WindowLayout::Vertical(vec![WindowLayout::Single, WindowLayout::Single]);
        let s = serialize_layout(&layout);
        assert!(s.contains("vsplit"));
    }

    #[test]
    fn session_file_exists_check() {
        assert!(session_file_exists("/tmp/session.vim"));
        assert!(!session_file_exists(""));
    }
}
