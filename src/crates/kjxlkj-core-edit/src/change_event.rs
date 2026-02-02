//! Text change events.
//!
//! Events emitted when buffer text changes.

use std::ops::Range;

/// Type of text change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// Text inserted.
    Insert,
    /// Text deleted.
    Delete,
    /// Text replaced.
    Replace,
}

/// A text change event.
#[derive(Debug, Clone)]
pub struct TextChange {
    /// Kind of change.
    pub kind: ChangeKind,
    /// Byte range affected.
    pub range: Range<usize>,
    /// Old text (for delete/replace).
    pub old_text: String,
    /// New text (for insert/replace).
    pub new_text: String,
    /// Tick at which change occurred.
    pub tick: u64,
}

impl TextChange {
    /// Creates an insert change.
    pub fn insert(position: usize, text: &str, tick: u64) -> Self {
        Self {
            kind: ChangeKind::Insert,
            range: position..position,
            old_text: String::new(),
            new_text: text.to_string(),
            tick,
        }
    }

    /// Creates a delete change.
    pub fn delete(range: Range<usize>, old_text: &str, tick: u64) -> Self {
        Self {
            kind: ChangeKind::Delete,
            range,
            old_text: old_text.to_string(),
            new_text: String::new(),
            tick,
        }
    }

    /// Creates a replace change.
    pub fn replace(range: Range<usize>, old_text: &str, new_text: &str, tick: u64) -> Self {
        Self {
            kind: ChangeKind::Replace,
            range,
            old_text: old_text.to_string(),
            new_text: new_text.to_string(),
            tick,
        }
    }

    /// Returns the inverse change (for undo).
    pub fn inverse(&self) -> Self {
        match self.kind {
            ChangeKind::Insert => Self {
                kind: ChangeKind::Delete,
                range: self.range.start..self.range.start + self.new_text.len(),
                old_text: self.new_text.clone(),
                new_text: String::new(),
                tick: self.tick,
            },
            ChangeKind::Delete => Self {
                kind: ChangeKind::Insert,
                range: self.range.start..self.range.start,
                old_text: String::new(),
                new_text: self.old_text.clone(),
                tick: self.tick,
            },
            ChangeKind::Replace => Self {
                kind: ChangeKind::Replace,
                range: self.range.start..self.range.start + self.new_text.len(),
                old_text: self.new_text.clone(),
                new_text: self.old_text.clone(),
                tick: self.tick,
            },
        }
    }

    /// Returns byte offset of the change.
    pub fn offset(&self) -> usize {
        self.range.start
    }

    /// Returns the net byte delta.
    pub fn delta(&self) -> isize {
        self.new_text.len() as isize - self.old_text.len() as isize
    }
}

/// Change listener callback type.
pub type ChangeListener = Box<dyn Fn(&TextChange) + Send + Sync>;

/// Change event dispatcher.
#[derive(Default)]
pub struct ChangeDispatcher {
    /// Registered listeners.
    listeners: Vec<ChangeListener>,
}

impl ChangeDispatcher {
    /// Creates new dispatcher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a listener.
    pub fn add_listener(&mut self, listener: ChangeListener) {
        self.listeners.push(listener);
    }

    /// Dispatches a change to all listeners.
    pub fn dispatch(&self, change: &TextChange) {
        for listener in &self.listeners {
            listener(change);
        }
    }

    /// Returns listener count.
    pub fn listener_count(&self) -> usize {
        self.listeners.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_change_insert() {
        let c = TextChange::insert(10, "hello", 1);
        assert_eq!(c.kind, ChangeKind::Insert);
        assert_eq!(c.delta(), 5);
    }

    #[test]
    fn test_text_change_delete() {
        let c = TextChange::delete(10..15, "hello", 1);
        assert_eq!(c.kind, ChangeKind::Delete);
        assert_eq!(c.delta(), -5);
    }

    #[test]
    fn test_text_change_replace() {
        let c = TextChange::replace(10..15, "hello", "hi", 1);
        assert_eq!(c.kind, ChangeKind::Replace);
        assert_eq!(c.delta(), -3);
    }

    #[test]
    fn test_text_change_inverse_insert() {
        let c = TextChange::insert(10, "hello", 1);
        let inv = c.inverse();
        assert_eq!(inv.kind, ChangeKind::Delete);
    }

    #[test]
    fn test_text_change_inverse_delete() {
        let c = TextChange::delete(10..15, "hello", 1);
        let inv = c.inverse();
        assert_eq!(inv.kind, ChangeKind::Insert);
    }

    #[test]
    fn test_change_dispatcher() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let count = Arc::new(AtomicUsize::new(0));
        let count2 = count.clone();

        let mut disp = ChangeDispatcher::new();
        disp.add_listener(Box::new(move |_| {
            count2.fetch_add(1, Ordering::SeqCst);
        }));

        disp.dispatch(&TextChange::insert(0, "a", 1));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
