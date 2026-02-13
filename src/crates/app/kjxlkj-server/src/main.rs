use actix_web::{web, App, HttpServer};
use kjxlkj_server::app_state::AppState;
use kjxlkj_server::handlers;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
struct Config {
    database_url: String,
    bind_addr: String,
    secure_cookies: bool,
}

impl Config {
    fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/kjxlkj".to_owned());
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_owned());
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_owned());
        let secure_cookies = env::var("SECURE_COOKIES")
            .map(|value| value.eq_ignore_ascii_case("true"))
            .unwrap_or_else(|_| app_env.eq_ignore_ascii_case("production"));

        Self {
            database_url,
            bind_addr,
            secure_cookies,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    let config = Config::from_env();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .map_err(io_err)?;

    kjxlkj_db::migrations::run(&pool).await.map_err(io_err)?;

    let state = AppState::new(pool, config.secure_cookies);
    tracing::info!(bind_addr = %config.bind_addr, "starting kjxlkj server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(handlers::configure)
    })
    .bind(&config.bind_addr)?
    .run()
    .await
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

fn io_err(error: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::other(error.to_string())
}
