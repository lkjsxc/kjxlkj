use kjxlkj_db::models::metadata::BacklinkRow;
use kjxlkj_db::repos;
use kjxlkj_domain::errors::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

/// Refresh outgoing backlinks from a source note to its wiki-link targets.
pub async fn refresh_backlinks(
    pool: &PgPool,
    source_id: Uuid,
    body: &str,
) -> Result<(), DomainError> {
    let targets = extract_wiki_links(body);
    repos::metadata::upsert_backlinks(pool, source_id, &targets)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

/// List notes that link to the given target note.
pub async fn list_backlinks(
    pool: &PgPool,
    target_id: Uuid,
) -> Result<Vec<BacklinkRow>, DomainError> {
    repos::metadata::list_backlinks(pool, target_id)
        .await
        .map_err(|e| DomainError::Internal(e.to_string()))
}

/// Extract UUID wiki-link targets from markdown body.
/// Pattern: `[[uuid]]` where uuid is a valid v4 UUID.
fn extract_wiki_links(body: &str) -> Vec<Uuid> {
    let mut links = Vec::new();
    let mut remaining = body;
    while let Some(start) = remaining.find("[[") {
        let after = &remaining[start + 2..];
        if let Some(end) = after.find("]]") {
            let candidate = &after[..end];
            if let Ok(uuid) = Uuid::parse_str(candidate.trim()) {
                links.push(uuid);
            }
            remaining = &after[end + 2..];
        } else {
            break;
        }
    }
    links
}
