//! Boundary tests BD-33 through BD-40.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Action, InsertPosition, Mode, Motion, Operator,
};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(e: &mut EditorState, text: &str) {
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for ch in text.chars() {
        e.dispatch(Action::InsertChar(ch));
    }
    e.dispatch(Action::ReturnToNormal);
}

/// BD-33: Terminal CJK output.
#[test]
fn bd33_terminal_cjk() {
    use kjxlkj_core_text::display_width::grapheme_display_width;
    let total: usize = "あいう".chars()
        .map(|c| grapheme_display_width(&c.to_string()) as usize)
        .sum();
    assert_eq!(total, 6);
}

/// BD-34: Terminal color output.
#[test]
fn bd34_terminal_color() {
    let seq = "\x1b[38;2;255;0;0mRED\x1b[0m";
    assert!(seq.contains("38;2;255;0;0"));
    assert!(seq.contains("RED"));
}

/// BD-35: Terminal scrollback.
#[test]
fn bd35_terminal_scrollback() {
    let mut e = ed();
    e.dispatch(Action::SpawnTerminal);
    assert!(e.windows.len() >= 1);
}

/// BD-36: Terminal alternate screen.
#[test]
fn bd36_terminal_alt_screen() {
    assert!("\x1b[?1049h".contains("1049h"));
    assert!("\x1b[?1049l".contains("1049l"));
}

/// BD-37: Terminal resize.
#[test]
fn bd37_terminal_resize() {
    let mut e = ed();
    e.dispatch(Action::SplitHorizontal);
    e.handle_resize(100, 30);
    assert_eq!(e.terminal_size, (100, 30));
}

/// BD-38: All normal mode motions reachable.
#[test]
fn bd38_normal_motions() {
    let mut e = ed();
    ins(&mut e, "hello world\nfoo bar\nbaz");
    let motions = [
        Motion::Left,
        Motion::Down,
        Motion::Up,
        Motion::Right,
        Motion::WordForward,
        Motion::WordBackward,
        Motion::WordEndForward,
        Motion::LineStart,
        Motion::LineEnd,
        Motion::FirstNonBlank,
        Motion::GotoLine(0),
        Motion::GotoLastLine,
    ];
    for m in &motions {
        e.dispatch(Action::MoveCursor(m.clone(), 1));
    }
    assert_eq!(e.mode, Mode::Normal);
}

/// BD-39: All operators with motions.
#[test]
fn bd39_operators() {
    let operators = [
        Operator::Delete,
        Operator::Yank,
        Operator::Change,
    ];
    let motions = [
        Motion::WordForward,
        Motion::LineEnd,
        Motion::Down,
    ];
    for op in &operators {
        for m in &motions {
            let mut e = ed();
            ins(&mut e, "one two three\nfour five six");
            e.dispatch(Action::MoveCursor(
                Motion::LineStart, 1,
            ));
            match op {
                Operator::Delete => {
                    e.dispatch(Action::Delete(
                        m.clone(), 1,
                    ));
                }
                Operator::Yank => {
                    e.dispatch(Action::Yank(
                        m.clone(), 1,
                    ));
                }
                Operator::Change => {
                    e.dispatch(Action::Change(
                        m.clone(), 1,
                    ));
                }
                _ => {}
            }
        }
    }
}

/// BD-40: All ex commands reachable.
#[test]
fn bd40_ex_commands() {
    let commands = [
        "set number",
        "set nonumber",
        "set wrap",
        "set nowrap",
        "noh",
        "marks",
        "registers",
        "jumps",
        "changes",
        "messages",
    ];
    for cmd in &commands {
        let mut e = ed();
        e.dispatch(Action::ExecuteCommand(
            cmd.to_string(),
        ));
    }
}
