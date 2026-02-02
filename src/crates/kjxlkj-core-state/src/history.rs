//! History management for command and search history.

use crate::history_list::HistoryList;
use crate::HistoryType;

/// Complete history state.
#[derive(Debug, Clone, Default)]
pub struct History {
    /// Command history.
    pub command: HistoryList,
    /// Search history.
    pub search: HistoryList,
    /// Expression history.
    pub expression: HistoryList,
}

impl History {
    /// Creates new history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets history for a type.
    pub fn get(&self, typ: HistoryType) -> &HistoryList {
        match typ {
            HistoryType::Command => &self.command,
            HistoryType::Search => &self.search,
            HistoryType::Expression => &self.expression,
            _ => &self.command,
        }
    }

    /// Gets mutable history for a type.
    pub fn get_mut(&mut self, typ: HistoryType) -> &mut HistoryList {
        match typ {
            HistoryType::Command => &mut self.command,
            HistoryType::Search => &mut self.search,
            HistoryType::Expression => &mut self.expression,
            _ => &mut self.command,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_types() {
        let mut hist = History::new();
        hist.command.add("cmd", 1);
        hist.search.add("pattern", 2);

        assert_eq!(hist.get(HistoryType::Command).len(), 1);
        assert_eq!(hist.get(HistoryType::Search).len(), 1);
    }
}
