//! Text objects for operator targets.

/// The kind of text object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    /// Word (inner or around).
    Word,
    /// WORD (inner or around).
    BigWord,
    /// Sentence.
    Sentence,
    /// Paragraph.
    Paragraph,
    /// Quoted string with the given quote character.
    Quote(char),
    /// Bracketed content with the given bracket pair.
    Bracket(char),
    /// Tag (HTML/XML).
    Tag,
}

/// A text object selection.
#[derive(Debug, Clone, Copy)]
pub struct TextObject {
    pub kind: TextObjectKind,
    /// True for "inner", false for "around".
    pub inner: bool,
    pub count: usize,
}

impl TextObject {
    /// Create an inner text object.
    pub fn inner(kind: TextObjectKind) -> Self {
        Self {
            kind,
            inner: true,
            count: 1,
        }
    }

    /// Create an around text object.
    pub fn around(kind: TextObjectKind) -> Self {
        Self {
            kind,
            inner: false,
            count: 1,
        }
    }

    /// With a count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Get the bracket pair for bracket text objects.
    pub fn bracket_pair(&self) -> Option<(char, char)> {
        match self.kind {
            TextObjectKind::Bracket(c) => Some(match c {
                '(' | ')' => ('(', ')'),
                '[' | ']' => ('[', ']'),
                '{' | '}' => ('{', '}'),
                '<' | '>' => ('<', '>'),
                _ => return None,
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_object_inner() {
        let obj = TextObject::inner(TextObjectKind::Word);
        assert!(obj.inner);
    }

    #[test]
    fn test_bracket_pair() {
        let obj = TextObject::inner(TextObjectKind::Bracket('('));
        assert_eq!(obj.bracket_pair(), Some(('(', ')')));
    }
}
