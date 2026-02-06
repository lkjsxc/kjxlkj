//! Comprehensive tests for core-edit motions.

use kjxlkj_core_edit::*;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::*;

// ──────────── h/j/k/l motions ────────────

#[test]
fn motion_left_basic() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 3), MotionKind::Left, 1),
        Position::new(0, 2)
    );
}

#[test]
fn motion_left_at_start() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::Left, 1),
        Position::new(0, 0)
    );
}

#[test]
fn motion_left_with_count() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 4), MotionKind::Left, 3),
        Position::new(0, 1)
    );
}

#[test]
fn motion_left_count_past_start() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 2), MotionKind::Left, 10),
        Position::new(0, 0)
    );
}

#[test]
fn motion_right_basic() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 1), MotionKind::Right, 1),
        Position::new(0, 2)
    );
}

#[test]
fn motion_right_at_end() {
    let buf = TextBuffer::from_text("hello");
    let end = apply_motion(&buf, Position::new(0, 4), MotionKind::Right, 1);
    assert_eq!(end, Position::new(0, 4));
}

#[test]
fn motion_right_with_count() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::Right, 3),
        Position::new(0, 3)
    );
}

#[test]
fn motion_up_basic() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    assert_eq!(
        apply_motion(&buf, Position::new(2, 1), MotionKind::Up, 1),
        Position::new(1, 1)
    );
}

#[test]
fn motion_up_at_first_line() {
    let buf = TextBuffer::from_text("abc\ndef");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 1), MotionKind::Up, 1),
        Position::new(0, 1)
    );
}

#[test]
fn motion_up_col_clamp() {
    let buf = TextBuffer::from_text("ab\ndefgh");
    // Line 0 has 2 chars, so col 4 clamps to 1
    assert_eq!(
        apply_motion(&buf, Position::new(1, 4), MotionKind::Up, 1),
        Position::new(0, 1)
    );
}

#[test]
fn motion_down_basic() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 1), MotionKind::Down, 1),
        Position::new(1, 1)
    );
}

#[test]
fn motion_down_at_last_line() {
    let buf = TextBuffer::from_text("abc\ndef");
    assert_eq!(
        apply_motion(&buf, Position::new(1, 2), MotionKind::Down, 1),
        Position::new(1, 2)
    );
}

#[test]
fn motion_down_with_count() {
    let buf = TextBuffer::from_text("a\nb\nc\nd\ne");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::Down, 3),
        Position::new(3, 0)
    );
}

// ──────────── line start / end motions ────────────

#[test]
fn motion_line_start() {
    let buf = TextBuffer::from_text("  hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 5), MotionKind::LineStart, 1),
        Position::new(0, 0)
    );
}

#[test]
fn motion_line_end() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::LineEnd, 1),
        Position::new(0, 4)
    );
}

#[test]
fn motion_first_non_blank() {
    let buf = TextBuffer::from_text("   hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::FirstNonBlank, 1),
        Position::new(0, 3)
    );
}

#[test]
fn motion_first_non_blank_no_leading() {
    let buf = TextBuffer::from_text("hello");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 3), MotionKind::FirstNonBlank, 1),
        Position::new(0, 0)
    );
}

#[test]
fn motion_last_non_blank() {
    let buf = TextBuffer::from_text("hello   ");
    assert_eq!(
        apply_motion(&buf, Position::new(0, 0), MotionKind::LastNonBlank, 1),
        Position::new(0, 4)
    );
}

// ──────────── file start / end ────────────

#[test]
fn motion_file_start() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    assert_eq!(
        apply_motion(&buf, Position::new(2, 2), MotionKind::FileStart, 1),
        Position::new(0, 0)
    );
}

#[test]
fn motion_file_end() {
    let buf = TextBuffer::from_text("abc\ndef\nghi");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::FileEnd, 1);
    assert_eq!(r.line, 2);
}

// ──────────── goto motions ────────────

#[test]
fn motion_goto_line() {
    let buf = TextBuffer::from_text("a\nb\nc\nd");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::GotoLine(3), 1);
    assert_eq!(r.line, 2); // 1-indexed → 0-indexed
}

#[test]
fn motion_goto_line_beyond() {
    let buf = TextBuffer::from_text("a\nb");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::GotoLine(100), 1);
    assert_eq!(r.line, 1);
}

#[test]
fn motion_goto_column() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::GotoColumn(5), 1);
    assert_eq!(r.col, 4); // 1-indexed → 0-indexed
}

#[test]
fn motion_goto_percent_50() {
    let buf = TextBuffer::from_text("a\nb\nc\nd\ne\nf\ng\nh\ni\nj");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::GotoPercent(50), 1);
    assert_eq!(r.line, 5);
}

#[test]
fn motion_goto_percent_100() {
    let buf = TextBuffer::from_text("a\nb\nc");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::GotoPercent(100), 1);
    assert_eq!(r.line, 2);
}

// ──────────── word motions ────────────

#[test]
fn motion_word_forward() {
    let buf = TextBuffer::from_text("hello world test");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::WordForward, 1);
    assert_eq!(r, Position::new(0, 6));
}

#[test]
fn motion_word_forward_count() {
    let buf = TextBuffer::from_text("a b c d");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::WordForward, 2);
    assert_eq!(r, Position::new(0, 4));
}

#[test]
fn motion_word_backward() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 8), MotionKind::WordBackward, 1);
    assert_eq!(r, Position::new(0, 6));
}

#[test]
fn motion_word_forward_end() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::WordForwardEnd, 1);
    assert_eq!(r, Position::new(0, 4));
}

// ──────────── paragraph motions ────────────

#[test]
fn motion_next_paragraph() {
    let buf = TextBuffer::from_text("a\nb\n\nc\nd");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::NextParagraph, 1);
    assert_eq!(r, Position::new(3, 0));
}

#[test]
fn motion_prev_paragraph() {
    let buf = TextBuffer::from_text("a\nb\n\nc\nd");
    let r = apply_motion(&buf, Position::new(4, 0), MotionKind::PrevParagraph, 1);
    // Goes back to before the blank line
    assert!(r.line <= 2);
}

// ──────────── matching bracket ────────────

#[test]
fn motion_matching_bracket_paren() {
    let buf = TextBuffer::from_text("(hello)");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 6));
}

#[test]
fn motion_matching_bracket_reverse() {
    let buf = TextBuffer::from_text("(hello)");
    let r = apply_motion(&buf, Position::new(0, 6), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 0));
}

#[test]
fn motion_matching_bracket_nested() {
    let buf = TextBuffer::from_text("((a)(b))");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 7));
}

#[test]
fn motion_matching_bracket_square() {
    let buf = TextBuffer::from_text("[1, [2, 3]]");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 10));
}

#[test]
fn motion_matching_bracket_curly() {
    let buf = TextBuffer::from_text("{a}");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 2));
}

#[test]
fn motion_matching_bracket_no_match() {
    let buf = TextBuffer::from_text("hello");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MatchingBracket, 1);
    assert_eq!(r, Position::new(0, 0));
}

// ──────────── find char motions ────────────

#[test]
fn motion_find_char_forward() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::FindCharForward('w'), 1);
    assert_eq!(r, Position::new(0, 6));
}

#[test]
fn motion_find_char_forward_not_found() {
    let buf = TextBuffer::from_text("hello");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::FindCharForward('z'), 1);
    assert_eq!(r, Position::new(0, 0));
}

#[test]
fn motion_find_char_backward() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 8), MotionKind::FindCharBackward('l'), 1);
    assert_eq!(r, Position::new(0, 3));
}

#[test]
fn motion_till_char_forward() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::TillCharForward('w'), 1);
    assert_eq!(r, Position::new(0, 5));
}

#[test]
fn motion_till_char_backward() {
    let buf = TextBuffer::from_text("hello world");
    let r = apply_motion(&buf, Position::new(0, 8), MotionKind::TillCharBackward('l'), 1);
    assert_eq!(r, Position::new(0, 4));
}

// ──────────── compute_motion_range ────────────

#[test]
fn range_forward_motion() {
    let buf = TextBuffer::from_text("hello world");
    let r = compute_motion_range(&buf, Position::new(0, 0), MotionKind::WordForward, 1);
    assert_eq!(r.start, Position::new(0, 0));
    assert!(r.end.col > 0);
}

#[test]
fn range_backward_motion() {
    let buf = TextBuffer::from_text("hello world");
    let r = compute_motion_range(&buf, Position::new(0, 8), MotionKind::WordBackward, 1);
    // For backward motion, start < end still
    assert!(r.start.col <= r.end.col);
}

// ──────────── middle of line ────────────

#[test]
fn motion_middle_of_line() {
    let buf = TextBuffer::from_text("0123456789");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::MiddleOfLine, 1);
    assert_eq!(r, Position::new(0, 5));
}

// ──────────── next/prev non-blank line ────────────

#[test]
fn motion_next_non_blank_line() {
    let buf = TextBuffer::from_text("   abc\n  def");
    let r = apply_motion(&buf, Position::new(0, 0), MotionKind::NextNonBlankLine, 1);
    assert_eq!(r.line, 1);
    assert_eq!(r.col, 2); // first non-blank
}

#[test]
fn motion_prev_non_blank_line() {
    let buf = TextBuffer::from_text("   abc\n  def");
    let r = apply_motion(&buf, Position::new(1, 0), MotionKind::PrevNonBlankLine, 1);
    assert_eq!(r.line, 0);
    assert_eq!(r.col, 3);
}
