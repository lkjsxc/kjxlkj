//! IME tests: JP-01 through JP-05 and routing tests.

#[cfg(test)]
mod tests {
    use crate::ime::ImeComposition;
    use crate::ime_route::{route_ime_key, ImeResult};

    #[test]
    fn jp01_composition_commit() {
        let mut ime = ImeComposition::new();
        ime.start_composition();
        assert!(ime.is_composing());
        ime.feed_preedit('に');
        ime.feed_preedit('ほ');
        ime.feed_preedit('ん');
        assert_eq!(ime.current_text(), "にほん");
        let committed = ime.commit();
        assert_eq!(committed, "にほん");
        assert!(!ime.is_composing());
    }

    #[test]
    fn jp02_cancel_leaves_buffer_unchanged() {
        let mut ime = ImeComposition::new();
        ime.start_composition();
        ime.feed_preedit('あ');
        ime.cancel();
        assert!(!ime.is_composing());
        assert_eq!(ime.preedit, "");
    }

    #[test]
    fn jp03_space_does_not_leak_to_leader() {
        let mut ime = ImeComposition::new();
        ime.start_composition();
        ime.enter_candidate_select(vec!["日本".into(), "にほん".into()]);
        let result = route_ime_key(&mut ime, &kjxlkj_core_types::KeyCode::Char(' '));
        assert_eq!(result, ImeResult::Consumed);
        assert_eq!(ime.candidate_index, 1);
    }

    #[test]
    fn jp04_candidate_cycling() {
        let mut ime = ImeComposition::new();
        ime.start_composition();
        ime.feed_preedit('に');
        ime.enter_candidate_select(vec!["日".into(), "二".into(), "荷".into()]);
        assert_eq!(ime.current_text(), "日");
        ime.next_candidate();
        assert_eq!(ime.current_text(), "二");
        ime.next_candidate();
        assert_eq!(ime.current_text(), "荷");
        ime.next_candidate();
        assert_eq!(ime.current_text(), "日"); // wraps
    }

    #[test]
    fn jp05_backspace_preedit() {
        let mut ime = ImeComposition::new();
        ime.start_composition();
        ime.feed_preedit('あ');
        ime.feed_preedit('い');
        assert!(ime.backspace_preedit());
        assert_eq!(ime.preedit, "あ");
        assert!(!ime.backspace_preedit()); // last char removed -> cancel
    }

    #[test]
    fn jp_idle_passthrough() {
        let mut ime = ImeComposition::new();
        let result = route_ime_key(&mut ime, &kjxlkj_core_types::KeyCode::Char('a'));
        assert_eq!(result, ImeResult::PassThrough);
    }
}
