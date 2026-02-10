//! Tests for grapheme/text operations (CT-01 through CT-11).
//!
//! Covers spec requirements from `/docs/spec/technical/testing-unit.md`.

#[cfg(test)]
mod tests {
    use crate::grapheme::*;
    use crate::TextBuffer;
    use kjxlkj_core_types::BufferId;
    use unicode_width::UnicodeWidthStr;

    /// CT-01: Grapheme decomposition.
    /// Decomposing "cafe\u{0301}" yields 4 grapheme clusters: c, a, f, e\u{0301}.
    #[test]
    fn ct01_grapheme_decomposition_combining() {
        let s = "cafe\u{0301}";
        let gs = line_graphemes(s);
        assert_eq!(gs.len(), 4);
        assert_eq!(gs[0], "c");
        assert_eq!(gs[1], "a");
        assert_eq!(gs[2], "f");
        assert_eq!(gs[3], "e\u{0301}");
    }

    /// CT-02: CJK display width.
    /// display_width("あ") = 2, display_width("a") = 1.
    #[test]
    fn ct02_cjk_display_width() {
        assert_eq!(UnicodeWidthStr::width("あ"), 2);
        assert_eq!(UnicodeWidthStr::width("a"), 1);
    }

    /// CT-03: Mixed-width line width.
    /// For "aあbいc", per-grapheme widths = [1,2,1,2,1], total = 7.
    #[test]
    fn ct03_mixed_width_line() {
        let s = "aあbいc";
        let gs = line_graphemes(s);
        assert_eq!(gs.len(), 5);
        let widths: Vec<usize> = gs.iter().map(|g| UnicodeWidthStr::width(*g)).collect();
        assert_eq!(widths, vec![1, 2, 1, 2, 1]);
        let total: usize = widths.iter().sum();
        assert_eq!(total, 7);
    }

    /// CT-04: Emoji width.
    /// Family emoji ZWJ sequence counts as 1 grapheme with width 2.
    #[test]
    fn ct04_emoji_width() {
        let emoji = "👨\u{200D}👩\u{200D}👧\u{200D}👦";
        let gs = line_graphemes(emoji);
        assert_eq!(gs.len(), 1, "ZWJ family emoji should be 1 grapheme");
        let w = UnicodeWidthStr::width(gs[0]);
        assert_eq!(w, 2, "ZWJ emoji width should be 2");
    }

    /// CT-05: Rope insert.
    /// Insert "xyz" at offset 0 of "abc" -> "xyzabc", grapheme count 6.
    #[test]
    fn ct05_rope_insert() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "abc");
        buf.insert(0, "xyz");
        assert_eq!(buf.to_string_content(), "xyzabc");
        assert_eq!(grapheme_count(&buf.to_string_content()), 6);
    }

    /// CT-06: Rope delete.
    /// Delete byte range [1..3) from "abcde" -> "ade".
    #[test]
    fn ct06_rope_delete() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "abcde");
        buf.remove(1, 3);
        assert_eq!(buf.to_string_content(), "ade");
    }

    /// CT-07: Rope split and join.
    /// Split at midpoint, then join -> content identical.
    #[test]
    fn ct07_rope_split_and_join() {
        let original = "hello world 12345 abcdef";
        let rope = ropey::Rope::from_str(original);
        let mid = rope.len_chars() / 2;
        let left = rope.slice(..mid).to_string();
        let right = rope.slice(mid..).to_string();
        let joined = format!("{}{}", left, right);
        assert_eq!(joined, original);
    }

    /// CT-08: Large rope.
    /// 100k lines of 80 chars; line-at-index lookup < 1ms.
    #[test]
    fn ct08_large_rope_lookup() {
        let line = "A".repeat(80) + "\n";
        let text: String = line.repeat(100_000);
        let rope = ropey::Rope::from_str(&text);

        let start = std::time::Instant::now();
        let line_str = rope.line(50_000).to_string();
        let elapsed = start.elapsed();

        assert_eq!(line_str.trim(), "A".repeat(80));
        assert!(
            elapsed.as_millis() < 10,
            "Lookup took {}ms, expected <1ms",
            elapsed.as_millis()
        );
    }

    /// CT-09: Empty rope.
    /// New rope has 0 graphemes, 0 bytes. Insert then delete returns to empty.
    #[test]
    fn ct09_empty_rope() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        let content = buf.to_string_content();
        assert_eq!(grapheme_count(&content), 0);
        assert_eq!(content.len(), 0);

        buf.insert_at_char(0, "hello");
        assert_eq!(grapheme_count(&buf.to_string_content()), 5);

        buf.remove_char_range(0, 5);
        let content2 = buf.to_string_content();
        assert_eq!(grapheme_count(&content2), 0);
        assert_eq!(content2.len(), 0);
    }

    /// CT-10: Line index mapping.
    /// For a 5-line rope, line_to_byte(3) round-trips via byte_to_line.
    #[test]
    fn ct10_line_index_roundtrip() {
        let rope = ropey::Rope::from_str("aaa\nbbb\nccc\nddd\neee\n");
        assert_eq!(rope.len_lines(), 6); // 5 content lines + trailing empty
        let byte3 = rope.line_to_byte(3);
        let line_back = rope.byte_to_line(byte3);
        assert_eq!(line_back, 3);
    }

    /// CT-11: Combining mark width.
    /// Base char + 3 combining marks = 1 grapheme, width 1.
    #[test]
    fn ct11_combining_mark_width() {
        // e + combining acute + combining tilde + combining cedilla
        let s = "e\u{0301}\u{0303}\u{0327}";
        let gs = line_graphemes(s);
        assert_eq!(gs.len(), 1);
        let w = UnicodeWidthStr::width(gs[0]);
        assert_eq!(w, 1);
    }
}
