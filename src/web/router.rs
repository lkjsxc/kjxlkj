use actix_web::web;

use crate::web::handlers::{admin, admin_preview, auth, public, setup, static_assets};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(configure_setup_routes)
        .configure(configure_auth_routes)
        .configure(configure_public_routes)
        .configure(configure_admin_routes)
        .configure(configure_static_routes);
}

fn configure_setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/setup")
            .route(web::get().to(setup::handle_get_setup))
            .route(web::post().to(setup::handle_post_setup)),
    );
}

fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::get().to(auth::handle_get_login))
            .route(web::post().to(auth::handle_post_login)),
    )
    .service(web::resource("/logout").route(web::post().to(auth::handle_post_logout)));
}

fn configure_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(public::handle_get_home)))
        .service(web::resource("/article/{slug}").route(web::get().to(public::handle_get_article)));
}

fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("", web::get().to(admin::handle_get_admin_shell))
            .route("/", web::get().to(admin::handle_get_admin_shell))
            .route("/open/{slug}", web::get().to(admin::handle_get_admin_open))
            .route(
                "/preview",
                web::post().to(admin_preview::handle_post_admin_preview),
            )
            .route("/create", web::post().to(admin::handle_post_admin_create))
            .route("/save", web::post().to(admin::handle_post_admin_save))
            .route("/rename", web::post().to(admin::handle_post_admin_rename))
            .route(
                "/delete/{slug}",
                web::post().to(admin::handle_post_admin_delete),
            )
            .route(
                "/toggle-private/{slug}",
                web::post().to(admin::handle_post_admin_toggle_private),
            ),
    );
}

fn configure_static_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/static/admin-runtime-core.js")
            .route(web::get().to(static_assets::handle_get_admin_runtime_core_js)),
    )
    .service(
        web::resource("/static/admin-runtime-autosave.js")
            .route(web::get().to(static_assets::handle_get_admin_runtime_autosave_js)),
    )
    .service(
        web::resource("/static/admin-runtime-shortcuts.js")
            .route(web::get().to(static_assets::handle_get_admin_runtime_shortcuts_js)),
    );
}
