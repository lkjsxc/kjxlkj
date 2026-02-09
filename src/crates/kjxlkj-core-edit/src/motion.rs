use kjxlkj_core_text::Rope;
use kjxlkj_core_types::CursorPosition;

use crate::motion_helpers::*;

/// A motion describes how the cursor should move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Motion {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    LineStart,
    FirstNonBlank,
    LineEnd,
    WordForward(usize),
    WordBackward(usize),
    WordEndForward(usize),
    FirstLine,
    LastLine,
    GotoLine(usize),
    MatchingBracket,
    PageUp(usize),
    PageDown(usize),
    HalfPageUp(usize),
    HalfPageDown(usize),
    FindCharForward(char),
    FindCharBackward(char),
    TillCharForward(char),
    TillCharBackward(char),
}

/// Whether a motion result is inclusive or exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionKind {
    Exclusive,
    Inclusive,
    Linewise,
}

/// Resolve a motion from a cursor position and rope content.
pub fn resolve_motion(
    motion: &Motion,
    pos: CursorPosition,
    rope: &Rope,
    viewport_height: usize,
) -> (CursorPosition, MotionKind) {
    let line_count = rope.len_lines().max(1);
    match motion {
        Motion::Left(n) => {
            let g = pos.grapheme.saturating_sub(*n);
            (CursorPosition::new(pos.line, g), MotionKind::Exclusive)
        }
        Motion::Right(n) => {
            let max_g = line_graphemes_excl_newline(rope, pos.line);
            let g = (pos.grapheme + n).min(max_g.saturating_sub(1));
            (CursorPosition::new(pos.line, g), MotionKind::Exclusive)
        }
        Motion::Up(n) => {
            let line = pos.line.saturating_sub(*n);
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::Down(n) => {
            let line = (pos.line + n).min(line_count.saturating_sub(1));
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::LineStart => (CursorPosition::new(pos.line, 0), MotionKind::Exclusive),
        Motion::FirstNonBlank => {
            let g = first_non_blank(rope, pos.line);
            (CursorPosition::new(pos.line, g), MotionKind::Exclusive)
        }
        Motion::LineEnd => {
            let max_g = line_graphemes_excl_newline(rope, pos.line);
            let g = max_g.saturating_sub(1);
            (CursorPosition::new(pos.line, g), MotionKind::Inclusive)
        }
        Motion::WordForward(n) => {
            let new_pos = word_forward(rope, pos, *n);
            (new_pos, MotionKind::Exclusive)
        }
        Motion::WordBackward(n) => {
            let new_pos = word_backward(rope, pos, *n);
            (new_pos, MotionKind::Exclusive)
        }
        Motion::WordEndForward(n) => {
            let new_pos = word_end_forward(rope, pos, *n);
            (new_pos, MotionKind::Inclusive)
        }
        Motion::FirstLine => {
            let g = first_non_blank(rope, 0);
            (CursorPosition::new(0, g), MotionKind::Linewise)
        }
        Motion::LastLine => {
            let last = line_count.saturating_sub(1);
            let g = first_non_blank(rope, last);
            (CursorPosition::new(last, g), MotionKind::Linewise)
        }
        Motion::GotoLine(n) => {
            let line = (*n).min(line_count.saturating_sub(1));
            let g = first_non_blank(rope, line);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::MatchingBracket => (pos, MotionKind::Inclusive),
        Motion::PageUp(n) => {
            let lines = viewport_height * n;
            let line = pos.line.saturating_sub(lines);
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::PageDown(n) => {
            let lines = viewport_height * n;
            let line = (pos.line + lines).min(line_count.saturating_sub(1));
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::HalfPageUp(n) => {
            let lines = (viewport_height / 2) * n;
            let line = pos.line.saturating_sub(lines);
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::HalfPageDown(n) => {
            let lines = (viewport_height / 2) * n;
            let line = (pos.line + lines).min(line_count.saturating_sub(1));
            let g = clamp_grapheme(rope, line, pos.grapheme);
            (CursorPosition::new(line, g), MotionKind::Linewise)
        }
        Motion::FindCharForward(ch) => {
            let max_g = line_graphemes_excl_newline(rope, pos.line);
            let mut found = pos.grapheme;
            let line_s: String = rope.line(pos.line).chars().collect();
            let chars: Vec<char> = line_s.chars().collect();
            for g in (pos.grapheme + 1)..max_g {
                if g < chars.len() && chars[g] == *ch {
                    found = g;
                    break;
                }
            }
            (CursorPosition::new(pos.line, found), MotionKind::Inclusive)
        }
        Motion::FindCharBackward(ch) => {
            let line_s: String = rope.line(pos.line).chars().collect();
            let chars: Vec<char> = line_s.chars().collect();
            let mut found = pos.grapheme;
            for g in (0..pos.grapheme).rev() {
                if g < chars.len() && chars[g] == *ch {
                    found = g;
                    break;
                }
            }
            (CursorPosition::new(pos.line, found), MotionKind::Exclusive)
        }
        Motion::TillCharForward(ch) => {
            let max_g = line_graphemes_excl_newline(rope, pos.line);
            let line_s: String = rope.line(pos.line).chars().collect();
            let chars: Vec<char> = line_s.chars().collect();
            let mut found = pos.grapheme;
            for g in (pos.grapheme + 1)..max_g {
                if g < chars.len() && chars[g] == *ch {
                    found = g.saturating_sub(1);
                    break;
                }
            }
            (CursorPosition::new(pos.line, found), MotionKind::Inclusive)
        }
        Motion::TillCharBackward(ch) => {
            let line_s: String = rope.line(pos.line).chars().collect();
            let chars: Vec<char> = line_s.chars().collect();
            let mut found = pos.grapheme;
            for g in (0..pos.grapheme).rev() {
                if g < chars.len() && chars[g] == *ch {
                    found = g + 1;
                    break;
                }
            }
            (CursorPosition::new(pos.line, found), MotionKind::Exclusive)
        }
    }
}
