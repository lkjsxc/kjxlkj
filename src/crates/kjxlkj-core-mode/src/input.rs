//! Input parsing results.

use kjxlkj_core_edit::{Motion, Operator, TextObject};
use kjxlkj_core_types::Mode;

/// Result of processing a key.
#[derive(Debug)]
pub enum InputResult {
    /// Key was handled, no further action needed.
    Handled,
    /// Key triggered a mode change.
    ModeChange(Mode),
    /// Key was parsed into an actionable input.
    Parsed(ParsedInput),
    /// Key is part of a multi-key sequence, awaiting more input.
    Pending,
    /// Key was not handled.
    Unhandled,
}

/// A fully parsed input action.
#[derive(Debug)]
pub enum ParsedInput {
    // Motions.
    Motion(Motion),

    // Operators with targets.
    OperatorMotion(Operator, Motion),
    OperatorLine(Operator, usize),
    OperatorTextObject(Operator, TextObject),

    // Single-character edits.
    DeleteChar(usize),
    DeleteCharBefore(usize),
    SubstituteChar(usize),
    SubstituteLine,
    DeleteToEnd,
    ChangeToEnd,
    YankLine(usize),
    JoinLines(usize, bool),
    ToggleCaseChar,

    // Insert mode entry variants.
    InsertAfter,
    InsertAtEnd,
    InsertAtFirstNonBlank,
    OpenBelow,
    OpenAbove,

    // Paste.
    PasteAfter(usize),
    PasteBefore(usize),
    PasteAfterCursorAtEnd(usize),
    PasteBeforeCursorAtEnd(usize),

    // Undo/redo.
    Undo(usize),
    Redo(usize),

    // Repeat.
    Repeat(usize),

    // Search.
    SearchForward,
    SearchBackward,
    SearchNext(usize),
    SearchPrev(usize),
    SearchWordForward,
    SearchWordBackward,
    SearchWordForwardPartial,
    SearchWordBackwardPartial,

    // Scrolling.
    ScrollHalfDown(usize),
    ScrollHalfUp(usize),
    ScrollPageDown(usize),
    ScrollPageUp(usize),
    ScrollLineDown(usize),
    ScrollLineUp(usize),
    ScrollCursorCenter,
    ScrollCursorTop,
    ScrollCursorBottom,
    ScrollCursorTopFirstNonBlank,
    ScrollCursorCenterFirstNonBlank,
    ScrollCursorBottomFirstNonBlank,

    // Jump list.
    JumpBack(usize),
    JumpForward(usize),

    // Change list.
    ChangeListOlder,
    ChangeListNewer,

    // Numbers.
    IncrementNumber(usize),
    DecrementNumber(usize),

    // Awaiting additional input.
    AwaitReplaceChar,
    AwaitFindChar(char),
    AwaitSetMark,
    AwaitGotoMark(bool),
    AwaitRegister,
    ToggleMacroRecord,
    AwaitPlayMacro,

    // Bracket matching.
    GotoPrevUnmatched(char),
    GotoNextUnmatched(char),

    // Write/quit shortcuts.
    WriteQuit,
    QuitNoSave,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_result_variants() {
        let _ = InputResult::Handled;
        let _ = InputResult::Pending;
    }
}
