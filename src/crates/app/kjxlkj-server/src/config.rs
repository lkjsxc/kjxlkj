use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub session_ttl_days: i64,
    pub attachment_max_bytes: i64,
    pub attachment_chunk_bytes: usize,
    pub export_dir: String,
    pub backup_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bind_addr: env_var("BIND_ADDR", "0.0.0.0:8080"),
            database_url: env_var(
                "DATABASE_URL",
                "postgres://kjxlkj:kjxlkj@127.0.0.1:5432/kjxlkj",
            ),
            session_ttl_days: env_var("SESSION_TTL_DAYS", "7").parse().unwrap_or(7),
            attachment_max_bytes: env_var("ATTACHMENT_MAX_BYTES", "524288000")
                .parse()
                .unwrap_or(524_288_000),
            attachment_chunk_bytes: env_var("ATTACHMENT_CHUNK_BYTES", "4194304")
                .parse()
                .unwrap_or(4_194_304),
            export_dir: env_var("EXPORT_DIR", "/tmp/kjxlkj-exports"),
            backup_dir: env_var("BACKUP_DIR", "/tmp/kjxlkj-backups"),
        }
    }
}

fn env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
