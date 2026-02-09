//! Japanese IME composition handling for insert mode.
//!
//! Manages pre-edit text, candidate selection, and
//! composition commit/cancel lifecycle.

/// IME composition state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImePhase {
    /// No composition in progress.
    Inactive,
    /// Pre-edit text being composed.
    Composing,
    /// Candidate selection window open.
    Selecting,
}

/// A conversion candidate from the IME.
#[derive(Debug, Clone)]
pub struct ImeCandidate {
    /// Display text of the candidate.
    pub text: String,
    /// Reading (kana representation).
    pub reading: Option<String>,
    /// Annotation or category.
    pub annotation: Option<String>,
}

/// IME composition state for insert mode.
#[derive(Debug, Clone, Default)]
pub struct ImeState {
    /// Current phase.
    pub phase: ImePhase,
    /// Pre-edit text (uncommitted).
    pub preedit: String,
    /// Cursor position within preedit.
    pub preedit_cursor: usize,
    /// Candidate list.
    pub candidates: Vec<ImeCandidate>,
    /// Selected candidate index.
    pub selected: usize,
    /// Committed text history.
    pub committed: Vec<String>,
}

impl Default for ImePhase {
    fn default() -> Self {
        ImePhase::Inactive
    }
}

impl ImeState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new composition.
    pub fn start_composition(&mut self, initial: &str) {
        self.phase = ImePhase::Composing;
        self.preedit = initial.to_string();
        self.preedit_cursor = initial.len();
        self.candidates.clear();
        self.selected = 0;
    }

    /// Update the preedit text.
    pub fn update_preedit(&mut self, text: &str) {
        self.preedit = text.to_string();
        self.preedit_cursor = text.len();
    }

    /// Set candidate list and enter selection phase.
    pub fn set_candidates(
        &mut self,
        candidates: Vec<ImeCandidate>,
    ) {
        self.candidates = candidates;
        self.selected = 0;
        if !self.candidates.is_empty() {
            self.phase = ImePhase::Selecting;
        }
    }

    /// Select next candidate.
    pub fn next_candidate(&mut self) {
        if !self.candidates.is_empty() {
            self.selected =
                (self.selected + 1) % self.candidates.len();
        }
    }

    /// Select previous candidate.
    pub fn prev_candidate(&mut self) {
        if !self.candidates.is_empty() {
            self.selected = if self.selected == 0 {
                self.candidates.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Commit the current selection or preedit text.
    pub fn commit(&mut self) -> String {
        let text = if let Some(cand) =
            self.candidates.get(self.selected)
        {
            cand.text.clone()
        } else {
            self.preedit.clone()
        };
        self.committed.push(text.clone());
        self.cancel();
        text
    }

    /// Cancel composition without committing.
    pub fn cancel(&mut self) {
        self.phase = ImePhase::Inactive;
        self.preedit.clear();
        self.preedit_cursor = 0;
        self.candidates.clear();
        self.selected = 0;
    }

    /// Whether composition is active.
    pub fn is_active(&self) -> bool {
        self.phase != ImePhase::Inactive
    }

    /// Get the currently selected candidate text.
    pub fn selected_text(&self) -> &str {
        if let Some(cand) = self.candidates.get(self.selected) {
            &cand.text
        } else {
            &self.preedit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_lifecycle() {
        let mut ime = ImeState::new();
        assert!(!ime.is_active());
        ime.start_composition("かんじ");
        assert!(ime.is_active());
        assert_eq!(ime.preedit, "かんじ");
        let text = ime.commit();
        assert_eq!(text, "かんじ");
        assert!(!ime.is_active());
    }

    #[test]
    fn candidate_selection() {
        let mut ime = ImeState::new();
        ime.start_composition("かんじ");
        ime.set_candidates(vec![
            ImeCandidate {
                text: "漢字".into(),
                reading: None,
                annotation: None,
            },
            ImeCandidate {
                text: "感じ".into(),
                reading: None,
                annotation: None,
            },
        ]);
        assert_eq!(ime.phase, ImePhase::Selecting);
        assert_eq!(ime.selected_text(), "漢字");
        ime.next_candidate();
        assert_eq!(ime.selected_text(), "感じ");
        let text = ime.commit();
        assert_eq!(text, "感じ");
    }

    #[test]
    fn cancel_composition() {
        let mut ime = ImeState::new();
        ime.start_composition("test");
        ime.cancel();
        assert!(!ime.is_active());
        assert!(ime.preedit.is_empty());
    }
}
