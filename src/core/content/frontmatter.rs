use super::errors::ContentValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub private: bool,
}

impl Frontmatter {
    pub fn private_default() -> Self {
        Self {
            title: None,
            private: true,
        }
    }
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self::private_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedMarkdown {
    pub frontmatter: Frontmatter,
    pub body: String,
}

pub fn parse_markdown_document(markdown: &str) -> Result<ParsedMarkdown, ContentValidationError> {
    let normalized = markdown.replace("\r\n", "\n");
    if !normalized.starts_with("---\n") {
        return Ok(ParsedMarkdown {
            frontmatter: Frontmatter::private_default(),
            body: normalized,
        });
    }

    let mut lines = normalized.split('\n');
    let _ = lines.next();
    let mut frontmatter_lines = Vec::new();
    let mut has_closing = false;

    for line in lines.by_ref() {
        if line == "---" {
            has_closing = true;
            break;
        }
        frontmatter_lines.push(line.to_owned());
    }

    if !has_closing {
        return Err(ContentValidationError::FrontmatterUnclosed);
    }

    let frontmatter = parse_frontmatter_lines(&frontmatter_lines)?;
    let body = lines.collect::<Vec<_>>().join("\n");

    Ok(ParsedMarkdown { frontmatter, body })
}

pub fn serialize_markdown_document(frontmatter: &Frontmatter, body: &str) -> String {
    if frontmatter.title.is_none() && !frontmatter.private {
        return body.to_owned();
    }

    let mut lines = vec!["---".to_owned()];
    if let Some(title) = &frontmatter.title {
        lines.push(format!("title: \"{}\"", escape_quoted(title)));
    }
    lines.push(format!("private: {}", frontmatter.private));
    lines.push("---".to_owned());

    if body.is_empty() {
        lines.join("\n")
    } else {
        format!("{}\n{}", lines.join("\n"), body)
    }
}

pub fn revision_token(markdown: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in markdown.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn parse_frontmatter_lines(lines: &[String]) -> Result<Frontmatter, ContentValidationError> {
    let mut title = None;
    let mut private = true;
    let mut private_seen = false;

    for (line_index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some((raw_key, raw_value)) = trimmed.split_once(':') else {
            return Err(ContentValidationError::FrontmatterInvalidLine {
                line: line_index + 1,
            });
        };

        let key = raw_key.trim();
        let value = raw_value.trim();
        match key {
            "title" => {
                if title.is_some() {
                    return Err(ContentValidationError::FrontmatterDuplicateKey {
                        key: "title".to_owned(),
                    });
                }
                title = Some(parse_title(value)?);
            }
            "private" => {
                if private_seen {
                    return Err(ContentValidationError::FrontmatterDuplicateKey {
                        key: "private".to_owned(),
                    });
                }
                private = parse_private(value)?;
                private_seen = true;
            }
            _ => {
                return Err(ContentValidationError::FrontmatterUnknownKey {
                    key: key.to_owned(),
                });
            }
        }
    }

    Ok(Frontmatter { title, private })
}

fn parse_title(raw: &str) -> Result<String, ContentValidationError> {
    if raw.is_empty() {
        return Err(ContentValidationError::FrontmatterEmptyTitle);
    }

    if let Some(inner) = strip_wrapped(raw, '"') {
        return Ok(unescape(inner));
    }
    if let Some(inner) = strip_wrapped(raw, '\'') {
        return Ok(unescape(inner));
    }
    Ok(raw.to_owned())
}

fn parse_private(raw: &str) -> Result<bool, ContentValidationError> {
    match raw {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ContentValidationError::FrontmatterInvalidPrivate {
            value: raw.to_owned(),
        }),
    }
}

fn strip_wrapped(raw: &str, quote: char) -> Option<&str> {
    raw.strip_prefix(quote)
        .and_then(|inner| inner.strip_suffix(quote))
}

fn escape_quoted(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn unescape(value: &str) -> String {
    value.replace("\\\"", "\"").replace("\\\\", "\\")
}
