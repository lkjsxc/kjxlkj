//! Command-line completion for ex commands.

pub use crate::completion_registry::CommandRegistry;
pub use crate::completion_types::{Candidate, CompletionSource};

/// Command completion state.
#[derive(Debug, Default)]
pub struct CommandCompletion {
    candidates: Vec<Candidate>,
    selected: Option<usize>,
    original: String,
    source: Option<CompletionSource>,
}

impl CommandCompletion {
    /// Creates a new command completion.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the candidates.
    pub fn set_candidates(&mut self, candidates: Vec<Candidate>, original: &str) {
        self.candidates = candidates;
        self.original = original.to_string();
        self.selected = if self.candidates.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    /// Returns the candidates.
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// Returns the selected candidate.
    pub fn selected(&self) -> Option<&Candidate> {
        self.selected.and_then(|i| self.candidates.get(i))
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Selects the next candidate.
    pub fn select_next(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % self.candidates.len(),
            None => 0,
        });
    }

    /// Selects the previous candidate.
    pub fn prev(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(0) => self.candidates.len() - 1,
            Some(i) => i - 1,
            None => self.candidates.len() - 1,
        });
    }

    /// Confirms the selection.
    pub fn confirm(&self) -> Option<String> {
        self.selected().map(|c| c.text.clone())
    }

    /// Cancels completion, returning original text.
    pub fn cancel(&self) -> &str {
        &self.original
    }

    /// Clears the completion state.
    pub fn clear(&mut self) {
        self.candidates.clear();
        self.selected = None;
        self.original.clear();
        self.source = None;
    }

    /// Returns whether completion is active.
    pub fn is_active(&self) -> bool {
        !self.candidates.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_new() {
        let c = Candidate::new("test");
        assert_eq!(c.text, "test");
    }

    #[test]
    fn test_completion_set_candidates() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(vec![Candidate::new("edit"), Candidate::new("exit")], "e");
        assert_eq!(cc.candidates().len(), 2);
        assert_eq!(cc.selected_index(), Some(0));
    }

    #[test]
    fn test_completion_next() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(
            vec![Candidate::new("a"), Candidate::new("b"), Candidate::new("c")],
            "",
        );
        cc.select_next();
        assert_eq!(cc.selected().unwrap().text, "b");
    }

    #[test]
    fn test_completion_prev() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(vec![Candidate::new("a"), Candidate::new("b")], "");
        cc.prev();
        assert_eq!(cc.selected().unwrap().text, "b");
    }
}
