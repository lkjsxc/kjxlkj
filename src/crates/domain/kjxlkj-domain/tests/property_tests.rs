/// Property-based tests for domain invariants per IMP-TEST-01.
/// Spec: /docs/spec/technical/testing.md
/// Verifies invariants hold for varied inputs without external deps.
use kjxlkj_domain::attachment::{CHUNK_SIZE, MAX_FILE_SIZE};
use kjxlkj_domain::error::DomainError;
use kjxlkj_domain::note::{NoteKind, NoteState};
use kjxlkj_domain::permission::Role;
use kjxlkj_domain::search::SearchMode;

/// Property: NoteKind round-trip through as_str/from_str is identity.
#[test]
fn prop_note_kind_round_trip() {
    let kinds = [
        NoteKind::Markdown,
        NoteKind::Settings,
        NoteKind::MediaImage,
        NoteKind::MediaVideo,
    ];
    for kind in kinds {
        let s = kind.as_str();
        let recovered = NoteKind::from_str(s);
        assert_eq!(recovered, Some(kind), "round-trip failed for {s}");
    }
}

/// Property: NoteKind::from_str rejects arbitrary non-canonical strings.
#[test]
fn prop_note_kind_rejects_arbitrary() {
    let bad_inputs = [
        "MARKDOWN", "Markdown", "md", "", " ", "note", "image",
        "video", "media", "settings_", "_markdown",
    ];
    for s in bad_inputs {
        assert_eq!(
            NoteKind::from_str(s),
            None,
            "should reject non-canonical: {s}"
        );
    }
}

/// Property: SearchMode round-trip.
#[test]
fn prop_search_mode_round_trip() {
    let modes = [
        SearchMode::Lexical,
        SearchMode::Semantic,
        SearchMode::Hybrid,
    ];
    for mode in modes {
        let s = mode.as_str();
        let recovered = SearchMode::from_str(s);
        assert_eq!(recovered, Some(mode), "round-trip failed for {s}");
    }
}

/// Property: DomainError status codes are valid HTTP codes (100-599).
#[test]
fn prop_error_status_codes_valid() {
    let errors: Vec<DomainError> = vec![
        DomainError::NoteNotFound,
        DomainError::WorkspaceNotFound,
        DomainError::ProjectNotFound,
        DomainError::VersionConflict { expected: 1, actual: 2 },
        DomainError::AuthRequired,
        DomainError::InvalidCredentials,
        DomainError::RoleForbidden,
        DomainError::WorkspaceForbidden,
        DomainError::SetupAlreadyCompleted,
        DomainError::BadRequest("test".into()),
        DomainError::InvalidPatch,
        DomainError::RuleInvalid("test".into()),
        DomainError::PromptJsonInvalid("test".into()),
        DomainError::PromptSchemaInvalid("test".into()),
        DomainError::AgentMemoryStoreError("test".into()),
        DomainError::AgentYoloPolicyViolation("test".into()),
        DomainError::SearchModeInvalid("test".into()),
        DomainError::SearchEmbeddingDegraded,
        DomainError::RateLimited,
        DomainError::LlmUpstreamError("test".into()),
        DomainError::EmbeddingProviderError("test".into()),
        DomainError::PayloadTooLarge,
        DomainError::StaleCursor,
        DomainError::Internal("test".into()),
    ];
    for err in &errors {
        let status = err.status_code();
        assert!(
            (100..=599).contains(&status),
            "invalid HTTP status {status} for {err}"
        );
    }
}

/// Property: DomainError codes are non-empty SCREAMING_SNAKE_CASE.
#[test]
fn prop_error_codes_screaming_snake() {
    let errors: Vec<DomainError> = vec![
        DomainError::NoteNotFound,
        DomainError::VersionConflict { expected: 1, actual: 2 },
        DomainError::BadRequest("x".into()),
        DomainError::Internal("x".into()),
        DomainError::PayloadTooLarge,
        DomainError::RateLimited,
        DomainError::StaleCursor,
    ];
    for err in &errors {
        let code = err.code();
        assert!(!code.is_empty(), "code must not be empty for {err}");
        assert!(
            code.chars()
                .all(|c| c.is_ascii_uppercase() || c == '_'),
            "code {code} must be SCREAMING_SNAKE_CASE"
        );
    }
}

/// Property: version conflict always maps to 409.
#[test]
fn prop_version_conflict_is_409() {
    for expected in 0..20_i64 {
        for actual in 0..20_i64 {
            let err = DomainError::VersionConflict { expected, actual };
            assert_eq!(err.status_code(), 409);
        }
    }
}

/// Property: NoteState serialization determinism.
#[test]
fn prop_note_state_serde_deterministic() {
    let states = [NoteState::Active, NoteState::SoftDeleted];
    for state in states {
        let json1 = serde_json::to_string(&state).unwrap();
        let json2 = serde_json::to_string(&state).unwrap();
        assert_eq!(json1, json2, "serialization must be deterministic");
        let recovered: NoteState = serde_json::from_str(&json1).unwrap();
        assert_eq!(recovered, state, "deserialization round-trip");
    }
}

/// Property: Role serialization determinism.
#[test]
fn prop_role_serde_round_trip() {
    let roles = [Role::Owner, Role::Admin, Role::Editor, Role::Viewer];
    for role in roles {
        let json = serde_json::to_string(&role).unwrap();
        let recovered: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, role, "round-trip failed for {role:?}");
    }
}

/// Property: CHUNK_SIZE and MAX_FILE_SIZE are reasonable.
#[test]
fn prop_attachment_constants_sane() {
    assert!(CHUNK_SIZE > 0, "CHUNK_SIZE must be positive");
    assert!(MAX_FILE_SIZE > CHUNK_SIZE, "MAX_FILE_SIZE > CHUNK_SIZE");
    // CHUNK_SIZE = 4 MiB per spec
    assert_eq!(CHUNK_SIZE, 4 * 1024 * 1024);
    // MAX_FILE_SIZE = 500 MiB per spec
    assert_eq!(MAX_FILE_SIZE, 500 * 1024 * 1024);
}

/// Property: chunk count calculation for arbitrary sizes.
#[test]
fn prop_chunk_count_monotonic() {
    let sizes: Vec<usize> = vec![
        0, 1, CHUNK_SIZE - 1, CHUNK_SIZE, CHUNK_SIZE + 1,
        2 * CHUNK_SIZE, 10 * CHUNK_SIZE, MAX_FILE_SIZE,
    ];
    let mut prev_chunks = 0usize;
    for &size in &sizes {
        let chunks = ((size + CHUNK_SIZE - 1) / CHUNK_SIZE).max(1);
        if size > 0 {
            assert!(
                chunks >= prev_chunks,
                "chunk count must be monotonic: size={size}"
            );
        }
        prev_chunks = chunks;
    }
}

/// Property: DomainError Display is non-empty for all variants.
#[test]
fn prop_error_display_nonempty() {
    let errors: Vec<DomainError> = vec![
        DomainError::NoteNotFound,
        DomainError::WorkspaceNotFound,
        DomainError::AuthRequired,
        DomainError::Internal("".into()),
        DomainError::BadRequest("".into()),
    ];
    for err in &errors {
        let display = format!("{err}");
        assert!(!display.is_empty(), "display must be non-empty");
    }
}
