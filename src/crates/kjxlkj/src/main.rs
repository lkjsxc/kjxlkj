//! kjxlkj - A Neovim-inspired TUI text editor.
//!
//! This is the main binary entrypoint.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use kjxlkj_core::EditorState;
use kjxlkj_host::{run_headless, TerminalHost};

fn main() -> ExitCode {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let mut file_path: Option<PathBuf> = None;
    let mut headless = false;
    let mut script_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--headless" => {
                headless = true;
            }
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

    // Create editor state
    let editor = if let Some(path) = file_path {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        EditorState::with_file(path, &content)
    } else {
        EditorState::new()
    };

    // Run in appropriate mode
    if headless {
        let mut editor = editor;
        if let Some(script) = script_path {
            let result = run_headless(&mut editor, &script)?;
            if !result.quit {
                eprintln!("Script did not quit editor");
            }
        }
        Ok(())
    } else {
        let mut host = TerminalHost::new(editor)?;
        host.run()
    }
}

#[cfg(test)]
mod tests;
