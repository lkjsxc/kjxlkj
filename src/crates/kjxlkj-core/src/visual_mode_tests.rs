use super::*;
use kjxlkj_core_edit::VisualSelection;
use kjxlkj_core_types::Position;

fn setup_visual() -> EditorState {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "hello world\nsecond line\n");
    state.mode.transition(Mode::Visual);
    state.visual = Some(VisualSelection {
        anchor: Position::ZERO,
        cursor: Position::ZERO,
        kind: kjxlkj_core_edit::VisualKind::Char,
    });
    state
}

#[test]
fn escape_returns_normal() {
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::plain(KeyCode::Escape));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Normal)));
    assert!(state.visual.is_none());
}

#[test]
fn d_deletes_selection() {
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::char('d'));
    assert!(matches!(action, Some(EditorAction::DeleteRange(_))));
}

#[test]
fn y_yanks_selection() {
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::char('y'));
    assert!(matches!(action, Some(EditorAction::Yank(_))));
}

#[test]
fn o_swaps_ends() {
    let mut state = setup_visual();
    state.active_window_mut().cursor = Position::new(0, 5);
    if let Some(ref mut sel) = state.visual {
        sel.cursor = Position::new(0, 5);
    }
    handle_visual_key(&mut state, KeyEvent::char('o'));
    if let Some(ref sel) = state.visual {
        assert_eq!(sel.anchor, Position::new(0, 5));
    }
}

#[test]
fn indent_outdent() {
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::char('>'));
    assert_eq!(action, Some(EditorAction::Indent));
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::char('<'));
    assert_eq!(action, Some(EditorAction::Outdent));
}

#[test]
fn colon_command_range() {
    let mut state = setup_visual();
    let action = handle_visual_key(&mut state, KeyEvent::char(':'));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Command)));
    assert!(state.command_line.active);
}
