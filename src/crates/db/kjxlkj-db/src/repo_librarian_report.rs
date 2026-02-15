use serde_json::Value as JsonValue;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

/// Per /docs/spec/technical/librarian-agent.md: run report.
#[derive(Debug, FromRow)]
pub struct LibrarianReportRow {
    pub run_id: Uuid,
    pub provider_kind: String,
    pub model: String,
    pub prompt_hash: String,
    pub parsed_operations: i32,
    pub applied_operations: i32,
    pub rejected_operations: i32,
    pub warnings: JsonValue,
}

pub async fn store_report(
    pool: &PgPool,
    run_id: Uuid,
    provider_kind: &str,
    model: &str,
    prompt_hash: &str,
    raw_prompt: Option<&str>,
    raw_response: Option<&str>,
) -> Result<LibrarianReportRow, sqlx::Error> {
    sqlx::query_as::<_, LibrarianReportRow>(
        "INSERT INTO librarian_run_reports
         (run_id, provider_kind, model, prompt_hash, raw_prompt, raw_response)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING run_id, provider_kind, model, prompt_hash,
                   parsed_operations, applied_operations, rejected_operations, warnings",
    )
    .bind(run_id)
    .bind(provider_kind)
    .bind(model)
    .bind(prompt_hash)
    .bind(raw_prompt)
    .bind(raw_response)
    .fetch_one(pool)
    .await
}

pub async fn find_report(
    pool: &PgPool,
    run_id: Uuid,
) -> Result<Option<LibrarianReportRow>, sqlx::Error> {
    sqlx::query_as::<_, LibrarianReportRow>(
        "SELECT run_id, provider_kind, model, prompt_hash,
                parsed_operations, applied_operations, rejected_operations, warnings
         FROM librarian_run_reports WHERE run_id = $1",
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await
}

/// Update report counters after operations are processed.
pub async fn update_report_counts(
    pool: &PgPool,
    run_id: Uuid,
    parsed: i32,
    applied: i32,
    rejected: i32,
    warnings: &JsonValue,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE librarian_run_reports
         SET parsed_operations = $2, applied_operations = $3,
             rejected_operations = $4, warnings = $5
         WHERE run_id = $1",
    )
    .bind(run_id)
    .bind(parsed)
    .bind(applied)
    .bind(rejected)
    .bind(warnings)
    .execute(pool)
    .await?;
    Ok(())
}
