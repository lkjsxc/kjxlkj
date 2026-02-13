/// Application configuration loaded from environment variables.
pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub static_dir: String,
    pub csrf_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://kjxlkj:kjxlkj@localhost:5432/kjxlkj".into()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            static_dir: std::env::var("STATIC_DIR")
                .unwrap_or_else(|_| "./static".into()),
            csrf_secret: std::env::var("CSRF_SECRET")
                .unwrap_or_else(|_| "default-csrf-secret-change-me".into()),
        }
    }
}
