use crate::core::RecordInput;

use super::FsStore;

#[tokio::test]
async fn store_upsert_list_get_delete_lifecycle() {
    let root = std::path::PathBuf::from("tmp")
        .join("tests")
        .join(format!("kjxlkj-store-{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir_all(&root).await.expect("mkdir");
    let store = FsStore::new(root.clone());
    store.ensure_ready().await.expect("ready");

    let (created, was_created) = store
        .upsert(
            "alpha-note",
            RecordInput {
                title: "Alpha".to_owned(),
                body: "one".to_owned(),
                tags: vec!["Ops".to_owned(), "ops".to_owned()],
            },
        )
        .await
        .expect("create");
    assert!(was_created);
    assert_eq!(created.revision, 1);
    assert_eq!(created.tags, vec!["ops".to_owned()]);

    let (updated, was_created_second) = store
        .upsert(
            "alpha-note",
            RecordInput {
                title: "Alpha 2".to_owned(),
                body: "two".to_owned(),
                tags: vec!["qa".to_owned()],
            },
        )
        .await
        .expect("update");
    assert!(!was_created_second);
    assert_eq!(updated.revision, 2);

    let listed = store.list().await.expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, "alpha-note");

    let fetched = store.get("alpha-note").await.expect("get");
    assert!(fetched.is_some());

    let deleted = store.delete("alpha-note").await.expect("delete");
    assert!(deleted);
    let missing = store.delete("alpha-note").await.expect("delete missing");
    assert!(!missing);

    let _ = tokio::fs::remove_dir_all(root).await;
}
