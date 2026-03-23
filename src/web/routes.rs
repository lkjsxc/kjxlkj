use actix_web::web;

use super::handlers::{delete_record, get_health, get_record, list_records, put_record};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(get_health))
        .route("/v1/records", web::get().to(list_records))
        .route("/v1/records/{id}", web::get().to(get_record))
        .route("/v1/records/{id}", web::put().to(put_record))
        .route("/v1/records/{id}", web::delete().to(delete_record));
}
