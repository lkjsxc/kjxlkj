#![forbid(unsafe_code)]

use anyhow::Context;
use clap::Parser;
use kjxlkj_core::{CoreAction, EditorState, Mode};
use kjxlkj_host::TerminalGuard;
use kjxlkj_render::draw_frame;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    headless: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.headless {
        return run_headless().await;
    }
    run_tui().await
}

async fn run_headless() -> anyhow::Result<()> {
    let mut state = EditorState::default();
    state.apply(CoreAction::SetMode(Mode::Insert));
    state.apply(CoreAction::InsertText("kjxlkj".to_string()));
    println!("{}", state.snapshot().buffers[0].lines.join(""));
    Ok(())
}

async fn run_tui() -> anyhow::Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).context("create terminal")?;

    let (tx, mut rx) = mpsc::channel::<CoreAction>(128);
    let _ = tx;

    let mut state = EditorState::default();
    state.apply(CoreAction::SetMode(Mode::Insert));
    while let Some(action) = rx.recv().await {
        state.apply(action);
        draw_frame(&mut terminal, &state.snapshot())?;
    }
    Ok(())
}

