//! IME composition state model for Japanese input.
//!
//! Implements the composition lifecycle: Idle -> Preedit -> CandidateSelect -> Committed/Cancelled.
//! The model ensures leader key isolation during composition and correct
//! Esc/Enter/Space/Backspace routing.

/// IME composition states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    /// No active composition.
    Idle,
    /// Composing preedit text.
    Preedit,
    /// Navigating candidate list.
    CandidateSelect,
}

/// IME composition model.
#[derive(Debug, Clone)]
pub struct ImeComposition {
    pub state: ImeState,
    /// Current preedit string (uncommitted composition text).
    pub preedit: String,
    /// Candidate list (populated during CandidateSelect).
    pub candidates: Vec<String>,
    /// Active candidate index.
    pub candidate_index: usize,
}

impl ImeComposition {
    pub fn new() -> Self {
        Self {
            state: ImeState::Idle,
            preedit: String::new(),
            candidates: Vec::new(),
            candidate_index: 0,
        }
    }

    /// Whether the IME is actively composing (not idle).
    pub fn is_composing(&self) -> bool {
        self.state != ImeState::Idle
    }

    /// Start a new composition session.
    pub fn start_composition(&mut self) {
        self.state = ImeState::Preedit;
        self.preedit.clear();
        self.candidates.clear();
        self.candidate_index = 0;
    }

    /// Append a character to the preedit buffer.
    pub fn feed_preedit(&mut self, c: char) {
        self.preedit.push(c);
    }

    /// Backspace in preedit: remove last character.
    /// Returns false if preedit is now empty (composition should cancel).
    pub fn backspace_preedit(&mut self) -> bool {
        self.preedit.pop();
        !self.preedit.is_empty()
    }

    /// Enter candidate selection with the given candidates.
    pub fn enter_candidate_select(&mut self, candidates: Vec<String>) {
        if candidates.is_empty() {
            return;
        }
        self.candidates = candidates;
        self.candidate_index = 0;
        self.state = ImeState::CandidateSelect;
    }

    /// Cycle to the next candidate (Space during candidate selection).
    pub fn next_candidate(&mut self) {
        if !self.candidates.is_empty() {
            self.candidate_index = (self.candidate_index + 1) % self.candidates.len();
        }
    }

    /// Cycle to the previous candidate.
    pub fn prev_candidate(&mut self) {
        if !self.candidates.is_empty() {
            self.candidate_index = if self.candidate_index == 0 {
                self.candidates.len() - 1
            } else {
                self.candidate_index - 1
            };
        }
    }

    /// Get the current candidate text, or the preedit if no candidates.
    pub fn current_text(&self) -> &str {
        match self.state {
            ImeState::CandidateSelect => {
                if let Some(c) = self.candidates.get(self.candidate_index) {
                    c.as_str()
                } else {
                    &self.preedit
                }
            }
            _ => &self.preedit,
        }
    }

    /// Commit the current text: returns the committed string and resets state.
    pub fn commit(&mut self) -> String {
        let text = self.current_text().to_string();
        self.reset();
        text
    }

    /// Cancel the composition: returns to Idle without committing.
    pub fn cancel(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        self.state = ImeState::Idle;
        self.preedit.clear();
        self.candidates.clear();
        self.candidate_index = 0;
    }
}

impl Default for ImeComposition {
    fn default() -> Self {
        Self::new()
    }
}
