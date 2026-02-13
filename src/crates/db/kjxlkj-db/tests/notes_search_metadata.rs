use kjxlkj_auth::hash_password;
use kjxlkj_db::repos;

#[tokio::test]
async fn metadata_tags_backlinks_and_search_flow() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_hash = hash_password("owner-password").expect("hash owner password");
    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &format!("owner-meta-{token}@example.com"),
        "Owner",
        &owner_hash,
        &format!("ws-meta-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner/workspace");

    let (stream, _) = repos::notes::create_note(
        &pool,
        owner.id,
        repos::notes::CreateNoteInput {
            workspace_id: workspace.id,
            project_id: None,
            title: "Searchable note".to_owned(),
            note_kind: "markdown".to_owned(),
            access_scope: "workspace".to_owned(),
            markdown: "Hello metadata [[Target Doc]]".to_owned(),
        },
    )
    .await
    .expect("create note");

    repos::notes::upsert_metadata(
        &pool,
        stream.id,
        "priority",
        serde_json::json!({"level": 2, "owner": "ops"}),
    )
    .await
    .expect("upsert metadata");

    repos::notes::replace_tags(
        &pool,
        stream.id,
        &["Incident".to_owned(), "Runbook".to_owned()],
    )
    .await
    .expect("replace tags");

    let backlinks = repos::notes::note_backlinks(&pool, stream.id)
        .await
        .expect("fetch backlinks");
    assert!(
        backlinks.iter().any(|row| row.target_title == "Target Doc"),
        "expected backlink token from wiki-link parsing"
    );

    let search_by_text = repos::notes::search_notes(&pool, workspace.id, "Hello")
        .await
        .expect("search by title/body text");
    assert!(
        search_by_text.iter().any(|row| row.note_id == stream.id),
        "expected note in full-text search results"
    );

    let search_by_metadata = repos::notes::search_notes(&pool, workspace.id, "ops")
        .await
        .expect("search by metadata text");
    assert!(
        search_by_metadata.iter().any(|row| row.note_id == stream.id),
        "expected note in metadata-backed search results"
    );

    repos::notes::delete_metadata_key(&pool, stream.id, "priority")
        .await
        .expect("delete metadata key");

    let (_, projection_after_delete) = repos::notes::get_note(&pool, stream.id)
        .await
        .expect("fetch note projection")
        .expect("note should exist");
    assert_eq!(projection_after_delete.metadata_json, serde_json::json!({}));

    repos::notes::soft_delete_note(&pool, owner.id, stream.id)
        .await
        .expect("soft delete note");

    let search_after_delete = repos::notes::search_notes(&pool, workspace.id, "Hello")
        .await
        .expect("search after delete");
    assert!(
        !search_after_delete.iter().any(|row| row.note_id == stream.id),
        "soft-deleted note must be excluded from default search"
    );
}
