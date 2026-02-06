//! Quickfix and location list scaffolding.

/// An entry in the quickfix or location list.
#[derive(Debug, Clone)]
pub struct QuickfixEntry {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub text: String,
    pub kind: QuickfixKind,
}

/// Kind of quickfix entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickfixKind { Error, Warning, Info, Note }

/// The quickfix list state.
#[derive(Debug, Clone, Default)]
pub struct QuickfixList {
    pub entries: Vec<QuickfixEntry>,
    pub current: usize,
    pub title: String,
}

impl QuickfixList {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, entries: Vec<QuickfixEntry>, title: &str) {
        self.entries = entries;
        self.current = 0;
        self.title = title.to_string();
    }

    pub fn next(&mut self) -> Option<&QuickfixEntry> {
        if self.entries.is_empty() { return None; }
        if self.current < self.entries.len() - 1 { self.current += 1; }
        self.entries.get(self.current)
    }

    pub fn prev(&mut self) -> Option<&QuickfixEntry> {
        if self.entries.is_empty() { return None; }
        if self.current > 0 { self.current -= 1; }
        self.entries.get(self.current)
    }

    pub fn current_entry(&self) -> Option<&QuickfixEntry> {
        self.entries.get(self.current)
    }

    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entries() -> Vec<QuickfixEntry> {
        vec![
            QuickfixEntry { file: "a.rs".into(), line: 1, col: 0, text: "error 1".into(), kind: QuickfixKind::Error },
            QuickfixEntry { file: "b.rs".into(), line: 5, col: 3, text: "warning".into(), kind: QuickfixKind::Warning },
            QuickfixEntry { file: "c.rs".into(), line: 10, col: 0, text: "note".into(), kind: QuickfixKind::Note },
        ]
    }

    #[test]
    fn quickfix_navigation() {
        let mut qf = QuickfixList::new();
        qf.set(sample_entries(), "test");
        assert_eq!(qf.current, 0);
        assert_eq!(qf.current_entry().unwrap().file, "a.rs");
        qf.next();
        assert_eq!(qf.current_entry().unwrap().file, "b.rs");
        qf.next();
        assert_eq!(qf.current_entry().unwrap().file, "c.rs");
        qf.next(); // at end, stays
        assert_eq!(qf.current_entry().unwrap().file, "c.rs");
        qf.prev();
        assert_eq!(qf.current_entry().unwrap().file, "b.rs");
    }

    #[test]
    fn quickfix_empty() {
        let mut qf = QuickfixList::new();
        assert!(qf.is_empty());
        assert!(qf.next().is_none());
        assert!(qf.prev().is_none());
    }

    #[test]
    fn quickfix_set_resets_index() {
        let mut qf = QuickfixList::new();
        qf.set(sample_entries(), "first");
        qf.next();
        qf.set(sample_entries(), "second");
        assert_eq!(qf.current, 0);
        assert_eq!(qf.title, "second");
    }
}
