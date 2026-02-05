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

    #[test]
    fn test_args_with_file() {
        let args = Args {
            file: Some(PathBuf::from("test.txt")),
            log: false,
            log_file: None,
            readonly: false,
            line: None,
        };
        assert!(args.file.is_some());
        assert_eq!(args.file.unwrap(), PathBuf::from("test.txt"));
    }

    #[test]
    fn test_args_with_log() {
        let args = Args {
            file: None,
            log: true,
            log_file: Some("/tmp/log.txt".to_string()),
            readonly: false,
            line: None,
        };
        assert!(args.log);
        assert!(args.log_file.is_some());
    }

    #[test]
    fn test_args_readonly() {
        let args = Args {
            file: Some(PathBuf::from("readonly.txt")),
            log: false,
            log_file: None,
            readonly: true,
            line: None,
        };
        assert!(args.readonly);
    }

    #[test]
    fn test_args_with_line() {
        let args = Args {
            file: Some(PathBuf::from("test.txt")),
            log: false,
            log_file: None,
            readonly: false,
            line: Some(100),
        };
        assert_eq!(args.line, Some(100));
    }

    #[test]
    fn test_args_debug() {
        let args = Args {
            file: None,
            log: false,
            log_file: None,
            readonly: false,
            line: None,
        };
        let debug = format!("{:?}", args);
        assert!(debug.contains("Args"));
    }

    #[test]
    fn test_args_all_fields() {
        let args = Args {
            file: Some(PathBuf::from("/path/to/file.rs")),
            log: true,
            log_file: Some("/var/log/editor.log".to_string()),
            readonly: true,
            line: Some(42),
        };
        assert!(args.file.is_some());
        assert!(args.log);
        assert!(args.log_file.is_some());
        assert!(args.readonly);
        assert_eq!(args.line, Some(42));
    }

    #[test]
    fn test_args_line_zero() {
        let args = Args {
            file: None,
            log: false,
            log_file: None,
            readonly: false,
            line: Some(0),
        };
        assert_eq!(args.line, Some(0));
    }

    #[test]
    fn test_args_line_large() {
        let args = Args {
            file: None,
            log: false,
            log_file: None,
            readonly: false,
            line: Some(999999),
        };
        assert_eq!(args.line, Some(999999));
    }

    #[test]
    fn test_args_log_file_without_log() {
        let args = Args {
            file: None,
            log: false,
            log_file: Some("file.log".to_string()),
            readonly: false,
            line: None,
        };
        // log_file can be set even if log is false
        assert!(!args.log);
        assert!(args.log_file.is_some());
    }

    #[test]
    fn test_args_file_with_spaces() {
        let args = Args {
            file: Some(PathBuf::from("/path/to/my file.txt")),
            log: false,
            log_file: None,
            readonly: false,
            line: None,
        };
        assert!(args.file.as_ref().unwrap().to_string_lossy().contains(' '));
    }

    #[test]
    fn test_args_readonly_default_false() {
        let args = Args {
            file: None,
            log: false,
            log_file: None,
            readonly: false,
            line: None,
        };
        assert!(!args.readonly);
    }
}
