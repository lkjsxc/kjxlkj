//! Tests for Position, Range, Mode, EditorAction, Motion, ContractChecker,
//! LatencyProbe, and KeyEvent.
use kjxlkj_core_types::{
    ContractChecker, ContractLevel, EditorAction, KeyCode, KeyEvent, LatencyBudget, LatencyProbe,
    Mode, Modifiers, Motion, Position, Range, TextObjectType,
};
use std::time::Duration;

#[test]
fn position_ordering_line() {
    assert!(Position::new(1, 5) < Position::new(2, 0));
}
#[test]
fn position_ordering_col() {
    assert!(Position::new(1, 3) < Position::new(1, 5));
}
#[test]
fn position_equality() {
    assert_eq!(Position::new(3, 7), Position::new(3, 7));
    assert_ne!(Position::new(3, 7), Position::new(3, 8));
}
#[test]
fn position_zero() {
    assert_eq!(Position::ZERO, Position::new(0, 0));
}
#[test]
fn range_contains_start() {
    let r = Range::new(Position::new(0, 0), Position::new(0, 10));
    assert!(!r.is_empty());
}
#[test]
fn range_normalized_swaps() {
    let r = Range::new(Position::new(5, 0), Position::new(1, 0));
    let n = r.normalized();
    assert!(n.start <= n.end);
    assert_eq!(n.start, Position::new(1, 0));
}
#[test]
fn range_empty() {
    let r = Range::new(Position::new(3, 3), Position::new(3, 3));
    assert!(r.is_empty());
}
#[test]
fn mode_all_nine_variants() {
    let modes = [
        Mode::Normal,
        Mode::Insert,
        Mode::Visual,
        Mode::VisualLine,
        Mode::VisualBlock,
        Mode::Command,
        Mode::Replace,
        Mode::Terminal,
        Mode::OperatorPending,
    ];
    assert_eq!(modes.len(), 9);
}
#[test]
fn mode_display() {
    assert_eq!(format!("{}", Mode::Normal), "NORMAL");
    assert_eq!(format!("{}", Mode::Insert), "INSERT");
    assert_eq!(format!("{}", Mode::VisualBlock), "V-BLOCK");
}
#[test]
fn mode_default_is_normal() {
    assert_eq!(Mode::default(), Mode::Normal);
}
#[test]
fn editor_action_is_edit() {
    assert!(EditorAction::InsertChar('a').is_edit());
    assert!(EditorAction::DeleteChar.is_edit());
    assert!(EditorAction::Indent.is_edit());
}
#[test]
fn editor_action_not_edit() {
    assert!(!EditorAction::Noop.is_edit());
    assert!(!EditorAction::Undo.is_edit());
    assert!(!EditorAction::Quit.is_edit());
}
#[test]
fn motion_linewise() {
    assert!(Motion::Up.is_linewise());
    assert!(Motion::Down.is_linewise());
    assert!(Motion::FileStart.is_linewise());
    assert!(!Motion::Left.is_linewise());
    assert!(!Motion::Right.is_linewise());
}
#[test]
fn motion_inclusive() {
    assert!(Motion::WordEnd.is_inclusive());
    assert!(Motion::LineEnd.is_inclusive());
    assert!(!Motion::WordForward.is_inclusive());
}
#[test]
fn contract_no_violations() {
    let mut c = ContractChecker::new(false);
    c.require(true, "test", "ok");
    c.ensure(true, "test", "ok");
    assert!(!c.has_violations());
}
#[test]
fn contract_records_precondition() {
    let mut c = ContractChecker::new(false);
    c.require(false, "mod", "bad");
    assert!(c.has_violations());
    assert_eq!(c.violations[0].level, ContractLevel::Precondition);
}
#[test]
fn contract_records_postcondition() {
    let mut c = ContractChecker::new(false);
    c.ensure(false, "mod", "bad");
    assert_eq!(c.violations[0].level, ContractLevel::Postcondition);
}
#[test]
fn contract_records_invariant() {
    let mut c = ContractChecker::new(false);
    c.invariant(false, "mod", "bad");
    assert_eq!(c.violations[0].level, ContractLevel::Invariant);
}
#[test]
fn contract_in_range() {
    let mut c = ContractChecker::new(false);
    c.in_range(5, 3, "test");
    assert!(c.has_violations());
    c.in_range(2, 3, "test");
    assert_eq!(c.violations.len(), 1);
}
#[test]
fn contract_summary() {
    let c = ContractChecker::new(false);
    assert_eq!(c.summary(), "No contract violations");
}
#[test]
fn latency_probe_statistics() {
    let mut p = LatencyProbe::new();
    p.record(Duration::from_millis(5));
    p.record(Duration::from_millis(10));
    p.record(Duration::from_millis(15));
    assert_eq!(p.count(), 3);
    assert_eq!(p.min(), Some(Duration::from_millis(5)));
    assert_eq!(p.max(), Some(Duration::from_millis(15)));
    assert_eq!(p.avg(), Some(Duration::from_millis(10)));
}
#[test]
fn latency_probe_empty() {
    let p = LatencyProbe::new();
    assert_eq!(p.min(), None);
    assert_eq!(p.avg(), None);
    assert!(!p.exceeds_budget(LatencyBudget::FRAME));
}
#[test]
fn latency_probe_exceeds_budget() {
    let mut p = LatencyProbe::new();
    p.record(Duration::from_millis(20));
    assert!(p.exceeds_budget(LatencyBudget::KEYSTROKE));
}
#[test]
fn key_event_char() {
    let k = KeyEvent::char('a');
    assert_eq!(k.code, KeyCode::Char('a'));
    assert!(k.modifiers.is_empty());
}
#[test]
fn key_event_ctrl() {
    let k = KeyEvent::ctrl('c');
    assert!(k.modifiers.contains(Modifiers::CTRL));
}
#[test]
fn modifier_union() {
    let m = Modifiers::CTRL.union(Modifiers::SHIFT);
    assert!(m.contains(Modifiers::CTRL));
    assert!(m.contains(Modifiers::SHIFT));
    assert!(!m.contains(Modifiers::ALT));
}
#[test]
fn text_object_delimiters() {
    assert_eq!(TextObjectType::Paren.delimiters(), Some(('(', ')')));
    assert_eq!(TextObjectType::Bracket.delimiters(), Some(('[', ']')));
    assert!(TextObjectType::Word.delimiters().is_none());
}
