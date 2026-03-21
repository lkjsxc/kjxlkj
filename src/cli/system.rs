use clap::Subcommand;
use serde_json::json;
use sqlx::PgPool;

#[derive(Subcommand)]
pub enum SystemCmd {
    Doctor,
}

pub async fn run(command: SystemCmd) -> anyhow::Result<()> {
    match command {
        SystemCmd::Doctor => {
            let db_url = std::env::var("DATABASE_URL").ok();
            let db_reachable = if let Some(url) = db_url.clone() {
                PgPool::connect(&url)
                    .await
                    .map(|pool| {
                        drop(pool);
                        true
                    })
                    .unwrap_or(false)
            } else {
                false
            };
            println!(
                "{}",
                json!({
                    "ok": true,
                    "database_url_present": db_url.is_some(),
                    "database_reachable": db_reachable,
                    "content_dir_present": std::path::Path::new("content/articles").exists()
                })
            );
        }
    }
    Ok(())
}
