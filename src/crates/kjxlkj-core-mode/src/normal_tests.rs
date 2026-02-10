//! Tests for normal mode dispatch.

#[cfg(test)]
mod tests {
    use crate::dispatch::{DispatchResult, ModeDispatcher};
    use crate::normal::dispatch_normal;
    use kjxlkj_core_types::{CommandKind, Key, Mode};

    #[test]
    fn test_normal_i_enters_insert() {
        let mut disp = ModeDispatcher::new();
        let result = dispatch_normal(&mut disp, &Key::char('i'));
        assert!(matches!(result, DispatchResult::ModeChange(Mode::Insert)));
    }

    #[test]
    fn test_normal_colon_enters_command() {
        let mut disp = ModeDispatcher::new();
        let result = dispatch_normal(&mut disp, &Key::char(':'));
        assert!(matches!(
            result,
            DispatchResult::ModeChange(Mode::Command(CommandKind::Ex))
        ));
    }

    #[test]
    fn test_count_accumulation() {
        let mut disp = ModeDispatcher::new();
        let r1 = dispatch_normal(&mut disp, &Key::char('3'));
        assert!(matches!(r1, DispatchResult::Pending));
        assert_eq!(disp.count, Some(3));
    }
}
