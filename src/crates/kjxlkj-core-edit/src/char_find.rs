//! Character find motion (f/F/t/T) state and execution.

/// Kind of character find.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharFindKind {
    /// `f` — find forward (inclusive).
    ForwardTo,
    /// `F` — find backward (inclusive).
    BackwardTo,
    /// `t` — till forward (exclusive: stop before).
    ForwardTill,
    /// `T` — till backward (exclusive: stop after).
    BackwardTill,
}

/// State for repeating character finds (`;` and `,`).
#[derive(Debug, Clone)]
pub struct CharFind {
    /// The character to find.
    pub ch: char,
    /// The kind of find.
    pub kind: CharFindKind,
}

impl CharFind {
    /// Create a new character find.
    pub fn new(ch: char, kind: CharFindKind) -> Self {
        Self { ch, kind }
    }

    /// Execute the find on a line string from a starting grapheme index.
    ///
    /// Returns the grapheme index of the match, or None.
    pub fn execute(&self, line: &str, from_idx: usize) -> Option<usize> {
        use unicode_segmentation::UnicodeSegmentation;
        let graphemes: Vec<&str> = line.graphemes(true).collect();

        match self.kind {
            CharFindKind::ForwardTo | CharFindKind::ForwardTill => {
                for i in (from_idx + 1)..graphemes.len() {
                    if grapheme_starts_with(graphemes[i], self.ch) {
                        return if self.kind == CharFindKind::ForwardTill {
                            if i > 0 { Some(i - 1) } else { Some(0) }
                        } else {
                            Some(i)
                        };
                    }
                }
                None
            }
            CharFindKind::BackwardTo | CharFindKind::BackwardTill => {
                for i in (0..from_idx).rev() {
                    if grapheme_starts_with(graphemes[i], self.ch) {
                        return if self.kind == CharFindKind::BackwardTill {
                            if i + 1 < graphemes.len() {
                                Some(i + 1)
                            } else {
                                Some(i)
                            }
                        } else {
                            Some(i)
                        };
                    }
                }
                None
            }
        }
    }

    /// Return the reversed find (for `,` repeat).
    pub fn reversed(&self) -> Self {
        let kind = match self.kind {
            CharFindKind::ForwardTo => CharFindKind::BackwardTo,
            CharFindKind::BackwardTo => CharFindKind::ForwardTo,
            CharFindKind::ForwardTill => CharFindKind::BackwardTill,
            CharFindKind::BackwardTill => CharFindKind::ForwardTill,
        };
        Self { ch: self.ch, kind }
    }
}

fn grapheme_starts_with(grapheme: &str, ch: char) -> bool {
    grapheme.starts_with(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_forward() {
        let find = CharFind::new('l', CharFindKind::ForwardTo);
        let result = find.execute("hello", 0);
        assert_eq!(result, Some(2)); // first 'l'
    }

    #[test]
    fn find_backward() {
        let find = CharFind::new('l', CharFindKind::BackwardTo);
        let result = find.execute("hello", 4);
        assert_eq!(result, Some(3)); // second 'l'
    }

    #[test]
    fn till_forward() {
        let find = CharFind::new('l', CharFindKind::ForwardTill);
        let result = find.execute("hello", 0);
        assert_eq!(result, Some(1)); // before first 'l'
    }

    #[test]
    fn find_not_found() {
        let find = CharFind::new('z', CharFindKind::ForwardTo);
        let result = find.execute("hello", 0);
        assert_eq!(result, None);
    }
}
