//! CLI argument parsing.

use std::path::PathBuf;

/// CLI arguments.
pub struct Args {
    /// Files to open.
    pub files: Vec<PathBuf>,
    /// Log file path.
    pub log_file: Option<String>,
    /// Headless mode.
    pub headless: bool,
    /// Dump mode - render once and dump output.
    pub dump: bool,
}

/// Parses CLI arguments.
pub fn parse() -> Args {
    let mut args = Args {
        files: Vec::new(),
        log_file: None,
        headless: false,
        dump: false,
    };

    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--log" => {
                args.log_file = iter.next();
            }
            "--headless" => {
                args.headless = true;
            }
            "--dump" => {
                args.dump = true;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            "--version" | "-v" => {
                print_version();
                std::process::exit(0);
            }
            _ => {
                if !arg.starts_with('-') {
                    args.files.push(PathBuf::from(arg));
                }
            }
        }
    }

    args
}

/// Prints help.
fn print_help() {
    println!("kjxlkj - A Neovim-inspired TUI text editor");
    println!();
    println!("Usage: kjxlkj [OPTIONS] [FILES]...");
    println!();
    println!("Options:");
    println!("  --log <FILE>   Write logs to FILE");
    println!("  --headless     Run in headless mode");
    println!("  --dump         Dump render output and exit");
    println!("  -h, --help     Print help");
    println!("  -v, --version  Print version");
}

/// Prints version.
fn print_version() {
    println!("kjxlkj {}", env!("CARGO_PKG_VERSION"));
}
