use kjxlkj_auth::hash_password;
use kjxlkj_db::repos;
use kjxlkj_db::repos::notes::{CreateNoteInput, NoteMutationError};
use kjxlkj_db::repos::notes_patch::PatchOp;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn notes_crud_history_rollback_and_conflict_flow() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    kjxlkj_db::migrations::run(&pool)
        .await
        .expect("apply migrations");

    let token = uuid::Uuid::now_v7().simple().to_string();
    let owner_email = format!("owner-{token}@example.com");
    let owner_hash = hash_password("owner-password").expect("hash owner password");

    let (owner, workspace) = repos::auth::create_owner_with_workspace(
        &pool,
        &owner_email,
        "Owner",
        &owner_hash,
        &format!("ws-{token}"),
        "Workspace",
    )
    .await
    .expect("create owner and workspace");

    let (stream, projection) = repos::notes::create_note(
        &pool,
        owner.id,
        CreateNoteInput {
            workspace_id: workspace.id,
            project_id: None,
            title: "First note".to_owned(),
            note_kind: "markdown".to_owned(),
            access_scope: "workspace".to_owned(),
            markdown: "Hello world".to_owned(),
        },
    )
    .await
    .expect("create note");

    assert_eq!(stream.current_version, 1);
    assert_eq!(projection.markdown, "Hello world");

    let patched = repos::notes::apply_note_patch(
        &pool,
        owner.id,
        stream.id,
        1,
        &[
            PatchOp::Retain { retain: 6 },
            PatchOp::Delete { delete: 5 },
            PatchOp::Insert {
                insert: "notes".to_owned(),
            },
        ],
        "patch-01",
    )
    .await
    .expect("apply patch");

    assert_eq!(patched.version, 2);

    let stale_patch = repos::notes::apply_note_patch(
        &pool,
        owner.id,
        stream.id,
        1,
        &[PatchOp::Retain { retain: 1 }],
        "patch-stale",
    )
    .await;

    assert!(matches!(
        stale_patch,
        Err(NoteMutationError::Conflict { current_version: 2 })
    ));

    let title_update = repos::notes::update_note_title(&pool, owner.id, stream.id, 2, "Renamed")
        .await
        .expect("update title");
    assert_eq!(title_update.version, 3);

    let stale_title = repos::notes::update_note_title(&pool, owner.id, stream.id, 2, "Stale")
        .await;
    assert!(matches!(
        stale_title,
        Err(NoteMutationError::Conflict { current_version: 3 })
    ));

    let history = repos::notes::note_history(&pool, stream.id)
        .await
        .expect("note history");
    assert!(history.len() >= 3, "history should include create, patch, and title update events");

    let rollback = repos::notes::rollback_note(&pool, owner.id, stream.id, 1)
        .await
        .expect("rollback note");
    assert_eq!(rollback.version, 4);

    let (_, rolled_back_projection) = repos::notes::get_note(&pool, stream.id)
        .await
        .expect("fetch rolled back note")
        .expect("note exists after rollback");
    assert_eq!(rolled_back_projection.markdown, "Hello world");

    repos::notes::soft_delete_note(&pool, owner.id, stream.id)
        .await
        .expect("soft delete note");

    let visible = repos::notes::list_notes(&pool, workspace.id, false)
        .await
        .expect("list visible notes");
    assert!(visible.is_empty(), "soft-deleted note must be hidden from default list");

    let with_deleted = repos::notes::list_notes(&pool, workspace.id, true)
        .await
        .expect("list notes including deleted");
    assert_eq!(with_deleted.len(), 1);
}
