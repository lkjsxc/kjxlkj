//! Command-line argument parsing.

use std::path::PathBuf;

/// Parsed command-line arguments.
pub struct CliArgs {
    /// Files to open on startup.
    pub files: Vec<PathBuf>,
}

/// Parse command-line arguments from std::env::args.
pub fn parse_args() -> CliArgs {
    let args: Vec<String> = std::env::args().collect();
    let files: Vec<PathBuf> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with('-'))
        .map(PathBuf::from)
        .collect();
    CliArgs { files }
}
