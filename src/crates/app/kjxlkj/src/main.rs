//! kjxlkj - A Neovim-inspired terminal editor.
//!
//! This is the main binary crate.

mod app;
mod channels;
mod cli;
mod signals;

use app::App;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    // Initialize logging.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting kjxlkj");

    // Parse CLI arguments.
    let args = cli::parse();

    // Build and run the app.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let app = App::new(args)?;
        app.run().await
    })
}
