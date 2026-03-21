use std::sync::Arc;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};

use crate::adapters::{auth_store::PgAuthStore, content_store::FsContentStore};
use crate::web::{handlers, state::AppState};

pub async fn run(bind_addr: &str, db_url: &str) -> anyhow::Result<()> {
    let auth = Arc::new(PgAuthStore::connect(db_url).await?);
    let content = Arc::new(FsContentStore::new("content/articles"));
    let state = web::Data::new(AppState { auth, content });
    let key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                key.clone(),
            ))
            .service(actix_files::Files::new("/static", "static"))
            .service(handlers::index)
            .service(handlers::article)
            .service(handlers::setup_get)
            .service(handlers::setup_post)
            .service(handlers::login_get)
            .service(handlers::login_post)
            .service(handlers::logout_post)
            .service(
                web::scope("/admin")
                    .service(handlers::admin_home)
                    .service(handlers::admin_open)
                    .service(handlers::admin_save)
                    .service(handlers::admin_create)
                    .service(handlers::admin_toggle_private)
                    .service(handlers::admin_rename)
                    .service(handlers::admin_delete),
            )
    })
    .bind(bind_addr)?
    .run()
    .await?;

    Ok(())
}
