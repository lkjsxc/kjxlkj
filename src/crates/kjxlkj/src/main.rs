//! kjxlkj - A Neovim-inspired TUI text editor.

mod app;
mod cli;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = cli::parse();

    // Initialize logging
    if let Some(log_file) = &args.log_file {
        init_logging(log_file)?;
    }

    // Run the application
    let app = app::App::new(args)?;
    app.run().await
}

/// Initializes logging.
fn init_logging(path: &str) -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let file = std::fs::File::create(path)?;
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with(fmt::layer().with_writer(file).with_ansi(false));

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
