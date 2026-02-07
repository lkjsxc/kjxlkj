//! Change list: tracks positions where changes occurred for g; / g, navigation.

use kjxlkj_core_types::Position;

/// A single change-list entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeListEntry {
    pub position: Position,
}

/// The change list with older/newer navigation.
#[derive(Debug, Clone)]
pub struct ChangeList {
    entries: Vec<ChangeListEntry>,
    current: usize,
}

impl ChangeList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current: 0,
        }
    }

    /// Record a new change position.
    pub fn push(&mut self, entry: ChangeListEntry) {
        self.entries.push(entry);
        self.current = self.entries.len();
    }

    /// Navigate to older change (g;).
    pub fn older(&mut self) -> Option<&ChangeListEntry> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        self.entries.get(self.current)
    }

    /// Navigate to newer change (g,).
    pub fn newer(&mut self) -> Option<&ChangeListEntry> {
        if self.current >= self.entries.len() {
            return None;
        }
        let entry = self.entries.get(self.current);
        self.current += 1;
        entry
    }

    pub fn entries(&self) -> &[ChangeListEntry] {
        &self.entries
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for ChangeList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_older() {
        let mut cl = ChangeList::new();
        cl.push(ChangeListEntry {
            position: Position::new(3, 0),
        });
        cl.push(ChangeListEntry {
            position: Position::new(7, 0),
        });
        let e = cl.older().unwrap();
        assert_eq!(e.position, Position::new(7, 0));
    }

    #[test]
    fn newer_after_older() {
        let mut cl = ChangeList::new();
        cl.push(ChangeListEntry {
            position: Position::new(3, 0),
        });
        cl.push(ChangeListEntry {
            position: Position::new(7, 0),
        });
        cl.older();
        let e = cl.newer().unwrap();
        assert_eq!(e.position, Position::new(7, 0));
    }

    #[test]
    fn empty_older_returns_none() {
        let mut cl = ChangeList::new();
        assert!(cl.older().is_none());
    }
}
