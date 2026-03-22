use actix_web::web;

use crate::web::handlers::{
    admin, article_edit, auth, public, search_page, settings_page, setup, static_assets, trash_page,
};

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
        .service(web::resource("/search").route(web::get().to(search_page::handle_get_search)))
        .service(web::resource("/article/{slug}").route(web::get().to(public::handle_get_article)))
        .service(
            web::resource("/article/{slug}/edit")
                .route(web::post().to(article_edit::handle_post_article_edit)),
        )
        .service(
            web::resource("/article/{slug}/history")
                .route(web::get().to(public::handle_get_article_history)),
        )
        .service(
            web::resource("/article/{slug}/history/restore")
                .route(web::post().to(article_edit::handle_post_article_restore)),
        );
}

fn configure_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .route("", web::get().to(admin::handle_get_admin_shell))
            .route("/", web::get().to(admin::handle_get_admin_shell))
            .route("/create", web::post().to(admin::handle_post_admin_create))
            .route("/rename", web::post().to(admin::handle_post_admin_rename))
            .route(
                "/settings",
                web::get().to(settings_page::handle_get_admin_settings),
            )
            .route(
                "/settings/save",
                web::post().to(settings_page::handle_post_admin_settings_save),
            )
            .route(
                "/settings/reindex",
                web::post().to(settings_page::handle_post_admin_settings_reindex),
            )
            .route("/trash", web::get().to(trash_page::handle_get_admin_trash))
            .route(
                "/trash/restore/{slug}",
                web::post().to(trash_page::handle_post_admin_trash_restore),
            )
            .route(
                "/trash/delete-permanent/{slug}",
                web::post().to(trash_page::handle_post_admin_trash_delete_permanent),
            )
            .route(
                "/delete/{slug}",
                web::post().to(admin::handle_post_admin_delete),
            ),
    );
}

fn configure_static_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/static/app-shell.js")
            .route(web::get().to(static_assets::handle_get_app_shell_js)),
    )
    .service(
        web::resource("/static/app.css").route(web::get().to(static_assets::handle_get_app_css)),
    );
}
