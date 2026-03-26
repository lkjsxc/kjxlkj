//! Rich-editor block analysis

#[derive(Clone, Debug)]
pub enum RichBlockKind {
    Heading(u8),
    Paragraph,
    Quote,
    Bullet,
    Ordered,
    Code(Option<String>),
}

#[derive(Clone, Debug)]
pub struct RichBlock {
    pub kind: RichBlockKind,
    pub lines: Vec<String>,
}
#[derive(Clone, Debug)]
pub struct EditorDocument {
    pub rich_mode: bool,
    pub fallback_notice: Option<&'static str>,
    pub blocks: Vec<RichBlock>,
}

pub fn editor_document(body: &str) -> EditorDocument {
    match parse_blocks(body) {
        Some(mut blocks) => {
            if blocks.is_empty() {
                blocks.push(RichBlock {
                    kind: RichBlockKind::Paragraph,
                    lines: vec![String::new()],
                });
            }
            EditorDocument {
                rich_mode: true,
                fallback_notice: None,
                blocks,
            }
        }
        None => EditorDocument {
            rich_mode: false,
            fallback_notice: Some("Text mode only for this note."),
            blocks: Vec::new(),
        },
    }
}

fn parse_blocks(body: &str) -> Option<Vec<RichBlock>> {
    let lines: Vec<&str> = body.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if unsupported(line, lines.get(i + 1).copied()) {
            return None;
        }
        if let Some((level, text)) = heading(line) {
            if inline_markup(text) {
                return None;
            }
            blocks.push(RichBlock {
                kind: RichBlockKind::Heading(level),
                lines: vec![text.to_string()],
            });
            i += 1;
            continue;
        }
        if let Some(lang) = line.strip_prefix("```") {
            let mut code = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].starts_with("```") {
                code.push(lines[i].to_string());
                i += 1;
            }
            if i == lines.len() {
                return None;
            }
            blocks.push(RichBlock {
                kind: RichBlockKind::Code(non_empty(lang.trim())),
                lines: code,
            });
            i += 1;
            continue;
        }
        if quote(line).is_some() || bullet(line).is_some() || ordered(line).is_some() {
            let (kind, parser) = if quote(line).is_some() {
                (RichBlockKind::Quote, quote as fn(&str) -> Option<&str>)
            } else if ordered(line).is_some() {
                (RichBlockKind::Ordered, ordered as fn(&str) -> Option<&str>)
            } else {
                (RichBlockKind::Bullet, bullet as fn(&str) -> Option<&str>)
            };
            let mut items = Vec::new();
            while i < lines.len() {
                let Some(text) = parser(lines[i]) else { break };
                if inline_markup(text) {
                    return None;
                }
                items.push(text.to_string());
                i += 1;
            }
            blocks.push(RichBlock { kind, lines: items });
            continue;
        }
        let mut paragraph = Vec::new();
        while i < lines.len() && !lines[i].trim().is_empty() && !special(lines[i]) {
            if unsupported(lines[i], lines.get(i + 1).copied()) || inline_markup(lines[i]) {
                return None;
            }
            paragraph.push(lines[i].to_string());
            i += 1;
        }
        blocks.push(RichBlock {
            kind: RichBlockKind::Paragraph,
            lines: paragraph,
        });
    }
    Some(blocks)
}

fn special(line: &str) -> bool {
    heading(line).is_some()
        || line.starts_with("```")
        || quote(line).is_some()
        || bullet(line).is_some()
        || ordered(line).is_some()
}

fn heading(line: &str) -> Option<(u8, &str)> {
    ["# ", "## ", "### "]
        .iter()
        .enumerate()
        .find_map(|(i, marker)| line.strip_prefix(marker).map(|text| ((i + 1) as u8, text)))
}

fn quote(line: &str) -> Option<&str> {
    line.strip_prefix("> ").or_else(|| line.strip_prefix('>'))
}

fn bullet(line: &str) -> Option<&str> {
    (line.starts_with("- ") || line.starts_with("* ")).then(|| &line[2..])
}

fn ordered(line: &str) -> Option<&str> {
    let digits = line.chars().take_while(|ch| ch.is_ascii_digit()).count();
    (digits > 0 && line.get(digits..digits + 2) == Some(". ")).then(|| &line[digits + 2..])
}

fn unsupported(line: &str, next: Option<&str>) -> bool {
    let trimmed = line.trim_start();
    line.starts_with("    ")
        || line.starts_with('\t')
        || trimmed.starts_with("####")
        || trimmed.starts_with("---")
        || trimmed.starts_with("***")
        || trimmed.starts_with("~~~")
        || trimmed.starts_with("<")
        || trimmed.starts_with("- [")
        || trimmed.starts_with("* [")
        || trimmed.starts_with("[^")
        || (trimmed.contains('|')
            && next
                .map(|row| row.chars().all(|ch| matches!(ch, '|' | '-' | ':' | ' ')))
                .unwrap_or(false))
}

fn inline_markup(text: &str) -> bool {
    text.contains("**")
        || text.contains("__")
        || text.contains('`')
        || text.contains("![")
        || text.contains("](")
        || text.contains("~~")
}

fn non_empty(text: &str) -> Option<String> {
    (!text.is_empty()).then(|| text.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rich_document_supports_plain_blocks() {
        let doc = editor_document("# Demo\n\nBody\n\n- First\n- Second\n\n> Quote");
        assert!(doc.rich_mode);
        assert_eq!(doc.blocks.len(), 4);
    }

    #[test]
    fn rich_document_falls_back_for_inline_markup() {
        let doc = editor_document("# Demo\n\nA **bold** line");
        assert!(!doc.rich_mode);
        assert_eq!(doc.fallback_notice, Some("Text mode only for this note."));
    }
}
