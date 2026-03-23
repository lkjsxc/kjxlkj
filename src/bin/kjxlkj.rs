use std::process::ExitCode;

use kjxlkj::cli::run_cli;

#[tokio::main]
async fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match run_cli(&args).await {
        Ok(result) if result.is_success() => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"error":error.code(),"message":error.to_string()})
            );
            ExitCode::FAILURE
        }
    }
}
