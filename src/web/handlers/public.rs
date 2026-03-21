use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

use crate::core::render;
use crate::web::handlers::common::{html, is_admin};
use crate::web::state::AppState;
use crate::web::templates::{PublicArticleTemplate, PublicIndexTemplate};

#[get("/")]
pub async fn index(state: web::Data<AppState>) -> impl Responder {
    match state.content.list(false).await {
        Ok(articles) => html(PublicIndexTemplate { articles }.render()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/article/{slug:.*}")]
pub async fn article(
    path: web::Path<String>,
    state: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    let slug = path.into_inner();
    let admin = match is_admin(&session, state.get_ref()).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    match state.content.get(&slug).await {
        Ok(Some(article)) if !article.private || admin => {
            let template = PublicArticleTemplate {
                title: article.title,
                body_html: render::markdown_to_html(&article.body),
            };
            html(template.render())
        }
        Ok(_) => HttpResponse::NotFound().body("not_found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
