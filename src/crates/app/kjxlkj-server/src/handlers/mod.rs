mod auth;
mod admin;
mod automation;
mod health;
mod notes;
mod ui;
mod users;
mod views;
mod workspaces;
mod ws;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(health::configure_api)
            .configure(auth::configure)
            .configure(admin::configure)
            .configure(automation::configure)
            .configure(notes::configure)
            .configure(users::configure)
            .configure(views::configure)
            .configure(workspaces::configure),
    )
    .configure(health::configure_root)
    .configure(ui::configure_root)
    .configure(ws::configure_root);
}
