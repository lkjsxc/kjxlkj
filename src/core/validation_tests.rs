use super::validation::*;

#[test]
fn valid_ids() {
    let id = generate_id();
    assert_eq!(id.len(), 26);
    assert!(validate_id(&id).is_ok());
}

#[test]
fn invalid_ids() {
    assert_eq!(validate_id("short"), Err(IdError::InvalidLength));
    assert_eq!(
        validate_id("containsinvalididletters1z"),
        Err(IdError::InvalidFormat)
    );
}

#[test]
fn aliases_normalize_and_validate() {
    assert_eq!(normalize_alias(Some("")), Ok(None));
    assert_eq!(
        normalize_alias(Some("release-notes")),
        Ok(Some("release-notes".to_string()))
    );
    assert_eq!(
        normalize_alias(Some("Release Notes")),
        Ok(Some("release-notes".to_string()))
    );
    assert_eq!(
        normalize_alias(Some("release_notes.v2")),
        Ok(Some("release_notes.v2".to_string()))
    );
    assert_eq!(
        normalize_alias(Some("release--notes")),
        Err(AliasError::InvalidFormat)
    );
    assert_eq!(
        normalize_alias(Some("release-.notes")),
        Err(AliasError::InvalidFormat)
    );
    assert_eq!(normalize_alias(Some("search")), Err(AliasError::Reserved));
    assert_eq!(
        normalize_alias(Some("abcdefghijklmnopqrstuvwxyz")),
        Err(AliasError::ConflictsWithId)
    );
}

#[test]
fn title_and_summary_derivation() {
    assert_eq!(derive_title("# Hello\n\nBody"), "Hello".to_string());
    assert_eq!(derive_title(""), "Untitled note".to_string());
    assert_eq!(derive_summary("# Hello\n\nBody"), "Body".to_string());
    assert_eq!(derive_summary("# Hello\n\n- Bullet"), "Bullet".to_string());
    assert_eq!(derive_summary("# Hello\n\n> Quote"), "Quote".to_string());
    assert_eq!(
        derive_summary("# Hello\n\nBody\n\nMore details"),
        "Body...".to_string()
    );
    assert!(derive_summary(&format!("# Hello\n\n{}", "A".repeat(180))).ends_with("..."));
    assert_eq!(derive_summary(""), "No summary yet.".to_string());
}
