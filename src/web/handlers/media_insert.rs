pub(super) struct InsertResult {
    pub body: String,
    pub selection_fallback: bool,
    pub cursor_utf8: usize,
}

pub(super) fn apply_insert(
    body: &str,
    start: usize,
    end: usize,
    inserted_markdown: &str,
) -> InsertResult {
    if valid_range(body, start, end) {
        return InsertResult {
            body: format!("{}{}{}", &body[..start], inserted_markdown, &body[end..]),
            selection_fallback: false,
            cursor_utf8: start + inserted_markdown.len(),
        };
    }
    InsertResult {
        body: format!("{body}{inserted_markdown}"),
        selection_fallback: true,
        cursor_utf8: body.len() + inserted_markdown.len(),
    }
}

fn valid_range(body: &str, start: usize, end: usize) -> bool {
    start <= end && end <= body.len() && body.is_char_boundary(start) && body.is_char_boundary(end)
}

#[cfg(test)]
mod tests {
    use super::apply_insert;

    #[test]
    fn inserts_at_valid_end_range() {
        let body = "# Title\n\n";
        let result = apply_insert(body, body.len(), body.len(), "![image](/files/id)");

        assert_eq!(result.body, "# Title\n\n![image](/files/id)");
        assert!(!result.selection_fallback);
        assert_eq!(result.cursor_utf8, body.len() + "![image](/files/id)".len());
    }

    #[test]
    fn replaces_valid_selection() {
        let result = apply_insert("before old after", 7, 10, "new");

        assert_eq!(result.body, "before new after");
        assert!(!result.selection_fallback);
        assert_eq!(result.cursor_utf8, 10);
    }

    #[test]
    fn appends_when_range_is_outside_reversed_or_not_utf8_boundary() {
        let cases = [("plain", 20, 20), ("plain", 4, 2), ("é\n", 1, 1)];

        for (body, start, end) in cases {
            let result = apply_insert(body, start, end, "\n![image](/files/id)");

            assert_eq!(result.body, format!("{body}\n![image](/files/id)"));
            assert!(result.selection_fallback);
            assert_eq!(
                result.cursor_utf8,
                body.len() + "\n![image](/files/id)".len()
            );
        }
    }
}
