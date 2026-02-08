//! Treesitter text objects: syntax-aware selection
//! boundaries using tree-sitter parse trees.

/// Node kind for treesitter text objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsNodeKind {
    /// Function/method definition.
    Function,
    /// Class/struct/impl definition.
    Class,
    /// If/else block.
    Conditional,
    /// Loop (for, while, loop).
    Loop,
    /// Block/scope.
    Block,
    /// Comment (single or multi-line).
    Comment,
    /// Parameter/argument list.
    Parameter,
    /// Return statement.
    Return,
    /// Assignment.
    Assignment,
    /// Generic named node.
    Named(String),
}

/// A treesitter text object span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsSpan {
    /// Start line (0-indexed).
    pub start_line: usize,
    /// Start column (grapheme offset).
    pub start_col: usize,
    /// End line.
    pub end_line: usize,
    /// End column.
    pub end_col: usize,
}

/// Treesitter text object state.
#[derive(Debug, Clone, Default)]
pub struct TsTextObjects {
    /// Whether treesitter is available for current buffer.
    pub available: bool,
    /// Cached node kinds at cursor position.
    pub nodes_at_cursor: Vec<(TsNodeKind, TsSpan)>,
    /// Language of current buffer.
    pub language: Option<String>,
}

impl TsTextObjects {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set availability.
    pub fn set_available(
        &mut self,
        lang: &str,
        available: bool,
    ) {
        self.available = available;
        self.language = if available {
            Some(lang.to_string())
        } else {
            None
        };
    }

    /// Update cached nodes at cursor. In a real implementation,
    /// this would query the tree-sitter parse tree.
    pub fn update_nodes(
        &mut self,
        nodes: Vec<(TsNodeKind, TsSpan)>,
    ) {
        self.nodes_at_cursor = nodes;
    }

    /// Find the innermost node of a specific kind.
    pub fn find_innermost(
        &self,
        kind: &TsNodeKind,
    ) -> Option<&TsSpan> {
        // Nodes are ordered outer-to-inner, so reverse search
        self.nodes_at_cursor
            .iter()
            .rev()
            .find(|(k, _)| k == kind)
            .map(|(_, span)| span)
    }

    /// Find the outermost node of a specific kind.
    pub fn find_outermost(
        &self,
        kind: &TsNodeKind,
    ) -> Option<&TsSpan> {
        self.nodes_at_cursor
            .iter()
            .find(|(k, _)| k == kind)
            .map(|(_, span)| span)
    }

    /// Get all function nodes.
    pub fn functions(&self) -> Vec<&TsSpan> {
        self.nodes_at_cursor
            .iter()
            .filter(|(k, _)| *k == TsNodeKind::Function)
            .map(|(_, s)| s)
            .collect()
    }

    /// Get all class nodes.
    pub fn classes(&self) -> Vec<&TsSpan> {
        self.nodes_at_cursor
            .iter()
            .filter(|(k, _)| *k == TsNodeKind::Class)
            .map(|(_, s)| s)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_innermost_function() {
        let mut ts = TsTextObjects::new();
        ts.update_nodes(vec![
            (
                TsNodeKind::Class,
                TsSpan {
                    start_line: 0,
                    start_col: 0,
                    end_line: 20,
                    end_col: 1,
                },
            ),
            (
                TsNodeKind::Function,
                TsSpan {
                    start_line: 5,
                    start_col: 4,
                    end_line: 10,
                    end_col: 5,
                },
            ),
        ]);
        let span = ts.find_innermost(&TsNodeKind::Function);
        assert!(span.is_some());
        assert_eq!(span.unwrap().start_line, 5);
    }

    #[test]
    fn no_treesitter() {
        let ts = TsTextObjects::new();
        assert!(!ts.available);
        assert!(ts.find_innermost(&TsNodeKind::Function).is_none());
    }

    #[test]
    fn set_availability() {
        let mut ts = TsTextObjects::new();
        ts.set_available("rust", true);
        assert!(ts.available);
        assert_eq!(ts.language.as_deref(), Some("rust"));
    }
}
