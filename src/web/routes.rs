use actix_web::web;

use super::admin::get_admin;
use super::handlers::{delete_record, get_health, get_record, list_records, put_record};
use super::home::get_home;
use super::login::{get_login, post_login, post_logout};
use super::setup::{get_setup, post_setup};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthz", web::get().to(get_health))
        .route("/", web::get().to(get_home))
        .service(
            web::resource("/setup")
                .route(web::get().to(get_setup))
                .route(web::post().to(post_setup)),
        )
        .service(
            web::resource("/login")
                .route(web::get().to(get_login))
                .route(web::post().to(post_login)),
        )
        .route("/logout", web::post().to(post_logout))
        .route("/admin", web::get().to(get_admin))
        .route("/admin/", web::get().to(get_admin))
        .route("/v1/records", web::get().to(list_records))
        .route("/v1/records/{id}", web::get().to(get_record))
        .route("/v1/records/{id}", web::put().to(put_record))
        .route("/v1/records/{id}", web::delete().to(delete_record));
}
