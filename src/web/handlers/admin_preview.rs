use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::web::handlers::common::require_admin_session;
use crate::web::state::WebState;

use super::admin_fragments::{
    html_fragment_response, render_admin_preview, render_admin_validation_banner,
};
use super::admin_input::{normalize_slug_input, PreviewForm};

pub async fn handle_post_admin_preview(
    request: HttpRequest,
    state: web::Data<WebState>,
    form: web::Form<PreviewForm>,
) -> HttpResponse {
    if let Err(response) = require_admin_session(&request, &state).await {
        return response;
    }

    if let Err(message) = normalize_slug_input(&form.slug, "slug") {
        return html_fragment_response(
            StatusCode::BAD_REQUEST,
            render_admin_validation_banner(&message),
        );
    }

    html_fragment_response(StatusCode::OK, render_admin_preview(&form.body, false))
}
