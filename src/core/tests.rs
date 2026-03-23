use crate::core::{normalize_tags, validate_id, validate_input, RecordInput};

#[test]
fn validate_id_accepts_kebab_case_and_rejects_invalid_values() {
    assert!(validate_id("demo-note-1").is_ok());
    assert!(validate_id("ab").is_err());
    assert!(validate_id("UPPER").is_err());
    assert!(validate_id("-invalid").is_err());
    assert!(validate_id("invalid-").is_err());
    assert!(validate_id("invalid--double").is_err());
}

#[test]
fn normalize_tags_lowercases_and_deduplicates() {
    let tags = normalize_tags(&[
        "Ops".to_owned(),
        "ops".to_owned(),
        " QA ".to_owned(),
        "".to_owned(),
        "qa".to_owned(),
    ]);
    assert_eq!(tags, vec!["ops".to_owned(), "qa".to_owned()]);
}

#[test]
fn validate_input_requires_non_empty_title() {
    let invalid = RecordInput {
        title: "   ".to_owned(),
        body: "body".to_owned(),
        tags: vec![],
    };
    assert!(validate_input(&invalid).is_err());
}
