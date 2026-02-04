//! kjxlkj - A modal text editor.
//!
//! Main entry point for the editor.

mod cli;

use anyhow::Result;
use cli::Args;
use kjxlkj_host::{Host, HostConfig};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    if args.log {
        init_logging(args.log_file.as_deref())?;
    }

    info!("kjxlkj starting");

    // Build host configuration
    let config = HostConfig {
        file: args.file,
        content: None,
    };

    // Run the editor
    let host = Host::new(config);
    host.run()?;

    info!("kjxlkj exiting");
    Ok(())
}

/// Initialize logging.
fn init_logging(log_file: Option<&str>) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    if let Some(path) = log_file {
        let file = std::fs::File::create(path)?;
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_writer(file))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_writer(std::io::stderr))
            .init();
    }

    Ok(())
}
