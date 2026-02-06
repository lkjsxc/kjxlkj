use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{MotionKind, Position};

#[test]
fn left_right() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(apply_motion(&buf, Position::new(0, 2), MotionKind::Left, 1), Position::new(0, 1));
    assert_eq!(apply_motion(&buf, Position::new(0, 2), MotionKind::Right, 1), Position::new(0, 3));
}

#[test]
fn up_down() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    assert_eq!(apply_motion(&buf, Position::new(1, 1), MotionKind::Up, 1), Position::new(0, 1));
    assert_eq!(apply_motion(&buf, Position::new(1, 1), MotionKind::Down, 1), Position::new(2, 1));
}

#[test]
fn line_start_end() {
    let buf = TextBuffer::from_text("  hello  ");
    assert_eq!(apply_motion(&buf, Position::new(0, 4), MotionKind::LineStart, 1), Position::new(0, 0));
    assert_eq!(apply_motion(&buf, Position::new(0, 0), MotionKind::FirstNonBlank, 1), Position::new(0, 2));
}

#[test]
fn file_start_end() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    assert_eq!(apply_motion(&buf, Position::new(1, 0), MotionKind::FileStart, 1), Position::new(0, 0));
    assert_eq!(apply_motion(&buf, Position::new(0, 0), MotionKind::FileEnd, 1).line, 2);
}

#[test]
fn matching_bracket_test() {
    let buf = TextBuffer::from_text("(hello (world))");
    assert_eq!(apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1), Position::new(0, 14));
}

#[test]
fn find_char_test() {
    let buf = TextBuffer::from_text("hello world");
    assert_eq!(apply_motion(&buf, Position::new(0, 0), MotionKind::FindCharForward('o'), 1), Position::new(0, 4));
}
