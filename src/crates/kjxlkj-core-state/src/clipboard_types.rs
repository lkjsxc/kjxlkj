//! Clipboard provider types.

/// Clipboard provider trait.
pub trait ClipboardProvider: Send + Sync {
    /// Gets text from clipboard.
    fn get(&self) -> Option<String>;

    /// Sets text to clipboard.
    fn set(&mut self, text: &str) -> bool;

    /// Returns true if clipboard is available.
    fn available(&self) -> bool;
}

/// Primary selection clipboard (Unix * register).
pub struct PrimarySelection {
    /// Contents.
    contents: Option<String>,
    /// Available flag.
    available: bool,
}

impl PrimarySelection {
    /// Creates a new primary selection.
    pub fn new() -> Self {
        Self {
            contents: None,
            available: cfg!(unix),
        }
    }
}

impl Default for PrimarySelection {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardProvider for PrimarySelection {
    fn get(&self) -> Option<String> {
        self.contents.clone()
    }

    fn set(&mut self, text: &str) -> bool {
        if self.available {
            self.contents = Some(text.to_string());
            true
        } else {
            false
        }
    }

    fn available(&self) -> bool {
        self.available
    }
}

/// System clipboard (+ register).
pub struct SystemClipboard {
    /// Contents.
    contents: Option<String>,
    /// Available flag.
    available: bool,
}

impl SystemClipboard {
    /// Creates a new system clipboard.
    pub fn new() -> Self {
        Self {
            contents: None,
            available: true, // Assume available
        }
    }
}

impl Default for SystemClipboard {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardProvider for SystemClipboard {
    fn get(&self) -> Option<String> {
        self.contents.clone()
    }

    fn set(&mut self, text: &str) -> bool {
        self.contents = Some(text.to_string());
        true
    }

    fn available(&self) -> bool {
        self.available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_selection() {
        let mut ps = PrimarySelection::new();
        ps.set("test");
        assert_eq!(ps.get(), Some("test".to_string()));
    }

    #[test]
    fn test_system_clipboard() {
        let mut sc = SystemClipboard::new();
        sc.set("hello");
        assert_eq!(sc.get(), Some("hello".to_string()));
    }
}
