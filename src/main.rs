use std::process::ExitCode;

use kjxlkj::config::AppConfig;
use kjxlkj::error::AppError;
use kjxlkj::web;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            print_error(&error);
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), AppError> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "serve".to_owned());

    match command.as_str() {
        "serve" => {
            let config = AppConfig::from_env()?;
            web::run_http_server(config).await
        }
        _ => Err(AppError::unsupported_command(command)),
    }
}

fn print_error(error: &AppError) {
    eprintln!(
        "{{\"error\":\"{}\",\"message\":\"{}\"}}",
        error.code(),
        error
    );
}
