use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::core::content::{revision_token, serialize_markdown_document};
use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::handlers::page_html::escape_html;
use crate::web::state::WebState;

use super::time_format::format_utc_timestamp;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

#[derive(Debug, Deserialize)]
pub struct EditForm {
    pub title: Option<String>,
    pub body: String,
    pub private: Option<bool>,
    pub last_known_revision: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RestoreForm {
    pub commit_id: String,
}

pub async fn handle_post_article_edit(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
    form: web::Form<EditForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    let slug = slug.into_inner();
    let private = form.private.unwrap_or(true);
    let outcome = match state
        .content_store
        .save_article(
            &slug,
            normalized_title(&form.title),
            &form.body,
            private,
            form.last_known_revision.as_deref(),
        )
        .await
    {
        Ok(outcome) => outcome,
        Err(error) => return internal_error(error),
    };
    let parsed = match state.content_store.read_article(&slug).await {
        Ok(parsed) => parsed,
        Err(error) => return internal_error(error),
    };
    let markdown = serialize_markdown_document(&parsed.frontmatter, &parsed.body);
    let revision = revision_token(&markdown);
    let status = format!(
        "Saved at {} (revision {}).",
        format_utc_timestamp(outcome.updated_at),
        escape_html(&revision)
    );
    HttpResponse::Ok()
        .content_type(HTML_CONTENT_TYPE)
        .body(render_inline_editor(
            &slug,
            parsed.frontmatter.title.as_deref(),
            &parsed.body,
            parsed.frontmatter.private,
            &revision,
            &status,
        ))
}

pub async fn handle_post_article_restore(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
    form: web::Form<RestoreForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }
    let slug = slug.into_inner();
    if let Err(error) = state
        .content_store
        .restore_article_version(&slug, form.commit_id.trim())
        .await
    {
        return internal_error(error);
    }
    HttpResponse::SeeOther()
        .append_header(("Location", format!("/article/{slug}")))
        .finish()
}

pub fn render_inline_editor(
    slug: &str,
    title: Option<&str>,
    body: &str,
    private: bool,
    revision: &str,
    status: &str,
) -> String {
    let title = title.unwrap_or_default();
    let checked = if private { " checked" } else { "" };
    format!(
        "<section id=\"article-inline-editor\"><h2>Inline edit</h2><section id=\"article-edit-status\" aria-live=\"polite\">{}</section><form id=\"article-edit-form\" method=\"post\" action=\"/article/{}/edit\" hx-post=\"/article/{}/edit\" hx-target=\"#article-inline-editor\" hx-swap=\"outerHTML\"><label for=\"title\">Title</label><input id=\"title\" name=\"title\" type=\"text\" value=\"{}\" /><label for=\"private\">Private</label><input id=\"private\" name=\"private\" type=\"checkbox\" value=\"true\"{} /><label for=\"body\">Body</label><textarea id=\"body\" name=\"body\" rows=\"22\">{}</textarea><input id=\"last_known_revision\" name=\"last_known_revision\" type=\"hidden\" value=\"{}\" /></form></section>",
        escape_html(status),
        escape_html(slug),
        escape_html(slug),
        escape_html(title),
        checked,
        escape_html(body),
        escape_html(revision)
    )
}

pub fn render_inline_editor_script() -> &'static str {
    r#"<script>
(() => {
  const form = document.getElementById("article-edit-form");
  if (!form) return;
  let dirty = false;
  let timer = null;
  const save = () => {
    if (!dirty) return;
    if (window.htmx) {
      window.htmx.trigger(form, "submit");
    } else {
      form.submit();
    }
    dirty = false;
  };
  const schedule = () => {
    clearTimeout(timer);
    timer = setTimeout(save, 2000);
  };
  form.addEventListener("input", () => {
    dirty = true;
    schedule();
  });
  form.addEventListener("change", () => {
    dirty = true;
    schedule();
  });
  form.addEventListener("blur", () => {
    if (dirty) save();
  }, true);
  window.addEventListener("beforeunload", (event) => {
    if (!dirty) return;
    save();
    event.preventDefault();
    event.returnValue = "";
  });
})();
</script>"#
}

fn normalized_title(value: &Option<String>) -> Option<String> {
    let trimmed = value.as_deref().map(str::trim).unwrap_or_default();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}
