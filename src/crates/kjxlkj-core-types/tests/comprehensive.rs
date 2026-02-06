//! Comprehensive tests for core-types geometry, ids, style.

use kjxlkj_core_types::*;

#[test]
fn position_ordering() {
    assert!(Position::new(0, 0) < Position::new(0, 1));
    assert!(Position::new(0, 5) < Position::new(1, 0));
    assert!(Position::new(2, 3) == Position::new(2, 3));
    assert!(Position::new(1, 0) > Position::new(0, 100));
}

#[test]
fn position_default() {
    let p = Position::default();
    assert_eq!(p.line, 0);
    assert_eq!(p.col, 0);
}

#[test]
fn range_creation() {
    let r = Range::new(Position::new(1, 2), Position::new(3, 4));
    assert_eq!(r.start.line, 1);
    assert_eq!(r.end.line, 3);
}

#[test]
fn range_ordered() {
    // Reversed range should be ordered
    let r = Range::new(Position::new(3, 4), Position::new(1, 2));
    let o = r.ordered();
    assert!(o.start <= o.end);
}

#[test]
fn range_is_empty() {
    let r = Range::new(Position::new(0, 0), Position::new(0, 0));
    assert!(r.is_empty());
    let r2 = Range::new(Position::new(0, 0), Position::new(0, 1));
    assert!(!r2.is_empty());
}

#[test]
fn size_creation() {
    let s = Size::new(80, 24);
    assert_eq!(s.width, 80);
    assert_eq!(s.height, 24);
}

#[test]
fn rect_area_and_boundaries() {
    let r = Rect::new(5, 5, 10, 10);
    assert_eq!(r.right(), 15);
    assert_eq!(r.bottom(), 15);
    assert_eq!(r.area(), 100);
}

#[test]
fn rect_zero_size() {
    let r = Rect::new(0, 0, 0, 0);
    assert_eq!(r.area(), 0);
    assert_eq!(r.right(), 0);
    assert_eq!(r.bottom(), 0);
}

#[test]
fn buffer_id_uniqueness() {
    let a = BufferId::next();
    let b = BufferId::next();
    assert_ne!(a, b);
}

#[test]
fn window_id_uniqueness() {
    let a = WindowId::next();
    let b = WindowId::next();
    assert_ne!(a, b);
}

#[test]
fn tab_id_uniqueness() {
    let a = TabId::next();
    let b = TabId::next();
    assert_ne!(a, b);
}

#[test]
fn buffer_version_ordering() {
    let v0 = BufferVersion(0);
    let v1 = v0.next();
    assert!(v1 > v0);
    assert_eq!(v1, BufferVersion(1));
}

#[test]
fn mode_all_variants() {
    let modes = [
        Mode::Normal, Mode::Insert, Mode::Visual, Mode::VisualLine,
        Mode::VisualBlock, Mode::Replace, Mode::Command, Mode::OperatorPending,
    ];
    for m in &modes {
        assert!(!format!("{}", m).is_empty());
    }
}

#[test]
fn mode_is_visual_all() {
    assert!(Mode::Visual.is_visual());
    assert!(Mode::VisualLine.is_visual());
    assert!(Mode::VisualBlock.is_visual());
    assert!(!Mode::Normal.is_visual());
    assert!(!Mode::Insert.is_visual());
    assert!(!Mode::Replace.is_visual());
    assert!(!Mode::Command.is_visual());
}

#[test]
fn mode_is_insert_like() {
    assert!(Mode::Insert.is_insert_like());
    assert!(Mode::Replace.is_insert_like());
    assert!(!Mode::Normal.is_insert_like());
    assert!(!Mode::Visual.is_insert_like());
}

#[test]
fn mode_cursor_shapes() {
    assert_eq!(Mode::Normal.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::Insert.cursor_shape(), CursorShape::Bar);
    assert_eq!(Mode::Replace.cursor_shape(), CursorShape::Underline);
    assert_eq!(Mode::Visual.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::VisualLine.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::VisualBlock.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::Command.cursor_shape(), CursorShape::Bar);
    assert_eq!(Mode::OperatorPending.cursor_shape(), CursorShape::Block);
}

#[test]
fn mode_from_name_comprehensive() {
    assert_eq!(Mode::from_name("normal"), Some(Mode::Normal));
    assert_eq!(Mode::from_name("n"), Some(Mode::Normal));
    assert_eq!(Mode::from_name("i"), Some(Mode::Insert));
    assert_eq!(Mode::from_name("v"), Some(Mode::Visual));
    assert_eq!(Mode::from_name("vl"), Some(Mode::VisualLine));
    assert_eq!(Mode::from_name("vb"), Some(Mode::VisualBlock));
    assert_eq!(Mode::from_name("r"), Some(Mode::Replace));
    assert_eq!(Mode::from_name("c"), Some(Mode::Command));
    assert_eq!(Mode::from_name("cmdline"), Some(Mode::Command));
    assert_eq!(Mode::from_name("op"), Some(Mode::OperatorPending));
    assert_eq!(Mode::from_name("NORMAL"), Some(Mode::Normal));
    assert_eq!(Mode::from_name("xyz"), None);
}

#[test]
fn register_all_special() {
    assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
    assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
    assert_eq!(RegisterName::from_char('+'), Some(RegisterName::Clipboard));
    assert_eq!(RegisterName::from_char('*'), Some(RegisterName::Primary));
    assert_eq!(RegisterName::from_char('/'), Some(RegisterName::Search));
    assert_eq!(RegisterName::from_char('.'), Some(RegisterName::LastInserted));
    assert_eq!(RegisterName::from_char(':'), Some(RegisterName::LastCommand));
    assert_eq!(RegisterName::from_char('%'), Some(RegisterName::CurrentFile));
    assert_eq!(RegisterName::from_char('#'), Some(RegisterName::AlternateFile));
    assert_eq!(RegisterName::from_char('='), Some(RegisterName::Expression));
    assert_eq!(RegisterName::from_char('-'), Some(RegisterName::SmallDelete));
}

#[test]
fn register_named_lowercase_uppercase() {
    for c in 'a'..='z' {
        assert_eq!(RegisterName::from_char(c), Some(RegisterName::Named(c)));
    }
    for c in 'A'..='Z' {
        assert_eq!(
            RegisterName::from_char(c),
            Some(RegisterName::Named(c.to_ascii_lowercase()))
        );
    }
}

#[test]
fn register_numbered() {
    assert_eq!(RegisterName::from_char('0'), Some(RegisterName::Yank));
    for d in 1..=9u8 {
        let c = (b'0' + d) as char;
        assert_eq!(RegisterName::from_char(c), Some(RegisterName::Numbered(d)));
    }
}

#[test]
fn register_invalid_chars() {
    assert_eq!(RegisterName::from_char('!'), None);
    assert_eq!(RegisterName::from_char('@'), None);
    assert_eq!(RegisterName::from_char('$'), None);
    assert_eq!(RegisterName::from_char(' '), None);
    assert_eq!(RegisterName::from_char('\n'), None);
}

#[test]
fn register_content_charwise() {
    let rc = RegisterContent::charwise("hello");
    assert_eq!(rc.text, "hello");
    assert_eq!(rc.reg_type, RegisterType::Charwise);
}

#[test]
fn register_content_linewise() {
    let rc = RegisterContent::linewise("hello\n");
    assert_eq!(rc.text, "hello\n");
    assert_eq!(rc.reg_type, RegisterType::Linewise);
}

#[test]
fn key_event_char() {
    let k = KeyEvent::char('a');
    assert_eq!(k.code, KeyCode::Char('a'));
    assert!(!k.ctrl);
    assert!(!k.alt);
    assert!(!k.shift);
}

#[test]
fn key_event_ctrl() {
    let k = KeyEvent::ctrl('r');
    assert!(k.ctrl);
    assert_eq!(k.code, KeyCode::Char('r'));
}

#[test]
fn key_event_special() {
    let k = KeyEvent::special(KeyCode::Escape);
    assert_eq!(k.code, KeyCode::Escape);
    assert!(!k.ctrl);
}

#[test]
fn editor_event_variants() {
    let e = EditorEvent::Key(KeyEvent::char('a'));
    assert!(matches!(e, EditorEvent::Key(_)));
    let e2 = EditorEvent::Resize(Size::new(80, 24));
    assert!(matches!(e2, EditorEvent::Resize(_)));
    let e3 = EditorEvent::Quit;
    assert!(matches!(e3, EditorEvent::Quit));
}

#[test]
fn intent_noop() {
    assert_eq!(Intent::Noop, Intent::Noop);
}

#[test]
fn intent_motion_equality() {
    assert_eq!(
        Intent::Motion(MotionKind::Left, 1),
        Intent::Motion(MotionKind::Left, 1)
    );
    assert_ne!(
        Intent::Motion(MotionKind::Left, 1),
        Intent::Motion(MotionKind::Right, 1)
    );
}

#[test]
fn style_builder() {
    let s = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .bold();
    assert_eq!(s.fg, Some(Color::Red));
    assert_eq!(s.bg, Some(Color::Blue));
    assert!(s.bold);
}

#[test]
fn style_merge() {
    let base = Style::default().fg(Color::Red);
    let overlay = Style::default().bg(Color::Blue).bold();
    let merged = base.merge(overlay);
    assert_eq!(merged.fg, Some(Color::Red));
    assert_eq!(merged.bg, Some(Color::Blue));
    assert!(merged.bold);
}

#[test]
fn color_variants() {
    let c1 = Color::Reset;
    let c2 = Color::Red;
    let c3 = Color::Rgb(128, 0, 255);
    let c4 = Color::Indexed(42);
    assert_ne!(c1, c2);
    assert_ne!(c2, c3);
    assert_ne!(c3, c4);
}

#[test]
fn motion_kind_all_variants_exist() {
    // Ensure we can construct all motion kinds
    let motions = vec![
        MotionKind::Left, MotionKind::Right, MotionKind::Up, MotionKind::Down,
        MotionKind::WordForward, MotionKind::WordForwardEnd, MotionKind::WordBackward,
        MotionKind::WORDForward, MotionKind::WORDForwardEnd, MotionKind::WORDBackward,
        MotionKind::LineStart, MotionKind::LineEnd, MotionKind::FirstNonBlank,
        MotionKind::FileStart, MotionKind::FileEnd, MotionKind::GotoLine(1),
        MotionKind::GotoColumn(1), MotionKind::GotoPercent(50),
        MotionKind::ScreenTop, MotionKind::ScreenMiddle, MotionKind::ScreenBottom,
        MotionKind::NextParagraph, MotionKind::PrevParagraph,
        MotionKind::MatchingBracket, MotionKind::FindCharForward('x'),
    ];
    assert!(motions.len() > 20);
}

#[test]
fn operator_kind_all_variants() {
    let ops = [
        OperatorKind::Delete, OperatorKind::Yank, OperatorKind::Change,
        OperatorKind::Indent, OperatorKind::Outdent, OperatorKind::Format,
        OperatorKind::ToggleCase, OperatorKind::Uppercase, OperatorKind::Lowercase,
    ];
    assert_eq!(ops.len(), 9);
}

#[test]
fn text_object_kind_all_variants() {
    let objs = [
        TextObjectKind::Word, TextObjectKind::WORD,
        TextObjectKind::Sentence, TextObjectKind::Paragraph,
        TextObjectKind::DoubleQuote, TextObjectKind::SingleQuote,
        TextObjectKind::BackTick, TextObjectKind::Paren,
        TextObjectKind::Bracket, TextObjectKind::Brace,
        TextObjectKind::AngleBracket, TextObjectKind::Tag,
    ];
    assert_eq!(objs.len(), 12);
}

#[test]
fn scroll_kind_all_variants() {
    let scrolls = [
        ScrollKind::HalfPageDown, ScrollKind::HalfPageUp,
        ScrollKind::FullPageDown, ScrollKind::FullPageUp,
        ScrollKind::LineDown, ScrollKind::LineUp,
        ScrollKind::CursorCenter, ScrollKind::CursorTop, ScrollKind::CursorBottom,
        ScrollKind::CursorCenterFirstNonBlank,
        ScrollKind::CursorTopFirstNonBlank,
        ScrollKind::CursorBottomFirstNonBlank,
    ];
    assert_eq!(scrolls.len(), 12);
}

#[test]
fn find_char_kind_all_variants() {
    let kinds = [
        FindCharKind::Forward, FindCharKind::Backward,
        FindCharKind::TillForward, FindCharKind::TillBackward,
    ];
    assert_eq!(kinds.len(), 4);
}

#[test]
fn paste_position_all_variants() {
    let positions = [
        PastePosition::After, PastePosition::Before,
        PastePosition::AfterCursorEnd, PastePosition::BeforeCursorEnd,
    ];
    assert_eq!(positions.len(), 4);
}

#[test]
fn case_op_all_variants() {
    let ops = [CaseOp::Toggle, CaseOp::Upper, CaseOp::Lower];
    assert_eq!(ops.len(), 3);
}

#[test]
fn insert_position_all() {
    let positions = [
        InsertPosition::BeforeCursor, InsertPosition::AfterCursor,
        InsertPosition::FirstNonBlank, InsertPosition::EndOfLine,
    ];
    assert_eq!(positions.len(), 4);
}

#[test]
fn diagnostic_creation() {
    let d = Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 5)),
        severity: DiagnosticSeverity::Error,
        message: "test error".to_string(),
        source: Some("test".to_string()),
    };
    assert_eq!(d.severity, DiagnosticSeverity::Error);
    assert_eq!(d.message, "test error");
}

#[test]
fn diagnostic_severity_all() {
    let sevs = [
        DiagnosticSeverity::Error, DiagnosticSeverity::Warning,
        DiagnosticSeverity::Info, DiagnosticSeverity::Hint,
    ];
    assert_eq!(sevs.len(), 4);
}
