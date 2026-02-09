//! CLI argument parsing and application entry point.

use clap::Parser;
use std::path::PathBuf;

/// kjxlkj — a terminal text editor
#[derive(Debug, Parser)]
#[command(name = "kjxlkj", version, about)]
pub struct Cli {
    /// Files to open.
    #[arg()]
    pub files: Vec<PathBuf>,

    /// Open in diff mode.
    #[arg(short = 'd', long)]
    pub diff: bool,

    /// Start in read-only mode.
    #[arg(short = 'R', long)]
    pub readonly: bool,

    /// Execute command after startup.
    #[arg(short = 'c', long)]
    pub command: Option<String>,

    /// Start at specific line.
    #[arg(short = '+', long = "line")]
    pub start_line: Option<usize>,

    /// Log level.
    #[arg(long, default_value = "warn")]
    pub log_level: String,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_cli() {
        let cli = Cli::try_parse_from(["kjxlkj"]).unwrap();
        assert!(cli.files.is_empty());
        assert!(!cli.diff);
        assert!(!cli.readonly);
    }

    #[test]
    fn cli_with_file() {
        let cli = Cli::try_parse_from(["kjxlkj", "test.txt"]).unwrap();
        assert_eq!(cli.files.len(), 1);
    }
}
