//! Flash/EasyMotion jump navigation.
//!
//! Per /docs/spec/features/navigation/flash.md:
//! Displays labels on jump targets, user types label to jump.

use crate::EditorState;

/// A labeled jump target.
#[derive(Debug, Clone)]
pub struct FlashTarget {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column (grapheme offset).
    pub col: usize,
    /// Label character(s) to type.
    pub label: String,
}

/// Flash jump state.
#[derive(Debug, Clone, Default)]
pub struct FlashState {
    /// Whether flash mode is active.
    pub active: bool,
    /// Available jump targets.
    pub targets: Vec<FlashTarget>,
    /// Characters typed so far for label.
    pub input: String,
}

impl FlashState {
    /// Create new empty flash state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start flash mode with targets.
    pub fn start(&mut self, targets: Vec<FlashTarget>) {
        self.active = true;
        self.targets = targets;
        self.input.clear();
    }

    /// Cancel flash mode.
    pub fn cancel(&mut self) {
        self.active = false;
        self.targets.clear();
        self.input.clear();
    }

    /// Feed a character and return matching target.
    pub fn feed(&mut self, c: char) -> Option<FlashTarget> {
        self.input.push(c);
        let matching: Vec<_> = self
            .targets
            .iter()
            .filter(|t| t.label.starts_with(&self.input))
            .cloned()
            .collect();
        if matching.len() == 1 && matching[0].label == self.input {
            let target = matching[0].clone();
            self.cancel();
            return Some(target);
        }
        if matching.is_empty() {
            self.cancel();
        }
        None
    }
}

/// Label chars for generating labels.
const LABEL_CHARS: &[u8] = b"asdghklqwertyuiopzxcvbnmfj";

impl EditorState {
    /// Activate flash jump mode.
    pub fn do_flash_jump(&mut self) {
        let Some(win) = self.focused_window() else {
            return;
        };
        let buf_id = match win.buffer_id() {
            Some(id) => id,
            None => return,
        };
        let top = win.viewport.top_line;
        let height = win.viewport.height as usize;

        let buf = match self.buffers.get(&buf_id) {
            Some(b) => b,
            None => return,
        };
        let end = (top + height).min(buf.content.line_count());

        let mut targets = Vec::new();
        let mut label_idx = 0usize;

        for line in top..end {
            let text = buf.content.line_str(line);
            // Target word beginnings.
            let mut prev_space = true;
            for (col, ch) in text.char_indices() {
                let is_space = ch.is_whitespace();
                if prev_space && !is_space {
                    if label_idx < LABEL_CHARS.len() {
                        targets.push(FlashTarget {
                            line,
                            col,
                            label: String::from(LABEL_CHARS[label_idx] as char),
                        });
                        label_idx += 1;
                    }
                }
                prev_space = is_space;
            }
        }

        self.flash_state.start(targets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_feed_match() {
        let mut state = FlashState::new();
        state.start(vec![
            FlashTarget {
                line: 0,
                col: 0,
                label: "a".into(),
            },
            FlashTarget {
                line: 1,
                col: 5,
                label: "s".into(),
            },
        ]);
        assert!(state.active);
        let result = state.feed('a');
        assert!(result.is_some());
        assert_eq!(result.unwrap().line, 0);
        assert!(!state.active);
    }

    #[test]
    fn flash_feed_no_match() {
        let mut state = FlashState::new();
        state.start(vec![FlashTarget {
            line: 0,
            col: 0,
            label: "a".into(),
        }]);
        let result = state.feed('z');
        assert!(result.is_none());
        assert!(!state.active);
    }
}
