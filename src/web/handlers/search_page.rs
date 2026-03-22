use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::web::handlers::app_shell::render_shell_page;
use crate::web::handlers::common::{enforce_setup_completion, internal_error};
use crate::web::handlers::page_html::escape_html;
use crate::web::session::valid_session;
use crate::web::state::WebState;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
}

pub async fn handle_get_search(
    request: HttpRequest,
    state: web::Data<WebState>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    if let Err(response) = enforce_setup_completion(&state).await {
        return response;
    }
    let is_admin = match valid_session(&request, &state).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(error) => return internal_error(error),
    };
    let settings = match state.settings_store.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return internal_error(error),
    };
    let slugs = match visible_slugs(&state, is_admin).await {
        Ok(slugs) => slugs,
        Err(error) => return internal_error(error),
    };
    let q = query.q.clone().unwrap_or_default();
    let hits = if q.trim().is_empty() {
        Vec::new()
    } else {
        match state.content_store.search_articles(&q, is_admin).await {
            Ok(hits) => hits,
            Err(error) => return internal_error(error),
        }
    };
    let main = render_search_main(&q, &hits);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(render_shell_page(
            &settings.site_title,
            "Search",
            &main,
            &slugs,
            is_admin,
        ))
}

pub fn render_search_main(query: &str, hits: &[crate::web::state::SearchHit]) -> String {
    render_search_main_impl(query, hits)
}

fn render_search_main_impl(query: &str, hits: &[crate::web::state::SearchHit]) -> String {
    let result_rows = if query.trim().is_empty() {
        "<p>Enter a query to search.</p>".to_owned()
    } else if hits.is_empty() {
        "<p>No matching articles.</p>".to_owned()
    } else {
        hits.iter()
            .map(|hit| {
                let private_badge = if hit.private { " <span data-private=\"true\">private</span>" } else { "" };
                format!(
                    "<li><a href=\"/article/{slug}\">{slug}</a>{private_badge}<p>{title}</p><p>{snippet}</p></li>",
                    slug = escape_html(&hit.slug),
                    title = escape_html(hit.title.as_deref().unwrap_or("")),
                    snippet = escape_html(&hit.snippet),
                )
            })
            .collect::<String>()
    };
    format!(
        "<main id=\"search-page\"><h1>Search</h1><form id=\"search-form\" method=\"get\" action=\"/search\"><input id=\"search-query\" name=\"q\" type=\"search\" value=\"{}\" /><button type=\"submit\">Search</button></form><section id=\"search-results\"><ul>{}</ul></section></main>",
        escape_html(query),
        result_rows
    )
}

async fn visible_slugs(
    state: &web::Data<WebState>,
    is_admin: bool,
) -> Result<Vec<String>, crate::error::AppError> {
    if is_admin {
        state.content_store.list_admin_slugs().await
    } else {
        state.content_store.list_public_slugs().await
    }
}
