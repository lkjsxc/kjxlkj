//! kjxlkj - Neovim-inspired TUI text editor
//!
//! This is the main binary entrypoint.

mod app;
mod command;
mod headless;

use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for headless mode
    if args.iter().any(|a| a == "--headless") {
        return headless::run_headless(&args);
    }

    // Run the TUI application
    app::run(args)
}
