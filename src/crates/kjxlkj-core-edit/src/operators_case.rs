//! Case-conversion operators for text ranges.

use kjxlkj_core_text::manipulation::{convert_case, CaseKind};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Range;

/// Toggle case of all characters in the range.
pub fn toggle_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Toggle);
}

/// Convert all characters in the range to upper case.
pub fn upper_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Upper);
}

/// Convert all characters in the range to lower case.
pub fn lower_case_range(buffer: &mut TextBuffer, range: Range) {
    apply_case(buffer, range, CaseKind::Lower);
}

fn apply_case(buffer: &mut TextBuffer, range: Range, kind: CaseKind) {
    let r = range.normalized();
    let text = crate::operators::extract_text(buffer, r);
    let converted = convert_case(&text, kind);
    buffer.delete_range(r.start, r.end);
    buffer.insert_text(r.start, &converted);
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_text::TextBuffer;
    use kjxlkj_core_types::{BufferId, Position};

    fn buf(text: &str) -> TextBuffer {
        TextBuffer::from_text(BufferId(1), "test".into(), text)
    }

    #[test]
    fn test_toggle_case() {
        let mut b = buf("Hello");
        toggle_case_range(&mut b, Range::new(Position::new(0, 0), Position::new(0, 5)));
        assert_eq!(b.line(0).unwrap(), "hELLO");
    }
}
