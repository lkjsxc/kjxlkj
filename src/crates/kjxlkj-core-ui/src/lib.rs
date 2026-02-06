//! UI model types — layout, status line, command line, popups, notifications, completion.

pub mod accessibility;
pub mod completion;
pub mod component;
pub mod cursor_state;
pub mod cursor_visibility;
pub mod float_window;
pub mod mode_config;
pub mod notification_queue;
pub mod notification_render;
pub mod popup_menu;
pub mod statusline;
pub mod theme;
pub mod view_tree;

use kjxlkj_core_types::{BufferId, Mode, Size, WindowId};

/// Represents the visible viewport of a window.
#[derive(Debug, Clone)]
pub struct Viewport {
    pub window_id: WindowId, pub buffer_id: BufferId,
    pub top_line: usize, pub height: u16,
    pub cursor_line: usize, pub cursor_col: usize,
}

/// Status-line data for a single window.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    pub mode: String, pub file_name: String, pub modified: bool,
    pub line: usize, pub col: usize, pub total_lines: usize,
    pub file_type: String, pub encoding: String,
    pub git_branch: Option<String>, pub git_status: Option<String>,
    pub diagnostics: DiagnosticSummary, pub lsp_status: Option<String>,
}

/// Summary of diagnostic counts for statusline display.
#[derive(Debug, Clone, Default)]
pub struct DiagnosticSummary {
    pub errors: usize, pub warnings: usize, pub info: usize, pub hints: usize,
}

/// A statusline component identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusComponent {
    Mode, FileName, FilePath, FileType, Modified, ReadOnly,
    Encoding, FileFormat, Position, Progress, GitBranch, GitStatus,
    Diagnostics, LspStatus, Selection, IndentInfo,
}

/// Command-line state (the `:` prompt at the bottom).
#[derive(Debug, Clone, Default)]
pub struct CommandLine {
    pub content: String, pub cursor_pos: usize, pub visible: bool,
}

/// A message to display in the message area.
#[derive(Debug, Clone)]
pub struct Message { pub text: String, pub kind: MessageKind }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageKind { Info, Warning, Error }

/// Notification level for the notification system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel { Info, Warning, Error, Action }

/// A notification entry.
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64, pub level: NotificationLevel,
    pub text: String, pub timeout_ms: Option<u64>,
}

/// Popup window state.
#[derive(Debug, Clone)]
pub struct PopupState {
    pub visible: bool, pub content: PopupContent,
    pub anchor: PopupAnchor, pub width: u16, pub height: u16,
}

/// Content type for a popup.
#[derive(Debug, Clone)]
pub enum PopupContent { Text(String), Menu(Vec<MenuItem>) }

/// Menu item in a popup.
#[derive(Debug, Clone)]
pub struct MenuItem { pub label: String, pub selected: bool }

/// Popup anchor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupAnchor { Cursor, Editor, Window(WindowId) }

/// Tab/tabline entry for buffer tabs.
#[derive(Debug, Clone)]
pub struct TabEntry {
    pub buffer_id: BufferId, pub name: String,
    pub modified: bool, pub active: bool,
}

/// The complete UI model snapshot used by the renderer.
#[derive(Debug, Clone)]
pub struct UiModel {
    pub size: Size,
    pub viewports: Vec<Viewport>,
    pub status_lines: Vec<StatusLine>,
    pub command_line: CommandLine,
    pub message: Option<Message>,
    pub current_mode: Mode,
    pub tabs: Vec<TabEntry>,
    pub notifications: Vec<Notification>,
    pub popup: Option<PopupState>,
}

impl UiModel {
    pub fn empty(size: Size) -> Self {
        Self {
            size, viewports: Vec::new(), status_lines: Vec::new(),
            command_line: CommandLine::default(), message: None,
            current_mode: Mode::Normal, tabs: Vec::new(),
            notifications: Vec::new(), popup: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_model_empty() {
        let ui = UiModel::empty(Size::new(80, 24));
        assert_eq!(ui.current_mode, Mode::Normal);
        assert!(ui.viewports.is_empty());
        assert!(ui.tabs.is_empty());
        assert!(ui.popup.is_none());
    }

    #[test]
    fn diagnostic_summary_default() {
        let d = DiagnosticSummary::default();
        assert_eq!(d.errors + d.warnings + d.info + d.hints, 0);
    }

    #[test]
    fn status_components() {
        let comps = [StatusComponent::Mode, StatusComponent::FileName,
                     StatusComponent::GitBranch, StatusComponent::Diagnostics,
                     StatusComponent::LspStatus, StatusComponent::Selection];
        assert_eq!(comps.len(), 6);
    }

    #[test]
    fn popup_content_menu() {
        let items = vec![
            MenuItem { label: "Open".into(), selected: true },
            MenuItem { label: "Close".into(), selected: false },
        ];
        let content = PopupContent::Menu(items);
        match content {
            PopupContent::Menu(items) => assert_eq!(items.len(), 2),
            _ => panic!("expected menu"),
        }
    }

    #[test]
    fn notification_levels() {
        let n = Notification { id: 1, level: NotificationLevel::Error,
                               text: "fail".into(), timeout_ms: None };
        assert_eq!(n.level, NotificationLevel::Error);
        assert!(n.timeout_ms.is_none());
    }

    #[test]
    fn tab_entry() {
        let tab = TabEntry { buffer_id: BufferId(1), name: "main.rs".into(),
                             modified: true, active: false };
        assert!(tab.modified);
        assert!(!tab.active);
    }
}
