#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    TerminalInsert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalResolvedAction {
    EnterInsertAtCursor,
    EnterInsertAfterCursor,
    EnterInsertAtEol,
    Quit,
    Ignore,
}

pub fn resolve_normal_char(ch: char) -> NormalResolvedAction {
    match ch {
        'i' => NormalResolvedAction::EnterInsertAtCursor,
        'a' => NormalResolvedAction::EnterInsertAfterCursor,
        'A' => NormalResolvedAction::EnterInsertAtEol,
        'q' => NormalResolvedAction::Quit,
        '\u{3}' => NormalResolvedAction::Quit,
        _ => NormalResolvedAction::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase_a_is_append_after_cursor() {
        assert_eq!(
            resolve_normal_char('a'),
            NormalResolvedAction::EnterInsertAfterCursor
        );
    }

    #[test]
    fn uppercase_a_is_append_at_eol() {
        assert_eq!(
            resolve_normal_char('A'),
            NormalResolvedAction::EnterInsertAtEol
        );
    }

    #[test]
    fn i_is_insert_at_cursor() {
        assert_eq!(
            resolve_normal_char('i'),
            NormalResolvedAction::EnterInsertAtCursor
        );
    }

    #[test]
    fn q_is_quit_command() {
        assert_eq!(resolve_normal_char('q'), NormalResolvedAction::Quit);
    }
}
