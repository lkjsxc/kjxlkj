//! Mode transition validation and tables.

use kjxlkj_core_types::Mode;

/// Whether a direct transition from `from` to `to` is valid.
pub fn is_valid_transition(from: Mode, to: Mode) -> bool {
    if from == to { return true; }
    matches!(
        (from, to),
        // Normal → anything
        (Mode::Normal, _)
        // Insert → Normal (Esc), InsertNormal (Ctrl-O)
        | (Mode::Insert, Mode::Normal)
        | (Mode::Insert, Mode::InsertNormal)
        // InsertNormal → Insert (after one command)
        | (Mode::InsertNormal, Mode::Insert)
        | (Mode::InsertNormal, Mode::Normal)
        // Visual modes → Normal (Esc), Command (:)
        | (Mode::Visual, Mode::Normal)
        | (Mode::Visual, Mode::Command)
        | (Mode::VisualLine, Mode::Normal)
        | (Mode::VisualLine, Mode::Command)
        | (Mode::VisualBlock, Mode::Normal)
        | (Mode::VisualBlock, Mode::Command)
        // Visual ↔ VisualLine ↔ VisualBlock
        | (Mode::Visual, Mode::VisualLine)
        | (Mode::Visual, Mode::VisualBlock)
        | (Mode::VisualLine, Mode::Visual)
        | (Mode::VisualLine, Mode::VisualBlock)
        | (Mode::VisualBlock, Mode::Visual)
        | (Mode::VisualBlock, Mode::VisualLine)
        // Visual/VLine/VBlock → Insert (change operator)
        | (Mode::Visual, Mode::Insert)
        | (Mode::VisualLine, Mode::Insert)
        | (Mode::VisualBlock, Mode::Insert)
        // Replace → Normal (Esc)
        | (Mode::Replace, Mode::Normal)
        // Command → Normal (Esc or Enter)
        | (Mode::Command, Mode::Normal)
        // OperatorPending → Normal (cancel or complete)
        | (Mode::OperatorPending, Mode::Normal)
        | (Mode::OperatorPending, Mode::Visual)
        // Terminal → Normal (Ctrl-\ Ctrl-N)
        | (Mode::Terminal, Mode::Normal)
    )
}

/// List all valid transitions from a given mode.
pub fn valid_targets(from: Mode) -> Vec<Mode> {
    let all = [
        Mode::Normal, Mode::Insert, Mode::Visual, Mode::VisualLine,
        Mode::VisualBlock, Mode::Replace, Mode::Command,
        Mode::OperatorPending, Mode::InsertNormal, Mode::Terminal,
    ];
    all.into_iter().filter(|&to| to != from && is_valid_transition(from, to)).collect()
}

/// Get the escape target for any mode (what Esc does).
pub fn escape_target(mode: Mode) -> Mode {
    match mode {
        Mode::Normal => Mode::Normal,
        Mode::Insert | Mode::Replace | Mode::Visual | Mode::VisualLine
        | Mode::VisualBlock | Mode::Command | Mode::OperatorPending
        | Mode::Terminal => Mode::Normal,
        Mode::InsertNormal => Mode::Insert,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_can_go_anywhere() {
        for m in valid_targets(Mode::Normal) {
            assert!(is_valid_transition(Mode::Normal, m));
        }
        assert!(valid_targets(Mode::Normal).len() >= 8);
    }

    #[test]
    fn insert_escapes_to_normal() {
        assert!(is_valid_transition(Mode::Insert, Mode::Normal));
        assert!(!is_valid_transition(Mode::Insert, Mode::Visual));
        assert_eq!(escape_target(Mode::Insert), Mode::Normal);
    }

    #[test]
    fn visual_modes_interchangeable() {
        assert!(is_valid_transition(Mode::Visual, Mode::VisualLine));
        assert!(is_valid_transition(Mode::VisualLine, Mode::VisualBlock));
        assert!(is_valid_transition(Mode::VisualBlock, Mode::Visual));
    }

    #[test]
    fn terminal_only_to_normal() {
        let targets = valid_targets(Mode::Terminal);
        assert_eq!(targets, vec![Mode::Normal]);
    }

    #[test]
    fn insert_normal_returns_to_insert() {
        assert_eq!(escape_target(Mode::InsertNormal), Mode::Insert);
        assert!(is_valid_transition(Mode::InsertNormal, Mode::Insert));
    }

    #[test]
    fn self_transitions_valid() {
        assert!(is_valid_transition(Mode::Normal, Mode::Normal));
        assert!(is_valid_transition(Mode::Insert, Mode::Insert));
    }

    #[test]
    fn operator_pending_exits() {
        assert!(is_valid_transition(Mode::OperatorPending, Mode::Normal));
        assert!(is_valid_transition(Mode::OperatorPending, Mode::Visual));
    }
}
