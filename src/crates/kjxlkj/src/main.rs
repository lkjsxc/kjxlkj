//! kjxlkj - A modal terminal editor.

mod app;
mod command;
mod handler;
mod headless;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing.
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();

    // Check for headless mode.
    if args.contains(&"--headless".to_string()) {
        return headless::run_headless(&args).await;
    }

    // Get file to open.
    let file = args.get(1).map(|s| s.as_str());

    // Run the editor.
    app::run(file).await
}
