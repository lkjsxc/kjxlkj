//! UI snapshot structures for rendering.

use kjxlkj_core_types::{
    BufferMeta, CursorPosition, CursorShape, Mode, Selection, ViewportState, WindowId,
};

/// Snapshot sequence number for ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SnapshotSeq(pub u64);

impl SnapshotSeq {
    /// Create new sequence number.
    pub fn new(seq: u64) -> Self {
        Self(seq)
    }

    /// Increment and return new value.
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Complete editor snapshot for rendering.
#[derive(Debug, Clone)]
pub struct EditorSnapshot {
    /// Sequence number for ordering.
    pub seq: SnapshotSeq,
    /// Current mode.
    pub mode: Mode,
    /// Active window snapshots.
    pub windows: Vec<WindowSnapshot>,
    /// Focused window ID.
    pub focused_window: WindowId,
    /// Terminal dimensions.
    pub terminal_size: TerminalSize,
    /// Status line content.
    pub statusline: StatusLine,
    /// Command line content (if in command mode).
    pub cmdline: Option<CommandLineState>,
    /// Message to display.
    pub message: Option<String>,
}

/// Terminal dimensions.
#[derive(Debug, Clone, Copy, Default)]
pub struct TerminalSize {
    /// Width in columns.
    pub cols: u16,
    /// Height in rows.
    pub rows: u16,
}

impl TerminalSize {
    /// Create new terminal size.
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

/// Window snapshot for rendering.
#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    /// Window ID.
    pub id: WindowId,
    /// Buffer metadata.
    pub buffer: BufferMeta,
    /// Visible lines of text.
    pub lines: Vec<LineSnapshot>,
    /// Cursor position.
    pub cursor: CursorPosition,
    /// Cursor shape.
    pub cursor_shape: CursorShape,
    /// Viewport state.
    pub viewport: ViewportState,
    /// Selection (if any).
    pub selection: Option<Selection>,
    /// Window position and size.
    pub rect: WindowRect,
    /// Show line numbers.
    pub show_numbers: bool,
    /// Focused window.
    pub focused: bool,
}

/// Window rectangle on screen.
#[derive(Debug, Clone, Copy, Default)]
pub struct WindowRect {
    /// Left column.
    pub x: u16,
    /// Top row.
    pub y: u16,
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
}

impl WindowRect {
    /// Create new window rect.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Snapshot of a single line for rendering.
#[derive(Debug, Clone)]
pub struct LineSnapshot {
    /// Line number (1-indexed for display).
    pub line_number: usize,
    /// Text content.
    pub text: String,
    /// Syntax highlighting spans.
    pub highlights: Vec<HighlightSpan>,
    /// Diagnostic markers.
    pub diagnostics: Vec<DiagnosticMarker>,
    /// Git diff status.
    pub git_status: Option<GitLineStatus>,
}

/// Syntax highlight span.
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start column.
    pub start: usize,
    /// End column.
    pub end: usize,
    /// Highlight group.
    pub group: String,
}

/// Diagnostic marker on a line.
#[derive(Debug, Clone)]
pub struct DiagnosticMarker {
    /// Column position.
    pub column: usize,
    /// Severity level.
    pub severity: DiagnosticSeverity,
    /// Short message.
    pub message: String,
}

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// Error.
    Error,
    /// Warning.
    Warning,
    /// Info.
    Info,
    /// Hint.
    Hint,
}

/// Git line status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitLineStatus {
    /// Added line.
    Added,
    /// Modified line.
    Modified,
    /// Deleted marker.
    Deleted,
}

/// Status line content.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    /// Left section.
    pub left: String,
    /// Center section.
    pub center: String,
    /// Right section.
    pub right: String,
}

impl StatusLine {
    /// Create new status line.
    pub fn new(left: String, center: String, right: String) -> Self {
        Self {
            left,
            center,
            right,
        }
    }

    /// Create from mode and buffer info.
    pub fn from_editor_state(mode: Mode, buffer: &BufferMeta, cursor: CursorPosition) -> Self {
        let left = format!(" {} ", mode.name());
        let center = buffer.name.as_str().to_string();
        let modified = if buffer.modified { "[+]" } else { "" };
        let right = format!("{} {}:{} ", modified, cursor.line + 1, cursor.column + 1);
        Self::new(left, center, right)
    }
}

/// Command line state.
#[derive(Debug, Clone)]
pub struct CommandLineState {
    /// Prompt character.
    pub prompt: char,
    /// Current input.
    pub input: String,
    /// Cursor position in input.
    pub cursor_pos: usize,
}

impl CommandLineState {
    /// Create new command line state.
    pub fn new(prompt: char, input: String) -> Self {
        let cursor_pos = input.len();
        Self {
            prompt,
            input,
            cursor_pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, BufferName};

    #[test]
    fn test_snapshot_seq() {
        let seq = SnapshotSeq::new(0);
        let next = seq.next();
        assert!(next > seq);
    }

    #[test]
    fn test_status_line() {
        let meta = BufferMeta::new(BufferId::new(1), BufferName::new("test.rs"));
        let status = StatusLine::from_editor_state(Mode::Normal, &meta, CursorPosition::new(0, 0));
        assert!(status.left.contains("NORMAL"));
        assert!(status.center.contains("test.rs"));
    }
}
