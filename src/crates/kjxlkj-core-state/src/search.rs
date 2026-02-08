//! Search state and operations for the editor.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_text::BufferContent;

/// State for search operations.
#[derive(Debug, Default)]
pub struct SearchState {
    /// The current search pattern.
    pub pattern: Option<String>,
    /// Whether the last search was forward.
    pub forward: bool,
    /// All match positions (line, grapheme_offset).
    pub matches: Vec<(usize, usize)>,
    /// Index of the currently selected match.
    pub current_match: Option<usize>,
    /// Search history (most recent last).
    pub history: Vec<String>,
    /// History navigation index.
    pub history_pos: Option<usize>,
}

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a new search pattern and find all matches.
    pub fn search(
        &mut self,
        pattern: &str,
        content: &BufferContent,
        forward: bool,
    ) {
        self.pattern = Some(pattern.to_string());
        self.forward = forward;
        self.matches.clear();
        self.current_match = None;
        self.push_history(pattern);

        if pattern.is_empty() {
            return;
        }

        // Simple case-sensitive substring search.
        let pat_lower = pattern.to_lowercase();
        for line_idx in 0..content.line_count() {
            let line = content.line_content(line_idx);
            let line_lower = line.to_lowercase();
            let mut start = 0;
            while let Some(pos) =
                line_lower[start..].find(&pat_lower)
            {
                let byte_pos = start + pos;
                let grapheme_offset =
                    char_byte_to_grapheme(
                        &line, byte_pos,
                    );
                self.matches
                    .push((line_idx, grapheme_offset));
                start = byte_pos + pat_lower.len();
            }
        }
    }

    /// Jump to the next match from the cursor position.
    pub fn next_match(
        &mut self,
        cursor: &CursorPosition,
    ) -> Option<CursorPosition> {
        if self.matches.is_empty() {
            return None;
        }
        let pos = (cursor.line, cursor.grapheme_offset);
        let idx = if self.forward {
            self.matches
                .iter()
                .position(|m| *m > pos)
                .unwrap_or(0)
        } else {
            self.matches
                .iter()
                .rposition(|m| *m < pos)
                .unwrap_or(self.matches.len() - 1)
        };
        self.current_match = Some(idx);
        let (line, col) = self.matches[idx];
        Some(CursorPosition::new(line, col))
    }

    /// Jump to the previous match from the cursor.
    pub fn prev_match(
        &mut self,
        cursor: &CursorPosition,
    ) -> Option<CursorPosition> {
        if self.matches.is_empty() {
            return None;
        }
        let pos = (cursor.line, cursor.grapheme_offset);
        let idx = if self.forward {
            self.matches
                .iter()
                .rposition(|m| *m < pos)
                .unwrap_or(self.matches.len() - 1)
        } else {
            self.matches
                .iter()
                .position(|m| *m > pos)
                .unwrap_or(0)
        };
        self.current_match = Some(idx);
        let (line, col) = self.matches[idx];
        Some(CursorPosition::new(line, col))
    }

    /// Get current match count.
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Push a pattern to search history.
    pub fn push_history(&mut self, pattern: &str) {
        if pattern.is_empty() {
            return;
        }
        let s = pattern.to_string();
        // Deduplicate: remove if present.
        self.history.retain(|h| h != &s);
        self.history.push(s);
        // Cap at 100 entries.
        if self.history.len() > 100 {
            self.history.remove(0);
        }
        self.history_pos = None;
    }

    /// Navigate search history (true=older, false=newer).
    pub fn navigate_history(
        &mut self,
        older: bool,
    ) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }
        let pos = match self.history_pos {
            Some(p) => {
                if older {
                    p.saturating_sub(1)
                } else {
                    (p + 1).min(
                        self.history.len()
                            .saturating_sub(1),
                    )
                }
            }
            None => {
                if older {
                    self.history.len()
                        .saturating_sub(1)
                } else {
                    return None;
                }
            }
        };
        self.history_pos = Some(pos);
        Some(&self.history[pos])
    }
}

/// Convert a byte offset within a string to a grapheme
/// cluster index.
fn char_byte_to_grapheme(
    s: &str,
    byte_offset: usize,
) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    let mut offset = 0;
    for (idx, g) in s.graphemes(true).enumerate() {
        if offset >= byte_offset {
            return idx;
        }
        offset += g.len();
    }
    s.graphemes(true).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_matches() {
        let content =
            BufferContent::from_str("hello world\nhello\n");
        let mut state = SearchState::new();
        state.search("hello", &content, true);
        assert_eq!(state.match_count(), 2);
        assert_eq!(state.matches[0], (0, 0));
        assert_eq!(state.matches[1], (1, 0));
    }

    #[test]
    fn next_match_wraps() {
        let content =
            BufferContent::from_str("abc\nabc\n");
        let mut state = SearchState::new();
        state.search("abc", &content, true);
        let cursor = CursorPosition::new(1, 0);
        let next = state.next_match(&cursor);
        assert!(next.is_some());
        assert_eq!(next.unwrap().line, 0);
    }

    #[test]
    fn empty_pattern() {
        let content =
            BufferContent::from_str("test\n");
        let mut state = SearchState::new();
        state.search("", &content, true);
        assert_eq!(state.match_count(), 0);
    }
}
