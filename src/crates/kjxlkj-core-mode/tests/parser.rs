use kjxlkj_core_mode::KeyParser;
use kjxlkj_core_types::*;

#[test]
fn basic_motions() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('h')), Intent::Motion(MotionKind::Left, 1));
    assert_eq!(p.parse_normal(&KeyEvent::char('j')), Intent::Motion(MotionKind::Down, 1));
}

#[test]
fn count_prefix() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('3'));
    assert_eq!(p.parse_normal(&KeyEvent::char('j')), Intent::Motion(MotionKind::Down, 3));
}

#[test]
fn insert_mode_entry() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('i')), Intent::EnterInsert(InsertPosition::BeforeCursor));
}

#[test]
fn escape_in_insert() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_insert(&KeyEvent::special(KeyCode::Escape)), Intent::EnterMode(Mode::Normal));
}

#[test]
fn gg_goes_to_file_start() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('g')), Intent::Noop);
    assert_eq!(p.parse_normal(&KeyEvent::char('g')), Intent::Motion(MotionKind::FileStart, 1));
}

#[test]
fn dd_deletes_line() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('d')), Intent::Noop);
    assert_eq!(p.parse_normal(&KeyEvent::char('d')), Intent::LineOperator(OperatorKind::Delete, 1));
}

#[test]
fn dw_deletes_word() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(p.parse_normal(&KeyEvent::char('w')), Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1));
}

#[test]
fn undo_redo() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('u')), Intent::Undo);
    assert_eq!(p.parse_normal(&KeyEvent::ctrl('r')), Intent::Redo);
}

#[test]
fn zz_scroll_center() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('z'));
    assert_eq!(p.parse_normal(&KeyEvent::char('z')), Intent::Scroll(ScrollKind::CursorCenter));
}

#[test]
fn find_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('f'));
    assert_eq!(p.parse_normal(&KeyEvent::char('x')), Intent::FindChar('x', FindCharKind::Forward));
}

#[test]
fn leader_key() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char(' '));
    assert_eq!(p.parse_normal(&KeyEvent::char('e')), Intent::ExCommand(":explorer".into()));
}

#[test]
fn visual_mode_operators() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_visual(&KeyEvent::char('d')), Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1));
}

#[test]
fn insert_char_in_insert_mode() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_insert(&KeyEvent::char('a')), Intent::InsertChar('a'));
}

#[test]
fn replace_mode() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_replace(&KeyEvent::char('x')), Intent::ReplaceInsert('x'));
}
