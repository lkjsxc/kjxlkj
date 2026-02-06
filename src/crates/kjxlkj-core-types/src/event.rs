//! Events flowing through the editor system.

use crate::{BufferId, Size};
use serde::{Deserialize, Serialize};

/// A key event from the terminal.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

impl KeyEvent {
    pub fn char(c: char) -> Self {
        Self { code: KeyCode::Char(c), ctrl: false, alt: false, shift: false }
    }
    pub fn ctrl(c: char) -> Self {
        Self { code: KeyCode::Char(c), ctrl: true, alt: false, shift: false }
    }
    pub fn special(code: KeyCode) -> Self {
        Self { code, ctrl: false, alt: false, shift: false }
    }
}

/// Key identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    Char(char), Escape, Enter, Backspace, Tab,
    Left, Right, Up, Down, Home, End,
    PageUp, PageDown, Delete, F(u8),
}

/// High-level editor event (terminal input decoded to intent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorEvent {
    Key(KeyEvent),
    Resize(Size),
    Quit,
}

/// Service message envelope.
#[derive(Debug, Clone)]
pub enum ServiceMessage {
    FileChanged(std::path::PathBuf),
    DiagnosticsUpdate(BufferId, Vec<Diagnostic>),
    Notification(String),
}

/// A diagnostic entry from LSP or similar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: crate::Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: Option<String>,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error, Warning, Info, Hint,
}
