// Backlinks per /docs/spec/domain/search.md
use sqlx::PgPool;
use uuid::Uuid;

/// Parse wiki links [[target]] from markdown.
pub fn parse_wiki_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = markdown.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 1 < len {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            i += 2;
            let start = i;
            while i + 1 < len {
                if bytes[i] == b']' && bytes[i + 1] == b']' {
                    let target = &markdown[start..i];
                    if !target.is_empty() {
                        links.push(target.to_string());
                    }
                    i += 2;
                    break;
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    links
}

/// Update backlinks for a source note.
pub async fn update_backlinks(
    pool: &PgPool,
    source_id: Uuid,
    targets: &[Uuid],
) -> Result<(), sqlx::Error> {
    // Clear existing
    sqlx::query("DELETE FROM backlinks WHERE source_note_id = $1")
        .bind(source_id)
        .execute(pool)
        .await?;
    // Insert new
    for target in targets {
        sqlx::query(
            "INSERT INTO backlinks (source_note_id, target_note_id) VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
        )
        .bind(source_id)
        .bind(target)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Get backlinks for a target note.
pub async fn get_backlinks(pool: &PgPool, note_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
    let rows: Vec<(Uuid,)> = sqlx::query_as(
        "SELECT source_note_id FROM backlinks WHERE target_note_id = $1",
    )
    .bind(note_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wiki_links() {
        let md = "See [[note1]] and [[note2]] for more.";
        let links = parse_wiki_links(md);
        assert_eq!(links, vec!["note1", "note2"]);
    }

    #[test]
    fn test_no_links() {
        let links = parse_wiki_links("No links here.");
        assert!(links.is_empty());
    }
}
