use super::*;

#[test]
fn x_deletes_char() {
    let mut state = EditorState::new();
    let action = handle_remaining(&mut state, &KeyEvent::char('x'));
    assert_eq!(action, Some(EditorAction::DeleteChar));
}

#[test]
fn u_undo() {
    let mut state = EditorState::new();
    let action = handle_remaining(&mut state, &KeyEvent::char('u'));
    assert_eq!(action, Some(EditorAction::Undo));
}

#[test]
fn paste_forward() {
    let mut state = EditorState::new();
    let action = handle_remaining(&mut state, &KeyEvent::char('p'));
    assert_eq!(action, Some(EditorAction::Paste(Direction::Forward)));
}

#[test]
fn join_line() {
    let mut state = EditorState::new();
    let action = handle_remaining(&mut state, &KeyEvent::char('J'));
    assert_eq!(action, Some(EditorAction::JoinLine(false)));
}

#[test]
fn open_below_enters_insert() {
    let mut state = EditorState::new();
    let action = open_below(&mut state);
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Insert)));
}

#[test]
fn open_above_enters_insert() {
    let mut state = EditorState::new();
    let action = open_above(&mut state);
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Insert)));
}

#[test]
fn to_eol_range_ok() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "hello");
    let r = to_eol_range(&state);
    assert_eq!(r.end.col, 5);
}
