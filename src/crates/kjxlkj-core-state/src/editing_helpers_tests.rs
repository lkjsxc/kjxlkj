//! Tests for editing helpers.
use crate::editing_helpers::{
    default_pairs, should_auto_pair, should_skip_close, CommentConfig,
};
use crate::editing_helpers_surround::{surround_add, surround_delete};

#[test]
fn test_auto_pair_open() {
    let pairs = default_pairs();
    assert_eq!(should_auto_pair('(', None, &pairs), Some(')'));
    assert_eq!(
        should_auto_pair('(', Some(' '), &pairs),
        Some(')')
    );
    assert_eq!(should_auto_pair('(', Some('a'), &pairs), None);
}

#[test]
fn test_skip_close() {
    let pairs = default_pairs();
    assert!(should_skip_close(')', Some(')'), &pairs));
    assert!(!should_skip_close(')', Some('a'), &pairs));
}

#[test]
fn test_toggle_comment_add() {
    let config = CommentConfig::for_extension("rs");
    let result = config.toggle_line_comment("    let x = 1;");
    assert_eq!(result, "    // let x = 1;");
}

#[test]
fn test_toggle_comment_remove() {
    let config = CommentConfig::for_extension("rs");
    let result = config.toggle_line_comment("    // let x = 1;");
    assert_eq!(result, "    let x = 1;");
}

#[test]
fn test_surround_add() {
    assert_eq!(surround_add("word", "(", ")"), "(word)");
    assert_eq!(surround_add("text", "\"", "\""), "\"text\"");
}

#[test]
fn test_surround_delete() {
    assert_eq!(
        surround_delete("(word)", "(", ")"),
        Some("word".to_string())
    );
    assert_eq!(surround_delete("word", "(", ")"), None);
}

#[test]
fn test_comment_config_python() {
    let config = CommentConfig::for_extension("py");
    assert_eq!(config.line_prefix, "#");
    let result = config.toggle_line_comment("x = 1");
    assert_eq!(result, "# x = 1");
}
