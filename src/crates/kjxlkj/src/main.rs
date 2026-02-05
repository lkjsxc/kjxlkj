//! kjxlkj - A Neovim-inspired TUI text editor.
//!
//! This is the main binary entry point.

use std::path::PathBuf;

mod headless;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let mut headless = false;
    let mut script: Option<PathBuf> = None;
    let mut file: Option<PathBuf> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--headless" => headless = true,
            "--script" => {
                let Some(path) = args.next() else {
                    eprintln!("Error: --script requires a path argument");
                    std::process::exit(2);
                };
                headless = true;
                script = Some(PathBuf::from(path));
            }
            "--help" | "-h" => {
                eprintln!("Usage: kjxlkj [--headless] [--script PATH] [FILE]");
                std::process::exit(0);
            }
            _ => {
                if file.is_none() {
                    file = Some(PathBuf::from(arg));
                } else {
                    eprintln!("Error: unexpected extra argument: {arg}");
                    std::process::exit(2);
                }
            }
        }
    }

    if headless {
        if let Err(e) = headless::run_headless(file, script) {
            eprintln!("Error: {e:#}");
            std::process::exit(1);
        }
        return;
    }

    if let Err(e) = kjxlkj_host::run(file) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
