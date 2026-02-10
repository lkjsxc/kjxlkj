//! kjxlkj – a Neovim-inspired terminal editor.
//!
//! This binary crate contains only startup handoff.

mod app;
mod channels;
mod cli;
mod services;
mod signals;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();

    // Initialize tracing with env-filter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("kjxlkj starting");

    // Build and run the async runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    runtime.block_on(app::run(args))?;

    Ok(())
}
