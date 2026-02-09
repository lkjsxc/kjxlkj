//! LSP feature dispatch stubs per /docs/spec/features/lsp/.
//!
//! Each function sends a ServiceRequest to the LSP service
//! and logs / applies the response to editor state.

use crate::EditorState;

impl EditorState {
    /// Trigger hover at current cursor position.
    /// Sends `textDocument/hover` via LSP service.
    pub fn do_lsp_hover(&mut self) {
        let Some(win) = self.focused_window() else {
            return;
        };
        let _cursor = win.cursor;
        // In a real implementation, we would send a
        // ServiceRequest::LspHover via the service bus.
        // For now, store hover state.
        self.lsp_state.hover_active = true;
    }

    /// Trigger code action at current cursor or selection.
    pub fn do_lsp_code_action(&mut self) {
        let Some(win) = self.focused_window() else {
            return;
        };
        let _cursor = win.cursor;
        self.lsp_state.code_action_active = true;
    }

    /// Format the current buffer via LSP.
    pub fn do_lsp_format(&mut self) {
        if self.active_buffer().is_none() {
            return;
        }
        self.lsp_state.format_requested = true;
    }

    /// Rename symbol under cursor.
    pub fn do_lsp_rename(&mut self, new_name: &str) {
        if self.focused_window().is_none() {
            return;
        }
        self.lsp_state.rename_new_name =
            Some(new_name.to_string());
    }

    /// Trigger signature help.
    pub fn do_lsp_signature_help(&mut self) {
        self.lsp_state.signature_help_active = true;
    }

    /// Find references for symbol under cursor.
    pub fn do_lsp_references(&mut self) {
        if self.focused_window().is_none() {
            return;
        }
        self.lsp_state.references_active = true;
    }

    /// Show document symbols.
    pub fn do_lsp_document_symbols(&mut self) {
        self.lsp_state.document_symbols_active = true;
    }

    /// Show workspace symbols.
    pub fn do_lsp_workspace_symbols(&mut self) {
        self.lsp_state.workspace_symbols_active = true;
    }

    /// Toggle code lens display.
    pub fn do_lsp_code_lens(&mut self) {
        self.lsp_state.code_lens_active =
            !self.lsp_state.code_lens_active;
    }

    /// Toggle inlay hints display.
    pub fn do_lsp_inlay_hints(&mut self) {
        self.lsp_state.inlay_hints_active =
            !self.lsp_state.inlay_hints_active;
    }

    /// Show call hierarchy.
    pub fn do_lsp_call_hierarchy(&mut self) {
        self.lsp_state.call_hierarchy_active = true;
    }

    /// Show type hierarchy.
    pub fn do_lsp_type_hierarchy(&mut self) {
        self.lsp_state.type_hierarchy_active = true;
    }
}

/// LSP feature state tracked in EditorState.
#[derive(Debug, Clone, Default)]
pub struct LspState {
    /// Whether hover popup is active.
    pub hover_active: bool,
    /// Whether code action menu is active.
    pub code_action_active: bool,
    /// Whether format was requested.
    pub format_requested: bool,
    /// New name for rename operation.
    pub rename_new_name: Option<String>,
    /// Whether signature help is active.
    pub signature_help_active: bool,
    /// Whether references list is active.
    pub references_active: bool,
    /// Whether document symbols are shown.
    pub document_symbols_active: bool,
    /// Whether workspace symbols are shown.
    pub workspace_symbols_active: bool,
    /// Whether code lens is displayed.
    pub code_lens_active: bool,
    /// Whether inlay hints are displayed.
    pub inlay_hints_active: bool,
    /// Whether call hierarchy is shown.
    pub call_hierarchy_active: bool,
    /// Whether type hierarchy is shown.
    pub type_hierarchy_active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;

    #[test]
    fn lsp_hover_toggle() {
        let mut ed = EditorState::new(80, 24);
        assert!(!ed.lsp_state.hover_active);
        ed.do_lsp_hover();
        assert!(ed.lsp_state.hover_active);
    }

    #[test]
    fn lsp_inlay_hints_toggle() {
        let mut ed = EditorState::new(80, 24);
        assert!(!ed.lsp_state.inlay_hints_active);
        ed.do_lsp_inlay_hints();
        assert!(ed.lsp_state.inlay_hints_active);
        ed.do_lsp_inlay_hints();
        assert!(!ed.lsp_state.inlay_hints_active);
    }

    #[test]
    fn lsp_rename() {
        let mut ed = EditorState::new(80, 24);
        ed.do_lsp_rename("new_name");
        assert_eq!(
            ed.lsp_state.rename_new_name.as_deref(),
            Some("new_name"),
        );
    }
}
