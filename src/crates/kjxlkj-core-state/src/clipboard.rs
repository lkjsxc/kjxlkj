//! System clipboard support.
//!
//! Integration with system clipboard (* and + registers).

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

/// Clipboard manager handling both clipboards.
#[derive(Default)]
pub struct ClipboardManager {
    /// Primary selection (*).
    primary: PrimarySelection,
    /// System clipboard (+).
    system: SystemClipboard,
}

impl ClipboardManager {
    /// Creates a new clipboard manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the primary selection.
    pub fn get_primary(&self) -> Option<String> {
        self.primary.get()
    }

    /// Sets the primary selection.
    pub fn set_primary(&mut self, text: &str) -> bool {
        self.primary.set(text)
    }

    /// Gets the system clipboard.
    pub fn get_system(&self) -> Option<String> {
        self.system.get()
    }

    /// Sets the system clipboard.
    pub fn set_system(&mut self, text: &str) -> bool {
        self.system.set(text)
    }

    /// Gets from the specified register.
    pub fn get_register(&self, register: char) -> Option<String> {
        match register {
            '*' => self.get_primary(),
            '+' => self.get_system(),
            _ => None,
        }
    }

    /// Sets to the specified register.
    pub fn set_register(&mut self, register: char, text: &str) -> bool {
        match register {
            '*' => self.set_primary(text),
            '+' => self.set_system(text),
            _ => false,
        }
    }

    /// Returns whether a clipboard register is available.
    pub fn is_available(&self, register: char) -> bool {
        match register {
            '*' => self.primary.available(),
            '+' => self.system.available(),
            _ => false,
        }
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

    #[test]
    fn test_clipboard_manager_primary() {
        let mut mgr = ClipboardManager::new();
        mgr.set_primary("primary");
        assert_eq!(mgr.get_primary(), Some("primary".to_string()));
    }

    #[test]
    fn test_clipboard_manager_system() {
        let mut mgr = ClipboardManager::new();
        mgr.set_system("system");
        assert_eq!(mgr.get_system(), Some("system".to_string()));
    }

    #[test]
    fn test_clipboard_manager_register() {
        let mut mgr = ClipboardManager::new();
        mgr.set_register('+', "plus");
        assert_eq!(mgr.get_register('+'), Some("plus".to_string()));
    }

    #[test]
    fn test_clipboard_is_available() {
        let mgr = ClipboardManager::new();
        assert!(mgr.is_available('+'));
    }
}
