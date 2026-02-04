//! kjxlkj - A Vim-like terminal editor.

mod app;
mod handlers;
mod headless;

use anyhow::Result;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    let mut file_path: Option<PathBuf> = None;
    let mut headless = false;
    let mut script_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--headless" => headless = true,
            "--script" => {
                i += 1;
                if i < args.len() {
                    script_path = Some(PathBuf::from(&args[i]));
                }
            }
            arg if !arg.starts_with('-') => {
                file_path = Some(PathBuf::from(arg));
            }
            _ => {}
        }
        i += 1;
    }

    if headless {
        headless::run_headless(file_path, script_path)
    } else {
        app::run_interactive(file_path)
    }
}
