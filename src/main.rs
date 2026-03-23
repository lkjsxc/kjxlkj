use std::process::ExitCode;

use kjxlkj::config::AppConfig;
use kjxlkj::error::AppError;
use kjxlkj::web;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"error":error.code(),"message":error.to_string()})
            );
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), AppError> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "serve".to_owned());
    if command != "serve" {
        return Err(AppError::UnsupportedCommand(command));
    }
    let config = AppConfig::from_env()?;
    web::run_http_server(config).await
}
