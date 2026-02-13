use kjxlkj_db::migrations;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn migration_and_ready_queries_smoke() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("set TEST_DATABASE_URL or DATABASE_URL for integration tests");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await
        .expect("connect postgres");

    migrations::run(&pool).await.expect("apply migrations");

    let value = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("select 1 should succeed");
    assert_eq!(value, 1);

    let migrations_present = migrations::migration_table_exists(&pool)
        .await
        .expect("migration table check should succeed");
    assert!(migrations_present, "_sqlx_migrations table must exist");
}
