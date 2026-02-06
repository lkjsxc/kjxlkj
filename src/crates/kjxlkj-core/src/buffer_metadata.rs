//! Buffer metadata — alternate file, buffer variables, buffer info.

/// Buffer information for display (e.g. :ls output).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferInfo {
    pub id: u64,
    pub name: String,
    pub modified: bool,
    pub readonly: bool,
    pub listed: bool,
    pub loaded: bool,
    pub line_count: usize,
}

/// Alternate file tracking for Ctrl-^.
#[derive(Debug, Clone)]
pub struct AlternateFile {
    current: Option<u64>,
    alternate: Option<u64>,
}

impl AlternateFile {
    pub fn new() -> Self { Self { current: None, alternate: None } }

    /// Switch to a new buffer, updating alternate.
    pub fn switch_to(&mut self, buffer_id: u64) {
        if self.current != Some(buffer_id) {
            self.alternate = self.current;
            self.current = Some(buffer_id);
        }
    }

    pub fn current(&self) -> Option<u64> { self.current }
    pub fn alternate(&self) -> Option<u64> { self.alternate }

    /// Swap current and alternate.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.alternate);
    }
}

impl Default for AlternateFile { fn default() -> Self { Self::new() } }

/// Buffer-local variables (b:var in Vim).
#[derive(Debug, Clone)]
pub struct BufferVariables {
    vars: std::collections::HashMap<String, String>,
}

impl BufferVariables {
    pub fn new() -> Self { Self { vars: std::collections::HashMap::new() } }

    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&str> { self.vars.get(key).map(|s| s.as_str()) }

    pub fn remove(&mut self, key: &str) -> bool { self.vars.remove(key).is_some() }

    pub fn keys(&self) -> Vec<&str> { self.vars.keys().map(|k| k.as_str()).collect() }
}

impl Default for BufferVariables { fn default() -> Self { Self::new() } }

/// Format a buffer info entry for :ls display.
pub fn format_buffer_info(info: &BufferInfo) -> String {
    let flags = format!("{}{}{}{}",
        if info.listed { " " } else { "u" },
        if info.loaded { "a" } else { " " },
        if info.modified { "+" } else { " " },
        if info.readonly { "=" } else { " " },
    );
    format!("{:>3} {}{:>20}  line {}", info.id, flags, format!("\"{}\"", info.name), info.line_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_file_switch() {
        let mut af = AlternateFile::new();
        af.switch_to(1);
        assert_eq!(af.current(), Some(1));
        assert_eq!(af.alternate(), None);
        af.switch_to(2);
        assert_eq!(af.current(), Some(2));
        assert_eq!(af.alternate(), Some(1));
    }

    #[test]
    fn alternate_file_swap() {
        let mut af = AlternateFile::new();
        af.switch_to(1); af.switch_to(2);
        af.swap();
        assert_eq!(af.current(), Some(1));
        assert_eq!(af.alternate(), Some(2));
    }

    #[test]
    fn alternate_same_buffer() {
        let mut af = AlternateFile::new();
        af.switch_to(1); af.switch_to(1);
        assert_eq!(af.current(), Some(1));
        assert_eq!(af.alternate(), None);
    }

    #[test]
    fn buffer_variables_set_get() {
        let mut bv = BufferVariables::new();
        bv.set("filetype", "rust");
        assert_eq!(bv.get("filetype"), Some("rust"));
        assert_eq!(bv.get("missing"), None);
    }

    #[test]
    fn buffer_variables_remove() {
        let mut bv = BufferVariables::new();
        bv.set("key", "val");
        assert!(bv.remove("key"));
        assert!(!bv.remove("key"));
    }

    #[test]
    fn format_buffer_listed_modified() {
        let info = BufferInfo { id: 1, name: "main.rs".into(), modified: true, readonly: false, listed: true, loaded: true, line_count: 42 };
        let s = format_buffer_info(&info);
        assert!(s.contains("main.rs"));
        assert!(s.contains("+"));
        assert!(s.contains("42"));
    }

    #[test]
    fn format_buffer_unlisted() {
        let info = BufferInfo { id: 2, name: "help".into(), modified: false, readonly: true, listed: false, loaded: false, line_count: 100 };
        let s = format_buffer_info(&info);
        assert!(s.contains("u"));
        assert!(s.contains("="));
    }
}
