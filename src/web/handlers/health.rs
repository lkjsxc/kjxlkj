use actix_web::HttpResponse;

pub async fn handle_get_healthz() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("ok")
}
