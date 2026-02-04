//! kjxlkj - Neovim-inspired TUI text editor.
//!
//! Single binary entrypoint that wires host, core, render, and services.

mod headless;

use std::env;

use anyhow::Result;

use kjxlkj_host::TerminalHost;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--headless".to_string()) {
        return run_headless(&args);
    }

    let mut host = if args.len() > 1 {
        TerminalHost::open_path(&args[1])?
    } else {
        TerminalHost::new()?
    };
    host.run()?;

    Ok(())
}

fn run_headless(args: &[String]) -> Result<()> {
    let script_idx = args
        .iter()
        .position(|a| a == "--script")
        .map(|i| i + 1);

    if let Some(idx) = script_idx {
        if idx < args.len() {
            let script_path = &args[idx];
            return headless::run_script(script_path);
        }
    }

    anyhow::bail!("--headless requires --script <path>");
}
