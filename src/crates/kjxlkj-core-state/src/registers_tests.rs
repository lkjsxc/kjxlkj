//! Tests for the register file.
use crate::registers::{RegisterContent, RegisterFile};

#[test]
fn test_set_and_get_named() {
    let mut rf = RegisterFile::new();
    rf.set('a', RegisterContent::Chars("hello".to_string()));
    assert_eq!(rf.get('a').unwrap().to_string_content(), "hello");
}

#[test]
fn test_yank_uses_register_0() {
    let mut rf = RegisterFile::new();
    rf.yank(RegisterContent::Chars("yanked".to_string()));
    assert_eq!(rf.get('0').unwrap().to_string_content(), "yanked");
}

#[test]
fn test_delete_rotates_numbered() {
    let mut rf = RegisterFile::new();
    rf.delete(RegisterContent::Chars("first".to_string()), false);
    rf.delete(
        RegisterContent::Chars("second".to_string()),
        false,
    );
    assert_eq!(rf.get('1').unwrap().to_string_content(), "second");
    assert_eq!(rf.get('2').unwrap().to_string_content(), "first");
}

#[test]
fn test_small_delete() {
    let mut rf = RegisterFile::new();
    rf.delete(RegisterContent::Chars("x".to_string()), true);
    assert_eq!(rf.get('-').unwrap().to_string_content(), "x");
}

#[test]
fn test_uppercase_appends() {
    let mut rf = RegisterFile::new();
    rf.set('a', RegisterContent::Chars("hello".to_string()));
    rf.set('A', RegisterContent::Chars(" world".to_string()));
    assert_eq!(
        rf.get('a').unwrap().to_string_content(),
        "hello world"
    );
}

#[test]
fn test_linewise_content() {
    let content = RegisterContent::Lines(vec![
        "line1".to_string(),
        "line2".to_string(),
    ]);
    assert!(content.is_linewise());
    assert_eq!(content.to_string_content(), "line1\nline2\n");
}

#[test]
fn test_search_register() {
    let mut rf = RegisterFile::new();
    rf.set_search("pattern".to_string());
    assert_eq!(rf.get_search(), Some("pattern"));
}

#[test]
fn test_list_nonempty() {
    let mut rf = RegisterFile::new();
    rf.set('a', RegisterContent::Chars("data".to_string()));
    rf.yank(RegisterContent::Chars("yanked".to_string()));
    let list = rf.list_nonempty();
    assert_eq!(list.len(), 2);
}
