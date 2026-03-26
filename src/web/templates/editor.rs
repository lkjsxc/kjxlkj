//! Rich note editor template

use super::layout::html_escape;
use crate::core::{EditorDocument, RichBlock, RichBlockKind};
use crate::web::db::Record;

pub fn editor_surface(record: &Record, document: &EditorDocument) -> String {
    format!(
        r#"<section class="surface editor-surface note-surface" data-rich-available="{}" data-initial-mode="{}">
<div class="editor-head">
<div class="editor-modes">
<button type="button" class="btn{}" data-mode-button="rich" {}>Rich mode</button>
<button type="button" class="btn{}" data-mode-button="text">Text mode</button>
{}
</div>
<div class="editor-side">
<label class="check-row" for="public-toggle">
<input type="checkbox" id="public-toggle" {} onchange="togglePublic()">
<span>Public</span>
</label>
<span id="save-status" class="save-status"></span>
</div>
</div>
<div class="editor-actions" {}>{}</div>
<div id="rich-editor" class="rich-editor" {}>{}</div>
<textarea id="editor" class="note-editor" {}>{}</textarea>
</section>"#,
        document.rich_mode,
        if document.rich_mode { "rich" } else { "text" },
        if document.rich_mode { " active" } else { "" },
        if document.rich_mode { "" } else { "disabled" },
        if document.rich_mode { "" } else { " active" },
        document
            .fallback_notice
            .map(|value| format!(r#"<small class="surface-empty">{value}</small>"#))
            .unwrap_or_default(),
        if record.is_private { "" } else { "checked" },
        if document.rich_mode { "" } else { " hidden" },
        add_actions(),
        if document.rich_mode { "" } else { " hidden" },
        rich_blocks(&document.blocks),
        if document.rich_mode { " hidden" } else { "" },
        html_escape(&record.body),
    )
}

fn add_actions() -> String {
    r#"<button type="button" class="btn" data-add-block="paragraph">Add paragraph</button>
<button type="button" class="btn" data-add-block="bullet">Add list</button>
<button type="button" class="btn" data-add-block="quote">Add quote</button>
<button type="button" class="btn" data-add-block="code">Add code</button>"#
        .to_string()
}

fn rich_blocks(blocks: &[RichBlock]) -> String {
    blocks.iter().map(render_block).collect::<Vec<_>>().join("")
}

fn render_block(block: &RichBlock) -> String {
    match &block.kind {
        RichBlockKind::Heading(level) => format!(
            r#"<article class="rich-block"><h{0} class="block-editable" contenteditable="true" spellcheck="true" data-kind="heading" data-level="{0}">{1}</h{0}></article>"#,
            level,
            text_html(&block.lines)
        ),
        RichBlockKind::Paragraph => format!(
            r#"<article class="rich-block"><p class="block-editable" contenteditable="true" spellcheck="true" data-kind="paragraph">{}</p></article>"#,
            text_html(&block.lines)
        ),
        RichBlockKind::Quote => format!(
            r#"<article class="rich-block"><blockquote class="block-editable" contenteditable="true" spellcheck="true" data-kind="quote">{}</blockquote></article>"#,
            text_html(&block.lines)
        ),
        RichBlockKind::Bullet => list_block("bullet", "ul", &block.lines),
        RichBlockKind::Ordered => list_block("ordered", "ol", &block.lines),
        RichBlockKind::Code(lang) => format!(
            r#"<article class="rich-block"><pre class="block-code" data-kind="code" data-lang="{}"><code class="block-editable block-code-input" contenteditable="true" spellcheck="false">{}</code></pre></article>"#,
            html_escape(lang.as_deref().unwrap_or("")),
            text_html(&block.lines)
        ),
    }
}

fn list_block(kind: &str, tag: &str, items: &[String]) -> String {
    format!(
        r#"<article class="rich-block"><{tag} class="block-list" data-kind="{kind}">{}</{tag}></article>"#,
        items
            .iter()
            .map(|item| format!(
                r#"<li class="block-item" contenteditable="true" spellcheck="true">{}</li>"#,
                html_escape(item)
            ))
            .collect::<Vec<_>>()
            .join("")
    )
}

fn text_html(lines: &[String]) -> String {
    html_escape(&lines.join("\n")).replace('\n', "<br>")
}
