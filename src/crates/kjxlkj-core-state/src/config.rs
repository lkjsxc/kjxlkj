//! Configuration file loading: reads init.vim-style config.

use crate::EditorState;
use std::path::PathBuf;

/// Default config file locations.
pub fn config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        paths.push(
            PathBuf::from(dir).join("kjxlkj").join("init.conf"),
        );
    }
    if let Some(home) = dirs_home() {
        paths.push(
            home.join(".config")
                .join("kjxlkj")
                .join("init.conf"),
        );
        paths.push(home.join(".kjxlkjrc"));
    }
    paths
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Load the first found config file, or return None.
pub fn find_config() -> Option<PathBuf> {
    for path in config_paths() {
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Execute a config file line by line (as ex commands).
pub fn load_config_file(
    state: &mut EditorState,
    path: &std::path::Path,
) -> Result<usize, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read config: {}", e))?;
    let count = execute_script(state, &contents);
    Ok(count)
}

/// Execute a series of ex command lines.
pub fn execute_script(
    state: &mut EditorState,
    script: &str,
) -> usize {
    let mut count = 0;
    for line in script.lines() {
        let trimmed = line.trim();
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('"') {
            continue;
        }
        // Ensure line starts with : for ex command
        let cmd = if trimmed.starts_with(':') {
            trimmed.to_string()
        } else {
            format!(":{}", trimmed)
        };
        crate::commands::dispatch_ex_command(state, &cmd);
        count += 1;
    }
    count
}

/// Try to load the default config file on startup.
pub fn load_default_config(state: &mut EditorState) {
    if let Some(path) = find_config() {
        match load_config_file(state, &path) {
            Ok(n) => {
                state.message = Some(format!(
                    "Loaded {} commands from {}",
                    n,
                    path.display()
                ));
            }
            Err(e) => {
                state.message = Some(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;
    use kjxlkj_core_types::Size;

    fn setup() -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text("hello");
        s.create_window(bid);
        s
    }

    #[test]
    fn execute_script_set() {
        let mut s = setup();
        let script = "set tabstop=8\nset number\n";
        let count = execute_script(&mut s, script);
        assert_eq!(count, 2);
        assert_eq!(s.options.tabstop, 8);
        assert!(s.options.number);
    }

    #[test]
    fn script_skips_comments() {
        let mut s = setup();
        let script =
            "\" This is a comment\nset nowrap\n\n\" Another\n";
        let count = execute_script(&mut s, script);
        assert_eq!(count, 1);
        assert!(!s.options.wrap);
    }

    #[test]
    fn script_with_colons() {
        let mut s = setup();
        let script = ":set tabstop=2\n:set shiftwidth=2\n";
        let count = execute_script(&mut s, script);
        assert_eq!(count, 2);
        assert_eq!(s.options.tabstop, 2);
    }

    #[test]
    fn script_map_command() {
        let mut s = setup();
        let script = "nmap jk :quit\n";
        execute_script(&mut s, script);
        let m = s.mappings.get(
            crate::mappings::MappingMode::Normal,
            "jk",
        );
        assert!(m.is_some());
        assert_eq!(m.unwrap().rhs, ":quit");
    }

    #[test]
    fn config_paths_not_empty() {
        // At minimum, we should get some paths
        let paths = config_paths();
        // We're on Linux, HOME should be set
        assert!(!paths.is_empty());
    }
}
