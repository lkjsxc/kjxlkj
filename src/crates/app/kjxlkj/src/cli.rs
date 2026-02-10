//! CLI argument parsing.

use std::path::PathBuf;

/// Parsed CLI arguments.
#[derive(Debug, Default)]
pub struct Args {
    /// Files to open.
    pub files: Vec<PathBuf>,
    /// Start in read-only mode.
    pub read_only: bool,
    /// Run in headless mode.
    pub headless: bool,
}

/// Parse command line arguments.
pub fn parse() -> Args {
    let mut args = Args::default();

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-R" | "--readonly" => {
                args.read_only = true;
            }
            "--headless" => {
                args.headless = true;
            }
            s if !s.starts_with('-') => {
                args.files.push(PathBuf::from(s));
            }
            _ => {
                // Ignore unknown flags.
            }
        }
    }

    args
}
