//! Comprehensive tests for core-ui types and UiModel.

use kjxlkj_core_ui::*;
use kjxlkj_core_types::{BufferId, Mode, Size, WindowId};

// ──────────── Viewport ────────────

#[test]
fn viewport_basic() {
    let v = Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 0,
        height: 24,
        cursor_line: 0,
        cursor_col: 0,
    };
    assert_eq!(v.top_line, 0);
    assert_eq!(v.height, 24);
}

#[test]
fn viewport_cursor_position() {
    let v = Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 10,
        height: 24,
        cursor_line: 15,
        cursor_col: 7,
    };
    assert_eq!(v.cursor_line, 15);
    assert_eq!(v.cursor_col, 7);
}

#[test]
fn viewport_clone() {
    let v = Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 5,
        height: 30,
        cursor_line: 10,
        cursor_col: 3,
    };
    let v2 = v.clone();
    assert_eq!(v2.top_line, 5);
    assert_eq!(v2.height, 30);
}

// ──────────── StatusLine ────────────

#[test]
fn status_line_default() {
    let sl = StatusLine::default();
    assert!(sl.mode.is_empty());
    assert!(sl.file_name.is_empty());
    assert!(!sl.modified);
    assert_eq!(sl.line, 0);
    assert_eq!(sl.col, 0);
    assert_eq!(sl.total_lines, 0);
}

#[test]
fn status_line_fields() {
    let sl = StatusLine {
        mode: "NORMAL".into(),
        file_name: "main.rs".into(),
        modified: true,
        line: 42,
        col: 10,
        total_lines: 200,
    };
    assert_eq!(sl.mode, "NORMAL");
    assert_eq!(sl.file_name, "main.rs");
    assert!(sl.modified);
    assert_eq!(sl.line, 42);
    assert_eq!(sl.col, 10);
    assert_eq!(sl.total_lines, 200);
}

// ──────────── CommandLine ────────────

#[test]
fn command_line_default() {
    let cl = CommandLine::default();
    assert!(cl.content.is_empty());
    assert_eq!(cl.cursor_pos, 0);
    assert!(!cl.visible);
}

#[test]
fn command_line_with_content() {
    let cl = CommandLine {
        content: ":wq".into(),
        cursor_pos: 3,
        visible: true,
    };
    assert_eq!(cl.content, ":wq");
    assert_eq!(cl.cursor_pos, 3);
    assert!(cl.visible);
}

// ──────────── Message ────────────

#[test]
fn message_info() {
    let m = Message {
        text: "file saved".into(),
        kind: MessageKind::Info,
    };
    assert_eq!(m.kind, MessageKind::Info);
}

#[test]
fn message_warning() {
    let m = Message {
        text: "no write since last change".into(),
        kind: MessageKind::Warning,
    };
    assert_eq!(m.kind, MessageKind::Warning);
}

#[test]
fn message_error() {
    let m = Message {
        text: "E492: Not an editor command".into(),
        kind: MessageKind::Error,
    };
    assert_eq!(m.kind, MessageKind::Error);
}

#[test]
fn message_kind_eq() {
    assert_eq!(MessageKind::Info, MessageKind::Info);
    assert_ne!(MessageKind::Info, MessageKind::Error);
    assert_ne!(MessageKind::Warning, MessageKind::Error);
}

// ──────────── UiModel ────────────

#[test]
fn ui_model_empty() {
    let m = UiModel::empty(Size::new(80, 24));
    assert_eq!(m.size.width, 80);
    assert_eq!(m.size.height, 24);
    assert!(m.viewports.is_empty());
    assert!(m.status_lines.is_empty());
    assert!(!m.command_line.visible);
    assert!(m.message.is_none());
    assert_eq!(m.current_mode, Mode::Normal);
}

#[test]
fn ui_model_with_viewport() {
    let mut m = UiModel::empty(Size::new(120, 40));
    m.viewports.push(Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 0,
        height: 38,
        cursor_line: 0,
        cursor_col: 0,
    });
    assert_eq!(m.viewports.len(), 1);
}

#[test]
fn ui_model_with_status_line() {
    let mut m = UiModel::empty(Size::new(80, 24));
    m.status_lines.push(StatusLine {
        mode: "INSERT".into(),
        file_name: "test.rs".into(),
        modified: false,
        line: 1,
        col: 0,
        total_lines: 50,
    });
    assert_eq!(m.status_lines.len(), 1);
    assert_eq!(m.status_lines[0].mode, "INSERT");
}

#[test]
fn ui_model_with_message() {
    let mut m = UiModel::empty(Size::new(80, 24));
    m.message = Some(Message {
        text: "hello".into(),
        kind: MessageKind::Info,
    });
    assert!(m.message.is_some());
}

#[test]
fn ui_model_mode() {
    let mut m = UiModel::empty(Size::new(80, 24));
    m.current_mode = Mode::Insert;
    assert_eq!(m.current_mode, Mode::Insert);
}

#[test]
fn ui_model_clone() {
    let m = UiModel::empty(Size::new(80, 24));
    let m2 = m.clone();
    assert_eq!(m2.size.width, 80);
}
