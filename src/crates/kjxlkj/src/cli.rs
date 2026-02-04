//! Command line argument parsing.

use clap::Parser;
use std::path::PathBuf;

/// kjxlkj - A modal text editor.
#[derive(Parser, Debug)]
#[command(name = "kjxlkj")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File to open.
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Enable logging.
    #[arg(short, long)]
    pub log: bool,

    /// Log file path.
    #[arg(long, value_name = "PATH")]
    pub log_file: Option<String>,

    /// Read-only mode.
    #[arg(short = 'R', long)]
    pub readonly: bool,

    /// Start at line number.
    #[arg(short = '+', value_name = "LINE")]
    pub line: Option<usize>,
}

impl Args {
    /// Parse command line arguments.
    pub fn parse() -> Self {
        Parser::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default() {
        // Test that Args can be constructed
        let args = Args {
            file: None,
            log: false,
            log_file: None,
            readonly: false,
            line: None,
        };
        assert!(args.file.is_none());
        assert!(!args.log);
    }
}
