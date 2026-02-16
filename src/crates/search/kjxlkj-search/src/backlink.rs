use uuid::Uuid;

/// Parse wiki-links from markdown content.
/// Extracts targets from `[[target]]` syntax.
pub fn parse_wiki_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let chars: Vec<char> = markdown.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            i += 2; // skip [[
            let mut target = String::new();
            let mut closed = false;
            while i < len {
                if i + 1 < len && chars[i] == ']' && chars[i + 1] == ']' {
                    i += 2; // skip ]]
                    closed = true;
                    break;
                }
                target.push(chars[i]);
                i += 1;
            }
            if closed && !target.is_empty() {
                links.push(target);
            }
        } else {
            i += 1;
        }
    }
    links
}

/// Resolve wiki-link targets to note UUIDs by title lookup.
/// Returns note IDs for links that match existing notes.
pub async fn resolve_links(
    pool: &sqlx::PgPool,
    workspace_id: Uuid,
    targets: &[String],
) -> Result<Vec<Uuid>, sqlx::Error> {
    if targets.is_empty() {
        return Ok(Vec::new());
    }

    let mut ids = Vec::new();
    for target in targets {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM note_streams
             WHERE workspace_id = $1 AND title = $2 AND NOT is_deleted
             LIMIT 1"
        )
        .bind(workspace_id)
        .bind(target)
        .fetch_optional(pool)
        .await?;
        if let Some((id,)) = row {
            ids.push(id);
        }
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wiki_links() {
        let md = "See [[My Note]] and [[Other Note]] for details.";
        let links = parse_wiki_links(md);
        assert_eq!(links, vec!["My Note", "Other Note"]);
    }

    #[test]
    fn test_parse_wiki_links_empty() {
        let md = "No links here.";
        let links = parse_wiki_links(md);
        assert!(links.is_empty());
    }

    #[test]
    fn test_parse_wiki_links_nested_brackets() {
        let md = "[[note with [brackets]]]";
        let links = parse_wiki_links(md);
        assert_eq!(links, vec!["note with [brackets]"]);
    }
}
