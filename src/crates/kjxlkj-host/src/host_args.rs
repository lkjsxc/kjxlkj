//! Command-line argument parsing for the host binary.

/// Parsed command-line arguments.
#[derive(Debug, Clone)]
pub struct HostArgs {
    /// File to open on startup.
    pub file: Option<String>,
    /// Run in headless mode (no terminal UI).
    pub headless: bool,
    /// Path to a JSON test script.
    pub script: Option<String>,
}

impl Default for HostArgs {
    fn default() -> Self {
        Self { file: None, headless: false, script: None }
    }
}

/// Parse command-line arguments into `HostArgs`.
///
/// Supports: `--headless`, `--script <path>`, and a positional file.
pub fn parse_args(args: &[String]) -> HostArgs {
    let mut result = HostArgs::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--headless" => result.headless = true,
            "--script" => {
                i += 1;
                if i < args.len() {
                    result.script = Some(args[i].clone());
                    // Headless implied when a script is provided.
                    result.headless = true;
                }
            }
            arg if !arg.starts_with('-') => {
                if result.file.is_none() {
                    result.file = Some(arg.to_string());
                }
            }
            _ => { /* ignore unknown flags */ }
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_args() {
        let a = parse_args(&[]);
        assert!(!a.headless);
        assert!(a.file.is_none());
        assert!(a.script.is_none());
    }

    #[test]
    fn headless_flag() {
        let a = parse_args(&["--headless".into()]);
        assert!(a.headless);
    }

    #[test]
    fn file_positional() {
        let a = parse_args(&["foo.txt".into()]);
        assert_eq!(a.file.as_deref(), Some("foo.txt"));
    }

    #[test]
    fn script_flag() {
        let a = parse_args(&["--script".into(), "test.json".into()]);
        assert_eq!(a.script.as_deref(), Some("test.json"));
        assert!(a.headless);
    }

    #[test]
    fn combined() {
        let a = parse_args(&[
            "main.rs".into(),
            "--headless".into(),
            "--script".into(),
            "s.json".into(),
        ]);
        assert_eq!(a.file.as_deref(), Some("main.rs"));
        assert!(a.headless);
        assert_eq!(a.script.as_deref(), Some("s.json"));
    }

    #[test]
    fn unknown_flags_ignored() {
        let a = parse_args(&["--verbose".into(), "file.rs".into()]);
        assert_eq!(a.file.as_deref(), Some("file.rs"));
    }

    #[test]
    fn default_impl() {
        let a = HostArgs::default();
        assert!(!a.headless);
    }
}
