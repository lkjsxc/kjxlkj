use super::nostr::{normalize_name, normalize_names_json, normalize_relays_json};

#[test]
fn normalize_names_accepts_hex_and_lowercases_names() {
    let value = normalize_names_json(
        r#"{"Alice":"ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789"}"#,
    )
    .unwrap();
    assert_eq!(
        value["alice"],
        "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    );
}

#[test]
fn normalize_names_accepts_npub() {
    let value = normalize_names_json(
        r#"{"_":"npub140x77qfrg4ncn27dauqjx3t83x4ummcpydzk0zdtehhszg69v7ystddknj"}"#,
    )
    .unwrap();
    assert_eq!(
        value["_"],
        "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
    );
}

#[test]
fn normalize_names_rejects_invalid_name() {
    assert!(normalize_name("bad name").is_err());
}

#[test]
fn normalize_relays_requires_wss_urls() {
    assert!(normalize_relays_json(r#"["https://relay.example.com"]"#).is_err());
    assert_eq!(
        normalize_relays_json(r#"["wss://relay.example.com"]"#).unwrap()[0],
        "wss://relay.example.com"
    );
}
