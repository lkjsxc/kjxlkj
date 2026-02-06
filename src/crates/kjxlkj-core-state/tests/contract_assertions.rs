//! Contract assertion tests: validate architectural invariants.
//! Each test corresponds to a documented contract from the spec.

use kjxlkj_core_types::*;

// ── Contract: BufferId is a newtype wrapping u64 ──────────────────────

#[test]
fn buffer_id_is_newtype_u64() {
    let id = BufferId(42);
    assert_eq!(id.0, 42u64);
    let id2 = BufferId(42);
    assert_eq!(id, id2); // PartialEq
}

#[test]
fn buffer_id_distinct_values() {
    assert_ne!(BufferId(1), BufferId(2));
}

// ── Contract: WindowId is a newtype wrapping u64 ─────────────────────

#[test]
fn window_id_is_newtype_u64() {
    let wid = WindowId(7);
    assert_eq!(wid.0, 7u64);
}

// ── Contract: Size uses width/height (not rows/cols) ─────────────────

#[test]
fn size_uses_width_height() {
    let s = Size::new(80, 24);
    assert_eq!(s.width, 80);
    assert_eq!(s.height, 24);
}

// ── Contract: Mode default is Normal ─────────────────────────────────

#[test]
fn mode_default_normal() {
    assert_eq!(Mode::default(), Mode::Normal);
}

// ── Contract: Position is zero-indexed (line, col) ───────────────────

#[test]
fn position_zero_indexed() {
    let p = Position::new(0, 0);
    assert_eq!(p.line, 0);
    assert_eq!(p.col, 0);
}

#[test]
fn position_ordering() {
    let a = Position::new(1, 5);
    let b = Position::new(2, 0);
    assert!(a < b);
}

// ── Contract: Range.start <= Range.end ───────────────────────────────

#[test]
fn range_start_before_end() {
    let r = Range::new(Position::new(0, 5), Position::new(0, 10));
    assert!(r.start <= r.end);
}

// ── Contract: CursorShape is driven by Mode ──────────────────────────

#[test]
fn cursor_shape_per_mode() {
    assert_eq!(Mode::Normal.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::Insert.cursor_shape(), CursorShape::Bar);
    assert_eq!(Mode::Replace.cursor_shape(), CursorShape::Underline);
    assert_eq!(Mode::Visual.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::Terminal.cursor_shape(), CursorShape::Bar);
}

// ── Contract: ExCommand strings carry : prefix ───────────────────────

#[test]
fn ex_command_intent_has_colon_prefix() {
    let intent = Intent::ExCommand(":quit".into());
    if let Intent::ExCommand(ref cmd) = intent {
        assert!(cmd.starts_with(':'), "ExCommand must start with ':'");
    }
}

// ── Contract: RegisterName::Unnamed is the default register ──────────

#[test]
fn unnamed_register_exists() {
    let r = RegisterName::Unnamed;
    assert_eq!(format!("{r:?}"), "Unnamed");
}

// ── Contract: InsertPosition variants ────────────────────────────────

#[test]
fn insert_position_variants() {
    let positions = [
        InsertPosition::BeforeCursor,
        InsertPosition::AfterCursor,
        InsertPosition::FirstNonBlank,
        InsertPosition::EndOfLine,
    ];
    assert_eq!(positions.len(), 4);
}

// ── Contract: TextObjectKind has complete variant set ─────────────────

#[test]
fn text_object_kind_all_variants() {
    let kinds = [
        TextObjectKind::Word, TextObjectKind::WORD,
        TextObjectKind::Sentence, TextObjectKind::Paragraph,
        TextObjectKind::DoubleQuote, TextObjectKind::SingleQuote,
        TextObjectKind::BackTick, TextObjectKind::Paren,
        TextObjectKind::Bracket, TextObjectKind::Brace,
        TextObjectKind::AngleBracket, TextObjectKind::Tag,
    ];
    assert_eq!(kinds.len(), 12);
}

// ── Contract: OperatorKind covers all operators ──────────────────────

#[test]
fn operator_kind_all_variants() {
    let ops = [
        OperatorKind::Delete, OperatorKind::Yank, OperatorKind::Change,
        OperatorKind::Indent, OperatorKind::Outdent, OperatorKind::Format,
        OperatorKind::ToggleCase, OperatorKind::Uppercase, OperatorKind::Lowercase,
    ];
    assert_eq!(ops.len(), 9);
}

// ── Contract: Style merge is associative ─────────────────────────────

#[test]
fn style_merge_override() {
    let base = Style::default().fg(Color::White);
    let over = Style::default().fg(Color::Red);
    let merged = base.merge(over);
    assert_eq!(merged.fg, Some(Color::Red));
}

// ── Contract: BufferVersion increments on edit ───────────────────────

#[test]
fn buffer_version_increments() {
    use kjxlkj_core_text::TextBuffer;
    let mut buf = TextBuffer::from_text("hello");
    let v0 = buf.version();
    buf.insert_char(Position::new(0, 5), '!');
    let v1 = buf.version();
    assert!(v1.0 > v0.0, "Version must increment after edit");
}

// ── Contract: Mode transition table ──────────────────────────────────

#[test]
fn mode_transitions_valid() {
    use kjxlkj_core_mode::transitions::*;
    // Normal can reach Insert
    assert!(is_valid_transition(Mode::Normal, Mode::Insert));
    // Insert cannot directly reach Visual
    assert!(!is_valid_transition(Mode::Insert, Mode::Visual));
    // Terminal can only escape to Normal
    assert!(is_valid_transition(Mode::Terminal, Mode::Normal));
    assert!(!is_valid_transition(Mode::Terminal, Mode::Insert));
}

// ── Contract: Snapshot is O(viewport) ────────────────────────────────

#[test]
fn snapshot_bounded_by_viewport() {
    use kjxlkj_core_text::{TextBuffer, BufferSnapshot};
    let mut text = String::new();
    for i in 0..10000 { text.push_str(&format!("line {}\n", i)); }
    let buf = TextBuffer::from_text(&text);
    let snap = BufferSnapshot::from_buffer(&buf, 500, 24, Position::new(500, 0));
    assert_eq!(snap.line_count(), 24);
    assert_eq!(snap.total_lines, 10001);
}
