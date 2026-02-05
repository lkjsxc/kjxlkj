//! kjxlkj - A terminal text editor.

use std::path::PathBuf;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let file = args.get(1).map(PathBuf::from);

    // Run the editor
    if let Err(e) = kjxlkj_host::run(file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
