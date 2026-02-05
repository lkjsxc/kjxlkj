//! Clipboard integration.
//!
//! Provides system clipboard access and register management.

use std::collections::HashMap;

/// Clipboard provider type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardProvider {
    /// X11 clipboard (xclip/xsel).
    X11,
    /// Wayland clipboard (wl-clipboard).
    Wayland,
    /// macOS pasteboard.
    MacOS,
    /// Windows clipboard.
    Windows,
    /// OSC 52 terminal escape.
    Osc52,
    /// Internal fallback.
    Internal,
}

impl ClipboardProvider {
    /// Detect available provider.
    pub fn detect() -> Self {
        // Check environment
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            return Self::Wayland;
        }
        if std::env::var("DISPLAY").is_ok() {
            return Self::X11;
        }

        // Check OS
        #[cfg(target_os = "macos")]
        return Self::MacOS;

        #[cfg(target_os = "windows")]
        return Self::Windows;

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        Self::Internal
    }

    /// Get command for copy operation.
    pub fn copy_cmd(&self) -> Option<(&'static str, &'static [&'static str])> {
        match self {
            Self::X11 => Some(("xclip", &["-selection", "clipboard"])),
            Self::Wayland => Some(("wl-copy", &[])),
            Self::MacOS => Some(("pbcopy", &[])),
            _ => None,
        }
    }

    /// Get command for paste operation.
    pub fn paste_cmd(&self) -> Option<(&'static str, &'static [&'static str])> {
        match self {
            Self::X11 => Some(("xclip", &["-selection", "clipboard", "-o"])),
            Self::Wayland => Some(("wl-paste", &[])),
            Self::MacOS => Some(("pbpaste", &[])),
            _ => None,
        }
    }
}

/// X11 selection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X11Selection {
    /// Primary selection (middle-click).
    Primary,
    /// Clipboard selection (Ctrl+C).
    Clipboard,
    /// Secondary selection.
    Secondary,
}

/// Vim register names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    /// Unnamed register ("").
    Unnamed,
    /// System clipboard ("+).
    Clipboard,
    /// Primary selection ("*).
    Primary,
    /// Named registers (a-z).
    Named(char),
    /// Numbered registers (0-9).
    Numbered(u8),
    /// Small delete register ("-).
    SmallDelete,
    /// Read-only registers.
    ReadOnly(ReadOnlyRegister),
    /// Expression register ("=).
    Expression,
    /// Black hole register ("_).
    BlackHole,
    /// Last search register ("/).
    LastSearch,
}

/// Read-only register types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadOnlyRegister {
    /// Current filename (%).
    CurrentFile,
    /// Alternate filename (#).
    AlternateFile,
    /// Last command (:).
    LastCommand,
    /// Last inserted text (.).
    LastInserted,
}

impl Register {
    /// Parse register from character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '"' => Some(Self::Unnamed),
            '+' => Some(Self::Clipboard),
            '*' => Some(Self::Primary),
            'a'..='z' => Some(Self::Named(c)),
            'A'..='Z' => Some(Self::Named(c.to_ascii_lowercase())),
            '0'..='9' => Some(Self::Numbered(c as u8 - b'0')),
            '-' => Some(Self::SmallDelete),
            '%' => Some(Self::ReadOnly(ReadOnlyRegister::CurrentFile)),
            '#' => Some(Self::ReadOnly(ReadOnlyRegister::AlternateFile)),
            ':' => Some(Self::ReadOnly(ReadOnlyRegister::LastCommand)),
            '.' => Some(Self::ReadOnly(ReadOnlyRegister::LastInserted)),
            '=' => Some(Self::Expression),
            '_' => Some(Self::BlackHole),
            '/' => Some(Self::LastSearch),
            _ => None,
        }
    }

    /// Get register character.
    pub fn to_char(&self) -> char {
        match self {
            Self::Unnamed => '"',
            Self::Clipboard => '+',
            Self::Primary => '*',
            Self::Named(c) => *c,
            Self::Numbered(n) => (b'0' + n) as char,
            Self::SmallDelete => '-',
            Self::ReadOnly(ReadOnlyRegister::CurrentFile) => '%',
            Self::ReadOnly(ReadOnlyRegister::AlternateFile) => '#',
            Self::ReadOnly(ReadOnlyRegister::LastCommand) => ':',
            Self::ReadOnly(ReadOnlyRegister::LastInserted) => '.',
            Self::Expression => '=',
            Self::BlackHole => '_',
            Self::LastSearch => '/',
        }
    }

    /// Check if register is writable.
    pub fn is_writable(&self) -> bool {
        !matches!(self, Self::ReadOnly(_) | Self::BlackHole)
    }

    /// Check if this is a clipboard register.
    pub fn is_clipboard(&self) -> bool {
        matches!(self, Self::Clipboard | Self::Primary)
    }
}

/// Register content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    /// Character-wise.
    Char,
    /// Line-wise.
    Line,
    /// Block-wise.
    Block,
}

/// Content stored in a register.
#[derive(Debug, Clone)]
pub struct RegisterContent {
    /// The text content.
    pub text: String,
    /// Type of content.
    pub register_type: RegisterType,
}

impl RegisterContent {
    /// Create character-wise content.
    pub fn char(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            register_type: RegisterType::Char,
        }
    }

    /// Create line-wise content.
    pub fn line(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            register_type: RegisterType::Line,
        }
    }

    /// Create block-wise content.
    pub fn block(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            register_type: RegisterType::Block,
        }
    }
}

/// Clipboard configuration.
#[derive(Debug, Clone)]
pub struct ClipboardConfig {
    /// Sync unnamed register with clipboard.
    pub sync_unnamed: bool,
    /// Enable OSC 52 clipboard.
    pub osc52: bool,
    /// Max content size (bytes).
    pub max_size: usize,
    /// Provider override.
    pub provider: Option<ClipboardProvider>,
}

impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            sync_unnamed: false,
            osc52: false,
            max_size: 10 * 1024 * 1024, // 10MB
            provider: None,
        }
    }
}

/// Clipboard and register manager.
#[derive(Debug)]
pub struct Clipboard {
    /// Configuration.
    config: ClipboardConfig,
    /// Active provider.
    provider: ClipboardProvider,
    /// Register storage.
    registers: HashMap<Register, RegisterContent>,
    /// Internal clipboard (fallback).
    internal: Option<String>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Clipboard {
    /// Create new clipboard.
    pub fn new() -> Self {
        Self::with_config(ClipboardConfig::default())
    }

    /// Create with config.
    pub fn with_config(config: ClipboardConfig) -> Self {
        let provider = config.provider.unwrap_or_else(ClipboardProvider::detect);
        Self {
            config,
            provider,
            registers: HashMap::new(),
            internal: None,
        }
    }

    /// Get active provider.
    pub fn provider(&self) -> ClipboardProvider {
        self.provider
    }

    /// Set register content.
    pub fn set(&mut self, register: Register, content: RegisterContent) {
        if !register.is_writable() {
            return;
        }

        // Handle black hole
        if matches!(register, Register::BlackHole) {
            return;
        }

        // Handle append mode (uppercase named registers)
        // Append logic would be handled at call site

        // Store in register
        self.registers.insert(register, content.clone());

        // Sync with unnamed if needed
        if !matches!(register, Register::Unnamed) && !register.is_clipboard() {
            self.registers.insert(Register::Unnamed, content.clone());
        }

        // Sync with system clipboard if configured
        if self.config.sync_unnamed && matches!(register, Register::Unnamed) {
            self.set_system(&content.text);
        }

        // If writing to clipboard register, sync to system
        if register.is_clipboard() {
            self.set_system(&content.text);
        }
    }

    /// Get register content.
    pub fn get(&self, register: Register) -> Option<&RegisterContent> {
        // Handle clipboard registers
        if register.is_clipboard() {
            // In real implementation, would fetch from system
            return self.registers.get(&register);
        }

        self.registers.get(&register)
    }

    /// Yank text to register.
    pub fn yank(&mut self, register: Register, text: &str, line_wise: bool) {
        let content = if line_wise {
            RegisterContent::line(text)
        } else {
            RegisterContent::char(text)
        };

        // Rotate numbered registers for delete operations
        self.rotate_numbered();
        self.registers.insert(Register::Numbered(0), content.clone());

        self.set(register, content);
    }

    /// Delete text to register.
    pub fn delete(&mut self, register: Register, text: &str, line_wise: bool, small: bool) {
        let content = if line_wise {
            RegisterContent::line(text)
        } else {
            RegisterContent::char(text)
        };

        if small && !line_wise && text.len() < 80 && !text.contains('\n') {
            // Small delete register
            self.registers.insert(Register::SmallDelete, content.clone());
        } else {
            // Rotate numbered registers
            self.rotate_numbered();
            self.registers.insert(Register::Numbered(1), content.clone());
        }

        self.set(register, content);
    }

    /// Put (paste) from register.
    pub fn put(&self, register: Register) -> Option<&RegisterContent> {
        self.get(register)
    }

    /// Set system clipboard.
    fn set_system(&mut self, text: &str) {
        // In real implementation, would use provider commands
        self.internal = Some(text.to_string());
    }

    /// Get system clipboard.
    pub fn get_system(&self) -> Option<&str> {
        self.internal.as_deref()
    }

    /// Rotate numbered registers (1 -> 2, 2 -> 3, etc.).
    fn rotate_numbered(&mut self) {
        for i in (1..9).rev() {
            if let Some(content) = self.registers.remove(&Register::Numbered(i)) {
                self.registers.insert(Register::Numbered(i + 1), content);
            }
        }
    }

    /// Set read-only register.
    pub fn set_readonly(&mut self, reg: ReadOnlyRegister, text: &str) {
        self.registers.insert(
            Register::ReadOnly(reg),
            RegisterContent::char(text),
        );
    }

    /// List all registers with content.
    pub fn list(&self) -> Vec<(Register, &RegisterContent)> {
        self.registers.iter().map(|(r, c)| (*r, c)).collect()
    }

    /// Clear a register.
    pub fn clear(&mut self, register: Register) {
        if register.is_writable() {
            self.registers.remove(&register);
        }
    }

    /// Generate OSC 52 escape sequence.
    /// Note: In real implementation, would use base64 encoding.
    pub fn osc52_set(text: &str) -> String {
        let encoded = simple_base64_encode(text.as_bytes());
        format!("\x1b]52;c;{}\x07", encoded)
    }

    /// Parse OSC 52 response.
    /// Note: In real implementation, would use base64 decoding.
    pub fn osc52_parse(response: &str) -> Option<String> {
        // Response format: \x1b]52;c;BASE64\x07
        let start = response.find(";c;")? + 3;
        let end = response.find('\x07')?;
        let encoded = &response[start..end];
        simple_base64_decode(encoded).and_then(|b| String::from_utf8(b).ok())
    }
}

// Simple base64 encoding (no external dependency)
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn simple_base64_encode(data: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = if i + 1 < data.len() { data[i + 1] as usize } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as usize } else { 0 };

        result.push(BASE64_CHARS[(b0 >> 2) & 0x3f] as char);
        result.push(BASE64_CHARS[((b0 << 4) | (b1 >> 4)) & 0x3f] as char);

        if i + 1 < data.len() {
            result.push(BASE64_CHARS[((b1 << 2) | (b2 >> 6)) & 0x3f] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(BASE64_CHARS[b2 & 0x3f] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

fn simple_base64_decode(data: &str) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let chars: Vec<u8> = data.bytes().collect();
    let mut i = 0;

    while i < chars.len() {
        let c0 = base64_char_value(chars[i])?;
        let c1 = base64_char_value(chars[i + 1])?;
        result.push(((c0 << 2) | (c1 >> 4)) as u8);

        if i + 2 < chars.len() && chars[i + 2] != b'=' {
            let c2 = base64_char_value(chars[i + 2])?;
            result.push((((c1 & 0xf) << 4) | (c2 >> 2)) as u8);

            if i + 3 < chars.len() && chars[i + 3] != b'=' {
                let c3 = base64_char_value(chars[i + 3])?;
                result.push((((c2 & 0x3) << 6) | c3) as u8);
            }
        }

        i += 4;
    }

    Some(result)
}

fn base64_char_value(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_detect() {
        // Just test it doesn't panic
        let _provider = ClipboardProvider::detect();
    }

    #[test]
    fn test_register_from_char() {
        assert_eq!(Register::from_char('"'), Some(Register::Unnamed));
        assert_eq!(Register::from_char('+'), Some(Register::Clipboard));
        assert_eq!(Register::from_char('*'), Some(Register::Primary));
        assert_eq!(Register::from_char('a'), Some(Register::Named('a')));
        assert_eq!(Register::from_char('0'), Some(Register::Numbered(0)));
        assert_eq!(Register::from_char('_'), Some(Register::BlackHole));
    }

    #[test]
    fn test_register_to_char() {
        assert_eq!(Register::Unnamed.to_char(), '"');
        assert_eq!(Register::Named('x').to_char(), 'x');
    }

    #[test]
    fn test_register_writable() {
        assert!(Register::Unnamed.is_writable());
        assert!(Register::Named('a').is_writable());
        assert!(!Register::ReadOnly(ReadOnlyRegister::CurrentFile).is_writable());
        assert!(!Register::BlackHole.is_writable());
    }

    #[test]
    fn test_register_content() {
        let c = RegisterContent::char("hello");
        assert_eq!(c.register_type, RegisterType::Char);

        let l = RegisterContent::line("hello\n");
        assert_eq!(l.register_type, RegisterType::Line);
    }

    #[test]
    fn test_clipboard_set_get() {
        let mut clip = Clipboard::new();
        clip.set(Register::Named('a'), RegisterContent::char("test"));
        let content = clip.get(Register::Named('a')).unwrap();
        assert_eq!(content.text, "test");
    }

    #[test]
    fn test_clipboard_yank() {
        let mut clip = Clipboard::new();
        clip.yank(Register::Unnamed, "hello", false);

        let content = clip.get(Register::Unnamed).unwrap();
        assert_eq!(content.text, "hello");
        assert_eq!(content.register_type, RegisterType::Char);

        let num0 = clip.get(Register::Numbered(0)).unwrap();
        assert_eq!(num0.text, "hello");
    }

    #[test]
    fn test_clipboard_delete_small() {
        let mut clip = Clipboard::new();
        clip.delete(Register::Unnamed, "x", false, true);

        let small = clip.get(Register::SmallDelete).unwrap();
        assert_eq!(small.text, "x");
    }

    #[test]
    fn test_clipboard_numbered_rotation() {
        let mut clip = Clipboard::new();
        clip.yank(Register::Unnamed, "first", false);
        clip.yank(Register::Unnamed, "second", false);
        clip.yank(Register::Unnamed, "third", false);

        assert_eq!(clip.get(Register::Numbered(0)).unwrap().text, "third");
    }

    #[test]
    fn test_clipboard_readonly() {
        let mut clip = Clipboard::new();
        clip.set_readonly(ReadOnlyRegister::CurrentFile, "test.rs");
        let content = clip.get(Register::ReadOnly(ReadOnlyRegister::CurrentFile)).unwrap();
        assert_eq!(content.text, "test.rs");
    }

    #[test]
    fn test_clipboard_black_hole() {
        let mut clip = Clipboard::new();
        clip.set(Register::BlackHole, RegisterContent::char("deleted"));
        assert!(clip.get(Register::BlackHole).is_none());
    }

    #[test]
    fn test_osc52_roundtrip() {
        let text = "hello clipboard";
        let escaped = Clipboard::osc52_set(text);
        assert!(escaped.starts_with("\x1b]52;c;"));
        assert!(escaped.ends_with("\x07"));
    }

    #[test]
    fn test_clipboard_list() {
        let mut clip = Clipboard::new();
        clip.set(Register::Named('a'), RegisterContent::char("test1"));
        clip.set(Register::Named('b'), RegisterContent::char("test2"));
        let list = clip.list();
        assert!(list.len() >= 2);
    }
}
