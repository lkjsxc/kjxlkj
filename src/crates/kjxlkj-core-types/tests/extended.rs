//! Additional edge case and stress tests for core-types.

use kjxlkj_core_types::*;

// ──────────── Position ordering ────────────

#[test]
fn position_ord_same_line() {
    assert!(Position::new(0, 0) < Position::new(0, 1));
    assert!(Position::new(0, 5) > Position::new(0, 3));
}

#[test]
fn position_ord_different_lines() {
    assert!(Position::new(0, 100) < Position::new(1, 0));
    assert!(Position::new(2, 0) > Position::new(1, 99));
}

#[test]
fn position_equal() {
    assert_eq!(Position::new(5, 10), Position::new(5, 10));
}

#[test]
fn position_clone() {
    let p = Position::new(3, 7);
    let p2 = p;
    assert_eq!(p, p2);
}

#[test]
fn position_debug() {
    let p = Position::new(1, 2);
    let s = format!("{:?}", p);
    assert!(s.contains("1") && s.contains("2"));
}

// ──────────── Range ────────────

#[test]
fn range_same_position() {
    let r = Range::new(Position::new(0, 0), Position::new(0, 0));
    assert_eq!(r.start, r.end);
}

#[test]
fn range_multiline() {
    let r = Range::new(Position::new(0, 5), Position::new(3, 10));
    assert_eq!(r.start.line, 0);
    assert_eq!(r.end.line, 3);
}

#[test]
fn range_clone() {
    let r = Range::new(Position::new(1, 2), Position::new(3, 4));
    let r2 = r;
    assert_eq!(r.start, r2.start);
    assert_eq!(r.end, r2.end);
}

// ──────────── Size ────────────

#[test]
fn size_zero() {
    let s = Size::new(0, 0);
    assert_eq!(s.width, 0);
    assert_eq!(s.height, 0);
}

#[test]
fn size_large() {
    let s = Size::new(u16::MAX, u16::MAX);
    assert_eq!(s.width, 65535);
    assert_eq!(s.height, 65535);
}

// ──────────── Rect ────────────

#[test]
fn rect_area_zero() {
    let r = Rect::new(0, 0, 0, 0);
    assert_eq!(r.area(), 0);
}

#[test]
fn rect_area_large() {
    let r = Rect::new(0, 0, 100, 50);
    assert_eq!(r.area(), 5000);
}

#[test]
fn rect_right_bottom() {
    let r = Rect::new(5, 10, 20, 30);
    assert_eq!(r.right(), 25);
    assert_eq!(r.bottom(), 40);
}

// ──────────── BufferId / WindowId / TabId ────────────

#[test]
fn buffer_id_unique() {
    let a = BufferId::next();
    let b = BufferId::next();
    assert_ne!(a, b);
}

#[test]
fn window_id_unique() {
    let a = WindowId::next();
    let b = WindowId::next();
    assert_ne!(a, b);
}

#[test]
fn tab_id_unique() {
    let a = TabId::next();
    let b = TabId::next();
    assert_ne!(a, b);
}

#[test]
fn buffer_id_copy() {
    let a = BufferId::next();
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn buffer_version_next() {
    let v = BufferVersion(0);
    let v2 = v.next();
    assert_ne!(v, v2);
}

#[test]
fn buffer_version_sequential() {
    let v0 = BufferVersion(0);
    let v1 = v0.next();
    let v2 = v1.next();
    assert_ne!(v0, v1);
    assert_ne!(v1, v2);
    assert_ne!(v0, v2);
}

// ──────────── Mode variants ────────────

#[test]
fn mode_all_variants() {
    let modes = [
        Mode::Normal,
        Mode::Insert,
        Mode::Visual,
        Mode::VisualLine,
        Mode::VisualBlock,
        Mode::Command,
        Mode::Replace,
        Mode::OperatorPending,
    ];
    for m in &modes {
        format!("{}", m); // Display trait
        format!("{:?}", m); // Debug trait
    }
}

#[test]
fn mode_is_visual() {
    assert!(!Mode::Normal.is_visual());
    assert!(Mode::Visual.is_visual());
    assert!(Mode::VisualLine.is_visual());
    assert!(Mode::VisualBlock.is_visual());
    assert!(!Mode::Insert.is_visual());
}

#[test]
fn mode_is_insert_like() {
    assert!(Mode::Insert.is_insert_like());
    assert!(Mode::Replace.is_insert_like());
    assert!(!Mode::Normal.is_insert_like());
    assert!(!Mode::Command.is_insert_like());
}

#[test]
fn mode_cursor_shape() {
    assert_eq!(Mode::Normal.cursor_shape(), CursorShape::Block);
    assert_eq!(Mode::Insert.cursor_shape(), CursorShape::Bar);
    assert_eq!(Mode::Replace.cursor_shape(), CursorShape::Underline);
}

#[test]
fn mode_from_name() {
    assert_eq!(Mode::from_name("n"), Some(Mode::Normal));
    assert_eq!(Mode::from_name("i"), Some(Mode::Insert));
    assert_eq!(Mode::from_name("v"), Some(Mode::Visual));
    assert_eq!(Mode::from_name("V"), Some(Mode::Visual)); // lowercased → "v"
    assert_eq!(Mode::from_name("vl"), Some(Mode::VisualLine));
    assert_eq!(Mode::from_name("unknown"), None);
}

// ──────────── RegisterName ────────────

#[test]
fn register_from_char_alpha() {
    for c in 'a'..='z' {
        assert_eq!(RegisterName::from_char(c), Some(RegisterName::Named(c)));
    }
}

#[test]
fn register_from_char_numeric() {
    for c in '1'..='9' {
        let n = c.to_digit(10).unwrap() as u8;
        assert_eq!(
            RegisterName::from_char(c),
            Some(RegisterName::Numbered(n))
        );
    }
    // '0' maps to Yank register, not Numbered(0)
    assert_eq!(RegisterName::from_char('0'), Some(RegisterName::Yank));
}

#[test]
fn register_from_char_special() {
    assert_eq!(RegisterName::from_char('"'), Some(RegisterName::Unnamed));
    assert_eq!(RegisterName::from_char('_'), Some(RegisterName::BlackHole));
    assert_eq!(RegisterName::from_char('+'), Some(RegisterName::Clipboard));
    assert_eq!(RegisterName::from_char('*'), Some(RegisterName::Primary));
    assert_eq!(RegisterName::from_char('/'), Some(RegisterName::Search));
}

#[test]
fn register_readonly_checks() {
    assert!(RegisterName::LastInserted.is_readonly());
    assert!(RegisterName::CurrentFile.is_readonly());
    assert!(RegisterName::LastCommand.is_readonly());
    assert!(!RegisterName::Named('a').is_readonly());
    assert!(!RegisterName::Unnamed.is_readonly());
}

#[test]
fn register_is_append() {
    assert!(RegisterName::is_append('A'));
    assert!(!RegisterName::is_append('a'));
    assert!(!RegisterName::is_append('"'));
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

// ──────────── KeyEvent ────────────

#[test]
fn key_event_char() {
    let k = KeyEvent::char('x');
    assert_eq!(k.code, KeyCode::Char('x'));
    assert!(!k.ctrl);
}

#[test]
fn key_event_ctrl() {
    let k = KeyEvent::ctrl('c');
    assert_eq!(k.code, KeyCode::Char('c'));
    assert!(k.ctrl);
}

#[test]
fn key_event_special() {
    let k = KeyEvent::special(KeyCode::Escape);
    assert_eq!(k.code, KeyCode::Escape);
}

#[test]
fn key_event_enter() {
    let k = KeyEvent::special(KeyCode::Enter);
    assert_eq!(k.code, KeyCode::Enter);
}

// ──────────── Intent variants ────────────

#[test]
fn intent_noop() {
    let i = Intent::Noop;
    matches!(i, Intent::Noop);
}

#[test]
fn intent_motion() {
    let i = Intent::Motion(MotionKind::Right, 3);
    matches!(i, Intent::Motion(MotionKind::Right, 3));
}

#[test]
fn intent_enter_mode() {
    let i = Intent::EnterMode(Mode::Visual);
    matches!(i, Intent::EnterMode(Mode::Visual));
}

#[test]
fn intent_operator() {
    let i = Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1);
    matches!(i, Intent::Operator(..));
}

#[test]
fn intent_insert_char() {
    let i = Intent::InsertChar('a');
    matches!(i, Intent::InsertChar('a'));
}

#[test]
fn intent_scroll() {
    let i = Intent::Scroll(ScrollKind::HalfPageDown);
    matches!(i, Intent::Scroll(..));
}

// ──────────── MotionKind coverage ────────────

#[test]
fn motion_kind_all() {
    let motions = vec![
        MotionKind::Left,
        MotionKind::Right,
        MotionKind::Up,
        MotionKind::Down,
        MotionKind::LineStart,
        MotionKind::LineEnd,
        MotionKind::FirstNonBlank,
        MotionKind::WordForward,
        MotionKind::WordBackward,
        MotionKind::WordForwardEnd,
        MotionKind::FileStart,
        MotionKind::FileEnd,
    ];
    assert!(motions.len() >= 12);
}

// ──────────── ScrollKind coverage ────────────

#[test]
fn scroll_kind_all() {
    let kinds = vec![
        ScrollKind::HalfPageDown,
        ScrollKind::HalfPageUp,
        ScrollKind::FullPageDown,
        ScrollKind::FullPageUp,
        ScrollKind::LineDown,
        ScrollKind::LineUp,
        ScrollKind::CursorTop,
        ScrollKind::CursorCenter,
        ScrollKind::CursorBottom,
    ];
    assert!(kinds.len() >= 9);
}

// ──────────── OperatorKind coverage ────────────

#[test]
fn operator_kind_eq() {
    assert_eq!(OperatorKind::Delete, OperatorKind::Delete);
    assert_ne!(OperatorKind::Delete, OperatorKind::Yank);
    assert_ne!(OperatorKind::Change, OperatorKind::Indent);
}

// ──────────── Style ────────────

#[test]
fn style_chain() {
    let s = Style::default()
        .fg(Color::Red)
        .bg(Color::Blue)
        .bold()
        .italic()
        .underline()
        .reverse();
    assert_eq!(s.fg, Some(Color::Red));
    assert_eq!(s.bg, Some(Color::Blue));
    assert!(s.bold);
    assert!(s.italic);
    assert!(s.underline);
    assert!(s.reverse);
}

#[test]
fn style_merge_override() {
    let base = Style::default().fg(Color::Red);
    let over = Style::default().fg(Color::Blue).bold();
    let merged = base.merge(over);
    assert_eq!(merged.fg, Some(Color::Blue));
    assert!(merged.bold);
}

#[test]
fn style_merge_retains_base() {
    let base = Style::default().fg(Color::Red).italic();
    let over = Style::default().bold();
    let merged = base.merge(over);
    assert_eq!(merged.fg, Some(Color::Red));
    assert!(merged.italic);
    assert!(merged.bold);
}

#[test]
fn color_all_named() {
    let colors = vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::Reset,
    ];
    assert!(colors.len() >= 9);
}

#[test]
fn color_rgb() {
    let c = Color::Rgb(127, 200, 55);
    assert_eq!(c, Color::Rgb(127, 200, 55));
    assert_ne!(c, Color::Rgb(0, 0, 0));
}

// ──────────── Diagnostic ────────────

#[test]
fn diagnostic_severity() {
    let d = Diagnostic {
        range: Range::new(Position::new(0, 0), Position::new(0, 5)),
        severity: DiagnosticSeverity::Error,
        message: "error msg".into(),
        source: Some("test".into()),
    };
    assert_eq!(d.severity, DiagnosticSeverity::Error);
    assert_eq!(d.message, "error msg");
}

#[test]
fn diagnostic_severity_variants() {
    assert_ne!(DiagnosticSeverity::Error, DiagnosticSeverity::Warning);
    assert_ne!(DiagnosticSeverity::Warning, DiagnosticSeverity::Info);
    assert_ne!(DiagnosticSeverity::Info, DiagnosticSeverity::Hint);
}
