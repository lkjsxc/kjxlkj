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
    let mut file = None;
    let mut headless = false;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--headless" => headless = true,
            _ if !arg.starts_with('-') => file = Some(PathBuf::from(arg)),
            other => {
                eprintln!("Unknown option: {}", other);
                std::process::exit(1);
            }
        }
    }

    // Run the editor
    let result = if headless {
        kjxlkj_host::run_headless(file)
    } else {
        kjxlkj_host::run(file)
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
