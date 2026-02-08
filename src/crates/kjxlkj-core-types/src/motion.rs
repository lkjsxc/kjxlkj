//! Motion types for cursor movement.

use serde::{Deserialize, Serialize};

/// Directional movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Scroll direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    HalfUp,
    HalfDown,
    PageUp,
    PageDown,
}

/// All motion types supported by the editor.
///
/// Motions define how the cursor moves. They are used standalone
/// (Normal mode navigation) and as the range specifier for operators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Motion {
    /// `h` — move left by graphemes.
    Left,
    /// `l` — move right by graphemes.
    Right,
    /// `j` — move down by lines.
    Down,
    /// `k` — move up by lines.
    Up,
    /// `w` — word forward.
    WordForward,
    /// `W` — WORD forward.
    WordForwardBig,
    /// `b` — word backward.
    WordBackward,
    /// `B` — WORD backward.
    WordBackwardBig,
    /// `e` — end of word forward.
    WordEndForward,
    /// `E` — end of WORD forward.
    WordEndForwardBig,
    /// `0` — start of line.
    LineStart,
    /// `^` — first non-blank.
    FirstNonBlank,
    /// `$` — end of line.
    LineEnd,
    /// `g_` — last non-blank.
    LastNonBlank,
    /// `gg` — go to first line (or line N).
    GotoFirstLine,
    /// `G` — go to last line (or line N).
    GotoLastLine,
    /// `{count}G` or `:{count}` — go to specific line.
    GotoLine(usize),
    /// `f{char}` — find character forward.
    FindCharForward(char),
    /// `F{char}` — find character backward.
    FindCharBackward(char),
    /// `t{char}` — to character forward (before).
    TillCharForward(char),
    /// `T{char}` — to character backward (after).
    TillCharBackward(char),
    /// `;` — repeat last character find.
    RepeatFindForward,
    /// `,` — repeat last character find reversed.
    RepeatFindBackward,
    /// `(` — sentence backward.
    SentenceBackward,
    /// `)` — sentence forward.
    SentenceForward,
    /// `{` — paragraph backward.
    ParagraphBackward,
    /// `}` — paragraph forward.
    ParagraphForward,
    /// `%` — matching bracket.
    MatchingBracket,
    /// `n` — next search match.
    NextSearchMatch,
    /// `N` — previous search match.
    PrevSearchMatch,
    /// `*` — search word under cursor forward.
    StarForward,
    /// `#` — search word under cursor backward.
    StarBackward,
    /// `H` — top of visible screen.
    ScreenTop,
    /// `M` — middle of visible screen.
    ScreenMiddle,
    /// `L` — bottom of visible screen.
    ScreenBottom,
    /// `+` — first non-blank on next line.
    NextLineFirstNonBlank,
    /// `-` — first non-blank on previous line.
    PrevLineFirstNonBlank,
    /// `|` — go to column (with count).
    GotoColumn(usize),
    /// `Ctrl-O` — jump list backward.
    JumpListBackward,
    /// `Ctrl-I` — jump list forward.
    JumpListForward,
}

impl Motion {
    /// Whether this motion moves linewise (affects operator type).
    pub fn is_linewise(&self) -> bool {
        matches!(
            self,
            Motion::Down
                | Motion::Up
                | Motion::GotoFirstLine
                | Motion::GotoLastLine
                | Motion::GotoLine(_)
                | Motion::SentenceBackward
                | Motion::SentenceForward
                | Motion::ParagraphBackward
                | Motion::ParagraphForward
                | Motion::ScreenTop
                | Motion::ScreenMiddle
                | Motion::ScreenBottom
                | Motion::NextLineFirstNonBlank
                | Motion::PrevLineFirstNonBlank
        )
    }

    /// Whether this motion is inclusive (endpoint included in range).
    pub fn is_inclusive(&self) -> bool {
        matches!(
            self,
            Motion::FindCharForward(_)
                | Motion::FindCharBackward(_)
                | Motion::WordEndForward
                | Motion::WordEndForwardBig
                | Motion::LineEnd
                | Motion::LastNonBlank
                | Motion::MatchingBracket
                | Motion::StarForward
                | Motion::StarBackward
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linewise_motions() {
        assert!(Motion::Down.is_linewise());
        assert!(Motion::GotoFirstLine.is_linewise());
        assert!(!Motion::Right.is_linewise());
    }

    #[test]
    fn inclusive_motions() {
        assert!(Motion::FindCharForward('x').is_inclusive());
        assert!(Motion::LineEnd.is_inclusive());
        assert!(!Motion::WordForward.is_inclusive());
    }
}
