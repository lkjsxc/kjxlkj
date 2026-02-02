//! Undo branch navigation.
//!
//! Navigates between undo branches.

use std::collections::VecDeque;

use crate::branch_types::{Branch, BranchId};

/// Branch manager.
#[derive(Debug, Clone, Default)]
pub struct BranchManager {
    /// All branches.
    branches: Vec<Branch>,
    /// Current branch.
    current: Option<BranchId>,
    /// Next branch ID.
    next_id: u64,
}

impl BranchManager {
    /// Creates new branch manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new branch.
    pub fn create(&mut self, head_seq: u64) -> BranchId {
        let id = BranchId(self.next_id);
        self.next_id += 1;
        self.branches.push(Branch::new(id, head_seq));
        if self.current.is_none() {
            self.current = Some(id);
        }
        id
    }

    /// Switches to a branch.
    pub fn switch(&mut self, id: BranchId) -> bool {
        if self.branches.iter().any(|b| b.id == id) {
            self.current = Some(id);
            true
        } else {
            false
        }
    }

    /// Returns current branch.
    pub fn current(&self) -> Option<&Branch> {
        self.current
            .and_then(|id| self.branches.iter().find(|b| b.id == id))
    }

    /// Returns all branches.
    pub fn all(&self) -> &[Branch] {
        &self.branches
    }

    /// Returns branch count.
    pub fn len(&self) -> usize {
        self.branches.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }

    /// Deletes a branch.
    pub fn delete(&mut self, id: BranchId) -> bool {
        if let Some(pos) = self.branches.iter().position(|b| b.id == id) {
            self.branches.remove(pos);
            if self.current == Some(id) {
                self.current = self.branches.first().map(|b| b.id);
            }
            true
        } else {
            false
        }
    }
}

/// Undo earlier/later time navigation.
#[derive(Debug, Clone, Default)]
pub struct TimeTravel {
    /// History of visited states.
    history: VecDeque<u64>,
    /// Current position.
    position: usize,
}

impl TimeTravel {
    /// Creates new time travel state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a state.
    pub fn record(&mut self, seq: u64) {
        self.history.push_back(seq);
        self.position = self.history.len().saturating_sub(1);
    }

    /// Goes earlier.
    pub fn earlier(&mut self) -> Option<u64> {
        if self.position > 0 {
            self.position -= 1;
            self.history.get(self.position).copied()
        } else {
            None
        }
    }

    /// Goes later.
    pub fn later(&mut self) -> Option<u64> {
        if self.position + 1 < self.history.len() {
            self.position += 1;
            self.history.get(self.position).copied()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_manager_create() {
        let mut mgr = BranchManager::new();
        let _id = mgr.create(0);
        assert_eq!(mgr.len(), 1);
        assert!(mgr.current().is_some());
    }

    #[test]
    fn test_branch_manager_switch() {
        let mut mgr = BranchManager::new();
        let id1 = mgr.create(0);
        let _id2 = mgr.create(10);
        assert!(mgr.switch(id1));
    }

    #[test]
    fn test_branch_manager_delete() {
        let mut mgr = BranchManager::new();
        let id = mgr.create(0);
        assert!(mgr.delete(id));
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_time_travel_record() {
        let mut tt = TimeTravel::new();
        tt.record(1);
        tt.record(2);
        assert_eq!(tt.history.len(), 2);
    }

    #[test]
    fn test_time_travel_navigation() {
        let mut tt = TimeTravel::new();
        tt.record(1);
        tt.record(2);
        assert_eq!(tt.earlier(), Some(1));
        assert_eq!(tt.later(), Some(2));
    }
}
