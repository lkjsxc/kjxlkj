//! Extended UI model tests.

use kjxlkj_core_ui::*;
use kjxlkj_core_types::*;

// ──────────── Viewport ────────────

#[test]
fn viewport_construction() {
    let vp = Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 0,
        height: 24,
        cursor_line: 5,
        cursor_col: 10,
    };
    assert_eq!(vp.top_line, 0);
    assert_eq!(vp.height, 24);
    assert_eq!(vp.cursor_line, 5);
    assert_eq!(vp.cursor_col, 10);
}

#[test]
fn viewport_clone() {
    let vp = Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 10,
        height: 30,
        cursor_line: 15,
        cursor_col: 20,
    };
    let vp2 = vp.clone();
    assert_eq!(vp2.top_line, 10);
    assert_eq!(vp2.cursor_col, 20);
}

// ──────────── StatusLine ────────────

#[test]
fn status_line_default() {
    let sl = StatusLine::default();
    assert_eq!(sl.mode, "");
    assert_eq!(sl.file_name, "");
    assert!(!sl.modified);
    assert_eq!(sl.line, 0);
    assert_eq!(sl.col, 0);
}

#[test]
fn status_line_populated() {
    let sl = StatusLine {
        mode: "NORMAL".to_string(),
        file_name: "test.rs".to_string(),
        modified: true,
        line: 42,
        col: 10,
        total_lines: 100,
    };
    assert_eq!(sl.mode, "NORMAL");
    assert!(sl.modified);
    assert_eq!(sl.total_lines, 100);
}

#[test]
fn status_line_clone() {
    let sl = StatusLine {
        mode: "INSERT".to_string(),
        file_name: "foo.rs".to_string(),
        modified: false,
        line: 1,
        col: 5,
        total_lines: 50,
    };
    let sl2 = sl.clone();
    assert_eq!(sl2.mode, "INSERT");
}

// ──────────── CommandLine ────────────

#[test]
fn command_line_default() {
    let cl = CommandLine::default();
    assert_eq!(cl.content, "");
    assert_eq!(cl.cursor_pos, 0);
    assert!(!cl.visible);
}

#[test]
fn command_line_with_content() {
    let cl = CommandLine {
        content: ":wq".to_string(),
        cursor_pos: 3,
        visible: true,
    };
    assert_eq!(cl.content, ":wq");
    assert!(cl.visible);
}

// ──────────── Message ────────────

#[test]
fn message_info() {
    let m = Message {
        text: "hello".to_string(),
        kind: MessageKind::Info,
    };
    assert_eq!(m.kind, MessageKind::Info);
}

#[test]
fn message_warning() {
    let m = Message {
        text: "warning".to_string(),
        kind: MessageKind::Warning,
    };
    assert_eq!(m.kind, MessageKind::Warning);
}

#[test]
fn message_error() {
    let m = Message {
        text: "error".to_string(),
        kind: MessageKind::Error,
    };
    assert_eq!(m.kind, MessageKind::Error);
}

#[test]
fn message_kinds_ne() {
    assert_ne!(MessageKind::Info, MessageKind::Warning);
    assert_ne!(MessageKind::Warning, MessageKind::Error);
    assert_ne!(MessageKind::Info, MessageKind::Error);
}

// ──────────── UiModel ────────────

#[test]
fn ui_model_empty() {
    let ui = UiModel::empty(Size::new(80, 24));
    assert_eq!(ui.size.width, 80);
    assert_eq!(ui.size.height, 24);
    assert!(ui.viewports.is_empty());
    assert!(ui.status_lines.is_empty());
    assert!(ui.message.is_none());
    assert_eq!(ui.current_mode, Mode::Normal);
}

#[test]
fn ui_model_with_viewport() {
    let mut ui = UiModel::empty(Size::new(120, 40));
    ui.viewports.push(Viewport {
        window_id: WindowId::next(),
        buffer_id: BufferId::next(),
        top_line: 0,
        height: 38,
        cursor_line: 0,
        cursor_col: 0,
    });
    assert_eq!(ui.viewports.len(), 1);
}

#[test]
fn ui_model_with_message() {
    let mut ui = UiModel::empty(Size::new(80, 24));
    ui.message = Some(Message {
        text: "File saved".to_string(),
        kind: MessageKind::Info,
    });
    assert!(ui.message.is_some());
}

#[test]
fn ui_model_multiple_viewports() {
    let mut ui = UiModel::empty(Size::new(160, 50));
    for _ in 0..4 {
        ui.viewports.push(Viewport {
            window_id: WindowId::next(),
            buffer_id: BufferId::next(),
            top_line: 0,
            height: 12,
            cursor_line: 0,
            cursor_col: 0,
        });
    }
    assert_eq!(ui.viewports.len(), 4);
}

#[test]
fn ui_model_clone() {
    let ui = UiModel::empty(Size::new(80, 24));
    let ui2 = ui.clone();
    assert_eq!(ui2.size.width, 80);
}

#[test]
fn ui_model_command_line_visible() {
    let mut ui = UiModel::empty(Size::new(80, 24));
    ui.command_line.visible = true;
    ui.command_line.content = ":set number".to_string();
    ui.current_mode = Mode::Command;
    assert!(ui.command_line.visible);
    assert_eq!(ui.current_mode, Mode::Command);
}
