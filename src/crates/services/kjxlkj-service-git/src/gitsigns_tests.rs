use super::*;

fn sample_state() -> GitSignState {
    let mut s = GitSignState::new("test.rs".into(), GitBase::Index);
    s.set_hunks(vec![
        Hunk { start: 2, count: 3, sign: SignType::Add },
        Hunk { start: 10, count: 1, sign: SignType::Change },
        Hunk { start: 20, count: 2, sign: SignType::Delete },
    ]);
    s
}

#[test]
fn sign_at_returns_correct_type() {
    let s = sample_state();
    assert_eq!(s.sign_at(2), Some(SignType::Add));
    assert_eq!(s.sign_at(4), Some(SignType::Add));
    assert_eq!(s.sign_at(5), None);
    assert_eq!(s.sign_at(10), Some(SignType::Change));
    assert_eq!(s.sign_at(20), Some(SignType::Delete));
    assert_eq!(s.sign_at(0), None);
}

#[test]
fn counts_computed_correctly() {
    let s = sample_state();
    assert_eq!(s.added, 3);
    assert_eq!(s.modified, 1);
    assert_eq!(s.removed, 2);
    assert_eq!(s.summary(), "+3 ~1 -2");
}

#[test]
fn next_hunk_wraps() {
    let s = sample_state();
    assert_eq!(s.next_hunk(0), Some(2));
    assert_eq!(s.next_hunk(5), Some(10));
    assert_eq!(s.next_hunk(15), Some(20));
    assert_eq!(s.next_hunk(25), Some(2)); // wrap
}

#[test]
fn prev_hunk_wraps() {
    let s = sample_state();
    assert_eq!(s.prev_hunk(25), Some(20));
    assert_eq!(s.prev_hunk(15), Some(10));
    assert_eq!(s.prev_hunk(5), Some(2));
    assert_eq!(s.prev_hunk(1), Some(20)); // wrap to last
}

#[test]
fn empty_hunks_returns_none() {
    let s = GitSignState::new("empty.rs".into(), GitBase::Head);
    assert_eq!(s.sign_at(0), None);
    assert_eq!(s.next_hunk(0), None);
    assert_eq!(s.prev_hunk(0), None);
}

#[test]
fn sign_type_chars() {
    assert_eq!(SignType::Add.char(), '│');
    assert_eq!(SignType::Delete.char(), '_');
    assert_eq!(SignType::TopDelete.char(), '‾');
    assert_eq!(SignType::ChangeDelete.char(), '~');
}

#[test]
fn set_hunks_replaces_old() {
    let mut s = sample_state();
    s.set_hunks(vec![Hunk { start: 0, count: 1, sign: SignType::Add }]);
    assert_eq!(s.hunks.len(), 1);
    assert_eq!(s.added, 1);
    assert_eq!(s.modified, 0);
    assert_eq!(s.removed, 0);
}
