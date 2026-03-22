use actix_web::{web, HttpRequest, HttpResponse};

use crate::core::content::VisibilityContext;
use crate::web::handlers::app_shell::render_shell_page;
use crate::web::handlers::article_edit::{render_inline_editor, render_inline_editor_script};
use crate::web::handlers::article_history_page::render_history_main;
use crate::web::handlers::article_page::{render_article_main, ArticleMainModel};
use crate::web::handlers::common::{has_admin_user, internal_error, redirect_to_setup};
use crate::web::handlers::home_page::render_home_main;
use crate::web::handlers::time_format::format_utc_timestamp;
use crate::web::render::render_markdown_html;
use crate::web::session::valid_session;
use crate::web::state::WebState;

pub async fn handle_get_home(request: HttpRequest, state: web::Data<WebState>) -> HttpResponse {
    match has_admin_user(&state).await {
        Ok(false) => return redirect_to_setup(),
        Ok(true) => {}
        Err(response) => return response,
    }

    let context = match valid_session(&request, &state).await {
        Ok(Some(_)) => VisibilityContext::Admin,
        Ok(None) => VisibilityContext::Public,
        Err(error) => return internal_error(error),
    };

    let is_admin = matches!(context, VisibilityContext::Admin);
    match visible_articles(&state, is_admin).await {
        Ok(articles) => {
            let settings = match state.settings_store.load_settings().await {
                Ok(settings) => settings,
                Err(error) => return internal_error(error),
            };
            let slugs = articles
                .iter()
                .map(|article| article.slug.clone())
                .collect::<Vec<_>>();
            let main = render_home_main(&articles, is_admin);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(render_shell_page(
                    &settings.site_title,
                    "Articles",
                    &main,
                    &slugs,
                    is_admin,
                ))
        }
        Err(error) => internal_error(error),
    }
}

pub async fn handle_get_article(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    let slug = slug.into_inner();
    let is_admin = match valid_session(&request, &state).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(error) => return internal_error(error),
    };

    let parsed = match state.content_store.read_article(&slug).await {
        Ok(parsed) => parsed,
        Err(crate::error::AppError::ContentIo { source, .. })
            if source.kind() == std::io::ErrorKind::NotFound =>
        {
            return HttpResponse::NotFound().finish()
        }
        Err(error) => return internal_error(error),
    };

    if parsed.frontmatter.private && !is_admin {
        return HttpResponse::NotFound().finish();
    }

    let html = render_markdown_html(&parsed.body);
    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let articles = match visible_articles(&state, is_admin).await {
        Ok(articles) => articles,
        Err(error) => return internal_error(error),
    };
    let slugs = articles
        .iter()
        .map(|article| article.slug.clone())
        .collect::<Vec<_>>();
    let summary = articles.iter().find(|item| item.slug == slug);
    let updated_at = summary
        .map(|item| format_utc_timestamp(item.updated_at))
        .unwrap_or_else(|| "unknown".to_owned());
    let nav = match state
        .content_store
        .article_navigation(&slug, is_admin)
        .await
    {
        Ok(nav) => nav,
        Err(error) => return internal_error(error),
    };
    let markdown =
        crate::core::content::serialize_markdown_document(&parsed.frontmatter, &parsed.body);
    let revision = crate::core::content::revision_token(&markdown);
    let inline = if is_admin {
        Some(render_inline_editor(
            &slug,
            parsed.frontmatter.title.as_deref(),
            &parsed.body,
            parsed.frontmatter.private,
            &revision,
            "Ready to edit inline.",
        ))
    } else {
        None
    };
    let script = if is_admin {
        Some(render_inline_editor_script())
    } else {
        None
    };
    let main = render_article_main(&ArticleMainModel {
        slug: &slug,
        html: &html,
        updated_at: &updated_at,
        previous_slug: nav.previous_slug.as_deref(),
        next_slug: nav.next_slug.as_deref(),
        is_admin,
        inline_editor: inline.as_deref(),
        autosave_script: script,
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_shell_page(
            &settings.site_title,
            &slug,
            &main,
            &slugs,
            is_admin,
        ))
}

pub async fn handle_get_article_history(
    request: HttpRequest,
    state: web::Data<WebState>,
    slug: web::Path<String>,
) -> HttpResponse {
    let slug = slug.into_inner();
    let is_admin = match valid_session(&request, &state).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(error) => return internal_error(error),
    };
    if !is_admin {
        return HttpResponse::NotFound().finish();
    }
    let history = match state.content_store.article_history(&slug).await {
        Ok(history) => history,
        Err(error) => return internal_error(error),
    };
    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let articles = match visible_articles(&state, true).await {
        Ok(articles) => articles,
        Err(error) => return internal_error(error),
    };
    let slugs = articles
        .iter()
        .map(|article| article.slug.clone())
        .collect::<Vec<_>>();
    let main = render_history_main(&history, true);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_shell_page(
            &settings.site_title,
            &format!("History · {slug}"),
            &main,
            &slugs,
            true,
        ))
}

async fn visible_articles(
    state: &web::Data<WebState>,
    is_admin: bool,
) -> Result<Vec<crate::web::state::ArticleSummary>, crate::error::AppError> {
    if is_admin {
        state.content_store.list_admin_articles().await
    } else {
        state.content_store.list_public_articles().await
    }
}
