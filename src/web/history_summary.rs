//! Changed-text summaries for history cards

pub fn changed_summary(before: &str, after: &str, fallback: &str) -> String {
    if before == after {
        return fallback.to_string();
    }
    let before_lines = before.lines().collect::<Vec<_>>();
    let after_lines = after.lines().collect::<Vec<_>>();
    let max = before_lines.len().max(after_lines.len());
    for index in 0..max {
        match (before_lines.get(index), after_lines.get(index)) {
            (Some(old), Some(new)) if old != new => return changed_line("Changed", new, fallback),
            (None, Some(new)) => return changed_line("Added", new, fallback),
            (Some(old), None) => return changed_line("Removed", old, fallback),
            _ => {}
        }
    }
    fallback.to_string()
}

fn changed_line(label: &str, line: &str, fallback: &str) -> String {
    let preview = plain_preview(line).unwrap_or_else(|| fallback.to_string());
    format!("{label}: {preview}")
}

fn plain_preview(line: &str) -> Option<String> {
    let trimmed = line
        .trim()
        .trim_start_matches('#')
        .trim_start_matches(['-', '*', '>', ' '])
        .trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(limit_chars(trimmed, 160))
    }
}

fn limit_chars(value: &str, limit: usize) -> String {
    let mut chars = value.chars();
    let preview = chars.by_ref().take(limit).collect::<String>();
    if chars.next().is_some() {
        format!("{preview}...")
    } else {
        preview
    }
}
