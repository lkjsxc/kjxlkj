//! Per-buffer state: text content, undo history, file association.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, LanguageId};
use kjxlkj_core_undo::UndoTree;

/// Per-buffer state aggregating text, undo, and metadata.
pub struct BufferState {
    pub id: BufferId,
    pub text: TextBuffer,
    pub undo: UndoTree,
    pub file_path: Option<String>,
    pub modified: bool,
    /// Detected language/filetype.
    pub language: LanguageId,
    /// Whether the buffer is read-only.
    pub readonly: bool,
    /// Whether the buffer is listed (shown in :ls).
    pub listed: bool,
    /// Whether the buffer is a scratch buffer (no file, no save warnings).
    pub scratch: bool,
    /// Buffer-local options (e.g., indentation overrides).
    pub local_tabstop: Option<usize>,
    pub local_shiftwidth: Option<usize>,
    pub local_expandtab: Option<bool>,
}

impl BufferState {
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            text: TextBuffer::new(),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
            language: LanguageId::Plain,
            readonly: false,
            listed: true,
            scratch: false,
            local_tabstop: None,
            local_shiftwidth: None,
            local_expandtab: None,
        }
    }

    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            text: TextBuffer::from_text(text),
            undo: UndoTree::new(),
            file_path: None,
            modified: false,
            language: LanguageId::Plain,
            readonly: false,
            listed: true,
            scratch: false,
            local_tabstop: None,
            local_shiftwidth: None,
            local_expandtab: None,
        }
    }

    /// Set file path and auto-detect language.
    pub fn set_file_path(&mut self, path: &str) {
        self.file_path = Some(path.to_string());
        self.language = LanguageId::detect(path);
    }

    /// Line count of the underlying text buffer.
    pub fn line_count(&self) -> usize {
        self.text.line_count()
    }

    /// Get a line's text content.
    pub fn line_text(&self, line: usize) -> Option<String> {
        if line < self.text.line_count() {
            Some(self.text.line_to_string(line))
        } else {
            None
        }
    }

    /// Effective tabstop (buffer-local or global fallback).
    pub fn effective_tabstop(&self, global: usize) -> usize {
        self.local_tabstop.unwrap_or(global)
    }

    /// Effective shiftwidth (buffer-local or global fallback).
    pub fn effective_shiftwidth(
        &self,
        global: usize,
    ) -> usize {
        self.local_shiftwidth.unwrap_or(global)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty() {
        let buf = BufferState::new(BufferId(1));
        assert!(!buf.modified);
        assert!(buf.file_path.is_none());
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.language, LanguageId::Plain);
    }

    #[test]
    fn buffer_from_text() {
        let buf =
            BufferState::from_text(BufferId(1), "hello\nworld");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn set_file_path_detects_language() {
        let mut buf = BufferState::new(BufferId(1));
        buf.set_file_path("main.rs");
        assert_eq!(buf.language, LanguageId::Rust);
        assert_eq!(buf.file_path.as_deref(), Some("main.rs"));
    }

    #[test]
    fn effective_options() {
        let mut buf = BufferState::new(BufferId(1));
        assert_eq!(buf.effective_tabstop(4), 4);
        buf.local_tabstop = Some(8);
        assert_eq!(buf.effective_tabstop(4), 8);
    }
}
