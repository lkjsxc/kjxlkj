use super::markdown_embeds::render_url_embed;

pub struct EmbedBlock {
    pub token: String,
    pub html: String,
}

pub fn extract(body: &str, public_base_url: Option<&str>) -> (String, Vec<EmbedBlock>) {
    let mut out = String::new();
    let mut blocks = Vec::new();
    let mut para = Vec::new();
    let mut in_fence = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if fence_marker(trimmed) {
            flush_para(&mut out, &mut para, &mut blocks, public_base_url);
            in_fence = !in_fence;
            push_line(&mut out, line);
            continue;
        }
        if in_fence {
            push_line(&mut out, line);
        } else if trimmed.is_empty() {
            flush_para(&mut out, &mut para, &mut blocks, public_base_url);
            out.push('\n');
        } else {
            para.push(line.to_string());
        }
    }
    flush_para(&mut out, &mut para, &mut blocks, public_base_url);
    (out, blocks)
}

pub fn restore(mut html: String, blocks: &[EmbedBlock]) -> String {
    for block in blocks {
        html = html.replace(&format!("<p>{}</p>", block.token), &block.html);
        html = html.replace(&block.token, &block.html);
    }
    html
}

fn flush_para(
    out: &mut String,
    para: &mut Vec<String>,
    blocks: &mut Vec<EmbedBlock>,
    public_base_url: Option<&str>,
) {
    if para.is_empty() {
        return;
    }
    if para.len() == 1 {
        let line = &para[0];
        if !line.starts_with(char::is_whitespace) {
            if let Some(url) = standalone_url(line.trim(), public_base_url) {
                let token = format!("KJXLKJ_EMBED_TOKEN_{}", blocks.len());
                if let Some(html) = render_url_embed(&url) {
                    push_line(out, &token);
                    blocks.push(EmbedBlock { token, html });
                    para.clear();
                    return;
                }
            }
        }
    }
    for line in para.drain(..) {
        push_line(out, &line);
    }
}

fn standalone_url(value: &str, public_base_url: Option<&str>) -> Option<String> {
    if value.chars().any(char::is_whitespace) {
        return None;
    }
    if value.starts_with('/') && !value.starts_with("//") {
        return Some(value.to_string());
    }
    if matches!(
        value.split_once("://").map(|(scheme, _)| scheme),
        Some("http" | "https")
    ) {
        return Some(
            to_local_public_path(value, public_base_url).unwrap_or_else(|| value.to_string()),
        );
    }
    None
}

fn to_local_public_path(value: &str, public_base_url: Option<&str>) -> Option<String> {
    let base = public_base_url?.trim_end_matches('/');
    value
        .strip_prefix(base)
        .filter(|path| path.starts_with('/'))
        .map(str::to_string)
}

fn fence_marker(trimmed: &str) -> bool {
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

fn push_line(out: &mut String, line: &str) {
    out.push_str(line);
    out.push('\n');
}
