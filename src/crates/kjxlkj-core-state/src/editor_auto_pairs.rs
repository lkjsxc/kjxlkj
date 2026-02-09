//! Auto-pairs: automatically insert closing brackets
//! and quotes when opening ones are typed.

use crate::EditorState;

/// Characters that trigger auto-pair insertion.
const PAIRS: &[(char, char)] = &[
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('"', '"'),
    ('\'', '\''),
    ('`', '`'),
];

impl EditorState {
    /// Check if a character has an auto-pair and insert
    /// the closing character after the cursor.
    ///
    /// Returns true if an auto-pair was inserted.
    pub(crate) fn try_auto_pair(
        &mut self,
        ch: char,
    ) -> bool {
        let close = match find_closing(ch) {
            Some(c) => c,
            None => return false,
        };
        // Don't auto-pair quotes if cursor is after
        // a word character.
        if is_quote(ch) {
            let (line, col) = self.cursor_pos();
            if col > 0 {
                if let Some(buf) =
                    self.active_buffer()
                {
                    let line_text =
                        buf.content.line_content(line);
                    let prev = line_text
                        .chars()
                        .nth(col.saturating_sub(1));
                    if prev.map_or(false, |c| {
                        c.is_alphanumeric() || c == '_'
                    }) {
                        return false;
                    }
                }
            }
        }
        // Insert closing char after cursor position.
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            // Insert opening char.
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            buf.content.insert_char(off, ch);
            // Insert closing char right after.
            buf.content.insert_char(off + 1, close);
            buf.modified = true;
        }
        // Move cursor to between the pair.
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
        true
    }

    /// Skip over a closing character if it matches
    /// the character at the cursor. Returns true if
    /// skipped (caller should not insert).
    pub(crate) fn try_auto_pair_skip(
        &mut self,
        ch: char,
    ) -> bool {
        if !is_closing(ch) {
            return false;
        }
        let (line, col) = self.cursor_pos();
        let matches = self
            .active_buffer()
            .map(|buf| {
                let line_text =
                    buf.content.line_content(line);
                line_text.chars().nth(col) == Some(ch)
            })
            .unwrap_or(false);
        if matches {
            // Skip over the character.
            if let Some(w) = self.focused_window_mut()
            {
                w.cursor.grapheme_offset += 1;
            }
            true
        } else {
            false
        }
    }
}

fn find_closing(ch: char) -> Option<char> {
    PAIRS.iter().find(|p| p.0 == ch).map(|p| p.1)
}

fn is_closing(ch: char) -> bool {
    matches!(ch, ')' | ']' | '}' | '"' | '\'' | '`')
}

fn is_quote(ch: char) -> bool {
    matches!(ch, '"' | '\'' | '`')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair() {
        assert_eq!(find_closing('('), Some(')'));
        assert_eq!(find_closing('['), Some(']'));
        assert_eq!(find_closing('x'), None);
    }

    #[test]
    fn auto_pair_insert() {
        let mut state =
            crate::EditorState::new(80, 24);
        let paired = state.try_auto_pair('(');
        assert!(paired);
        let buf = state.active_buffer().unwrap();
        let line = buf.content.line_content(0);
        assert_eq!(line, "()");
    }
}
