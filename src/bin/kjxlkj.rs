use std::process::ExitCode;

use kjxlkj::cli::run_cli;
use kjxlkj::error::AppError;

#[tokio::main]
async fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    match run_cli(&args).await {
        Ok(result) => {
            if result.is_success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Err(error) => {
            print_error(&error);
            ExitCode::FAILURE
        }
    }
}

fn print_error(error: &AppError) {
    eprintln!(
        "{{\"error\":\"{}\",\"message\":\"{}\"}}",
        error.code(),
        error
    );
}
