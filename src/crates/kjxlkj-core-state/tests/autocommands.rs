use kjxlkj_core_state::{AutoCmdTable, AutoEvent};

#[test]
fn add_and_match() {
    let mut t = AutoCmdTable::new();
    t.add(AutoEvent::BufRead, "*.rs", "set tabstop=4", None);
    let cmds = t.matching(AutoEvent::BufRead, "main.rs");
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0], "set tabstop=4");
}

#[test]
fn no_match_wrong_ext() {
    let mut t = AutoCmdTable::new();
    t.add(AutoEvent::BufRead, "*.rs", "set tabstop=4", None);
    assert!(t.matching(AutoEvent::BufRead, "main.py").is_empty());
}

#[test]
fn wildcard_matches_all() {
    let mut t = AutoCmdTable::new();
    t.add(AutoEvent::BufRead, "*", "echo loaded", None);
    assert_eq!(t.matching(AutoEvent::BufRead, "anything.txt").len(), 1);
}

#[test]
fn group_clear() {
    let mut t = AutoCmdTable::new();
    t.add(AutoEvent::BufRead, "*", "cmd1", Some("mygroup"));
    t.add(AutoEvent::BufRead, "*", "cmd2", None);
    t.clear_group("mygroup");
    let cmds = t.matching(AutoEvent::BufRead, "a.txt");
    assert_eq!(cmds.len(), 1);
    assert_eq!(cmds[0], "cmd2");
}

#[test]
fn display_autocommands() {
    let mut t = AutoCmdTable::new();
    t.add(AutoEvent::FileType, "*.py", "set tabstop=4", None);
    let s = t.display();
    assert!(s.contains("*.py"));
    assert!(s.contains("tabstop"));
}
