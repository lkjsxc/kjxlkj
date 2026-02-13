use actix_web::{web, HttpResponse};

const INDEX_HTML: &str = include_str!("../../static/index.html");

pub fn configure_root(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index))
        .route("/app", web::get().to(index));
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(INDEX_HTML)
}
