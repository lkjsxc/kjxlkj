//! Operator-pending mode handling.
//!
//! This module handles input after an operator (d, y, c, etc.) is pressed.

mod apply;

pub use apply::apply_operator_motion;

use crossterm::event::KeyCode;
use kjxlkj_core_types::intent::Operator;

/// Result of handling a key in operator-pending mode.
pub enum OperatorPendingAction {
    /// Continue waiting for motion.
    Continue,
    /// Complete operator with motion.
    Complete(OperatorMotion),
    /// Cancel operator (Esc).
    Cancel,
}

/// Motion received in operator-pending mode.
#[derive(Debug, Clone)]
pub struct OperatorMotion {
    pub operator: Operator,
    pub count: Option<usize>,
    pub motion: Motion,
}

/// Motion types for operators.
#[derive(Debug, Clone, Copy)]
pub enum Motion {
    /// Character-wise (h, l)
    Left,
    Right,
    /// Line-wise (j, k)
    Down,
    Up,
    /// Word-wise (w, e, b)
    Word,
    WordEnd,
    WordBack,
    /// Line motions (0, ^, $)
    LineStart,
    FirstNonBlank,
    LineEnd,
    /// Whole line (dd, yy, cc)
    WholeLine,
    /// Text objects
    InnerWord,
    AroundWord,
}

/// Handles a key press in operator-pending mode.
pub fn handle_operator_pending_key(
    code: KeyCode,
    _buffer: &mut kjxlkj_core::Buffer,
    operator: Operator,
    count: Option<usize>,
) -> OperatorPendingAction {
    match code {
        KeyCode::Esc => OperatorPendingAction::Cancel,

        // Repeat operator for whole line (dd, yy, cc)
        KeyCode::Char('d') if operator == Operator::Delete => {
            OperatorPendingAction::Complete(OperatorMotion {
                operator,
                count,
                motion: Motion::WholeLine,
            })
        }
        KeyCode::Char('y') if operator == Operator::Yank => {
            OperatorPendingAction::Complete(OperatorMotion {
                operator,
                count,
                motion: Motion::WholeLine,
            })
        }
        KeyCode::Char('c') if operator == Operator::Change => {
            OperatorPendingAction::Complete(OperatorMotion {
                operator,
                count,
                motion: Motion::WholeLine,
            })
        }

        // Motions
        KeyCode::Char('h') | KeyCode::Left => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::Left,
        }),
        KeyCode::Char('l') | KeyCode::Right => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::Right,
        }),
        KeyCode::Char('j') | KeyCode::Down => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::Down,
        }),
        KeyCode::Char('k') | KeyCode::Up => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::Up,
        }),
        KeyCode::Char('w') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::Word,
        }),
        KeyCode::Char('e') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::WordEnd,
        }),
        KeyCode::Char('b') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::WordBack,
        }),
        KeyCode::Char('0') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::LineStart,
        }),
        KeyCode::Char('^') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::FirstNonBlank,
        }),
        KeyCode::Char('$') => OperatorPendingAction::Complete(OperatorMotion {
            operator,
            count,
            motion: Motion::LineEnd,
        }),

        // Inner text objects
        KeyCode::Char('i') => {
            // Next key will be the text object type
            OperatorPendingAction::Continue
        }
        KeyCode::Char('a') => {
            // Next key will be the text object type
            OperatorPendingAction::Continue
        }

        _ => OperatorPendingAction::Continue,
    }
}
