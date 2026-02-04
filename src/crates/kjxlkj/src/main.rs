//! kjxlkj - A Neovim-inspired TUI text editor.

mod app;
mod args;
mod headless;

use anyhow::Result;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.headless {
        headless::run(&args)
    } else {
        app::run(&args)
    }
}
