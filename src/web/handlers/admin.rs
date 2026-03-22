use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::{internal_error, require_admin_session};
use crate::web::state::WebState;

use super::admin_page::render_admin_shell;

const HTML_CONTENT_TYPE: &str = "text/html; charset=utf-8";

pub use super::admin_actions::{handle_post_admin_create, handle_post_admin_rename};
pub use super::admin_mutation::handle_post_admin_delete;

pub async fn handle_get_admin_shell(
    request: HttpRequest,
    state: web::Data<WebState>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    match state.content_store.list_admin_articles().await {
        Ok(articles) => {
            let settings = match state.settings_store.load_settings().await {
                Ok(settings) => settings,
                Err(error) => return internal_error(error),
            };
            let slugs = articles
                .iter()
                .map(|article| article.slug.clone())
                .collect::<Vec<_>>();
            HttpResponse::Ok()
                .content_type(HTML_CONTENT_TYPE)
                .body(render_admin_shell(
                    &settings.site_title,
                    &slugs,
                    chrono::Utc::now(),
                ))
        }
        Err(error) => internal_error(error),
    }
}
