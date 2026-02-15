/// Wiki-link parser per /docs/spec/domain/search.md.
///
/// Extracts `[[target]]` references from markdown text.
/// Returns a deduplicated, sorted list of target strings.
pub fn extract_wiki_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = markdown.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i + 1 < len {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            i += 2; // skip [[
            let start = i;
            let mut found_end = false;
            while i + 1 < len {
                if bytes[i] == b']' && bytes[i + 1] == b']' {
                    found_end = true;
                    break;
                }
                i += 1;
            }
            if found_end {
                let target = &markdown[start..i];
                let trimmed = target.trim();
                if !trimmed.is_empty() {
                    let s = trimmed.to_string();
                    if !links.contains(&s) {
                        links.push(s);
                    }
                }
                i += 2; // skip ]]
            }
        } else {
            i += 1;
        }
    }

    links.sort();
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_basic() {
        let md = "See [[NoteA]] and [[NoteB]] for details.";
        let expected = vec!["NoteA".to_string(), "NoteB".to_string()];
        assert_eq!(extract_wiki_links(md), expected);
    }

    #[test]
    fn test_dedup() {
        let md = "[[A]] then [[A]] again";
        assert_eq!(extract_wiki_links(md), vec!["A".to_string()]);
    }

    #[test]
    fn test_empty() {
        assert!(extract_wiki_links("no links here").is_empty());
    }

    #[test]
    fn test_whitespace_trimming() {
        let md = "[[  Spaced  ]]";
        assert_eq!(extract_wiki_links(md), vec!["Spaced".to_string()]);
    }
}
