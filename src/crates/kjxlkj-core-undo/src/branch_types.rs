//! Branch types.

/// Branch identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchId(pub u64);

/// Branch metadata.
#[derive(Debug, Clone)]
pub struct Branch {
    /// Branch identifier.
    pub id: BranchId,
    /// Branch name.
    pub name: Option<String>,
    /// Head sequence number.
    pub head_seq: u64,
    /// Number of changes.
    pub change_count: usize,
}

impl Branch {
    /// Creates a new branch.
    pub fn new(id: BranchId, head_seq: u64) -> Self {
        Self {
            id,
            name: None,
            head_seq,
            change_count: 0,
        }
    }

    /// Sets branch name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_new() {
        let b = Branch::new(BranchId(1), 10);
        assert_eq!(b.head_seq, 10);
        assert_eq!(b.id.0, 1);
    }

    #[test]
    fn test_branch_with_name() {
        let b = Branch::new(BranchId(1), 0).with_name("feature");
        assert_eq!(b.name, Some("feature".to_string()));
    }
}
