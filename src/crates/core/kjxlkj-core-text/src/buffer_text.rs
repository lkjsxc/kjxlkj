use kjxlkj_core_types::TextRange;
use ropey::Rope;

use crate::TextError;

#[derive(Clone, Debug, Default)]
pub struct BufferText {
    rope: Rope,
}

impl BufferText {
    pub fn from_str(s: &str) -> Self {
        Self { rope: Rope::from_str(s) }
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, line_idx: usize) -> Option<String> {
        self.rope.get_line(line_idx).map(|l| l.to_string())
    }

    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), TextError> {
        if char_idx > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        self.rope.insert(char_idx, text);
        Ok(())
    }

    pub fn remove(&mut self, range: TextRange) -> Result<String, TextError> {
        if range.end > self.rope.len_chars() {
            return Err(TextError::IndexOutOfBounds);
        }
        let removed = self.rope.slice(range.start..range.end).to_string();
        self.rope.remove(range.start..range.end);
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_remove_roundtrip() {
        let mut t = BufferText::from_str("abc");
        t.insert(1, "Z").unwrap();
        assert_eq!(t.to_string(), "aZbc");

        let removed = t.remove(TextRange { start: 1, end: 2 }).unwrap();
        assert_eq!(removed, "Z");
        assert_eq!(t.to_string(), "abc");
    }
}

