#![forbid(unsafe_code)]

use anyhow::Context;
use clap::Parser;
use kjxlkj_core::{CoreEvent, EditorState, Effect, ServiceResult};
use kjxlkj_host::TerminalGuard;
use kjxlkj_input::{map_event, InputEvent};
use kjxlkj_render::draw_frame;
use kjxlkj_services::{fs, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tokio::sync::{mpsc, watch};

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
    let state = EditorState::default();
    println!("{}", state.snapshot().buffers[0].lines.join(""));
    Ok(())
}

async fn run_tui() -> anyhow::Result<()> {
    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).context("create terminal")?;

    let (event_tx, event_rx) = mpsc::channel::<CoreEvent>(1024);
    spawn_input_thread(event_tx.clone());

    let (fs_tx, fs_rx) = mpsc::channel::<fs::FsRequest>(128);
    let (fs_res_tx, mut fs_res_rx) = mpsc::channel::<fs::FsResponse>(128);
    let _fs_task = fs::spawn(fs_rx, fs_res_tx);

    let (term_tx, term_rx) = mpsc::channel::<terminal::TerminalRequest>(32);
    let (term_res_tx, mut term_res_rx) = mpsc::channel::<terminal::TerminalResponse>(32);
    let _term_task = terminal::spawn(term_rx, term_res_tx);

    {
        let event_tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(res) = fs_res_rx.recv().await {
                let _ = event_tx.send(CoreEvent::Service(map_fs_response(res))).await;
            }
        });
    }

    {
        let event_tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(res) = term_res_rx.recv().await {
                let _ = event_tx.send(CoreEvent::Service(map_term_response(res))).await;
            }
        });
    }

    let (snap_tx, mut snap_rx) = watch::channel(EditorState::default().snapshot());
    let core_task = tokio::spawn(core_loop(event_rx, snap_tx, fs_tx, term_tx));

    draw_frame(&mut terminal, &snap_rx.borrow())?;
    loop {
        if snap_rx.changed().await.is_err() {
            break;
        }
        draw_frame(&mut terminal, &snap_rx.borrow())?;
    }

    let _ = core_task.await;
    Ok(())
}

async fn core_loop(
    mut event_rx: mpsc::Receiver<CoreEvent>,
    snap_tx: watch::Sender<kjxlkj_core::EditorSnapshot>,
    fs_tx: mpsc::Sender<fs::FsRequest>,
    term_tx: mpsc::Sender<terminal::TerminalRequest>,
) {
    let mut state = EditorState::default();
    let _ = snap_tx.send(state.snapshot());

    while let Some(event) = event_rx.recv().await {
        let effects = state.handle_event(event);
        let _ = snap_tx.send(state.snapshot());
        for eff in effects {
            if handle_effect(eff, &fs_tx, &term_tx).await {
                return;
            }
        }
    }
}

async fn handle_effect(
    eff: Effect,
    fs_tx: &mpsc::Sender<fs::FsRequest>,
    term_tx: &mpsc::Sender<terminal::TerminalRequest>,
) -> bool {
    match eff {
        Effect::Quit { .. } => true,
        Effect::FsRead { request_id, path } => {
            let _ = fs_tx.send(fs::FsRequest::Read { request_id, path }).await;
            false
        }
        Effect::FsWrite {
            request_id,
            path,
            contents,
        } => {
            let _ = fs_tx
                .send(fs::FsRequest::Write {
                    request_id,
                    path,
                    contents,
                })
                .await;
            false
        }
        Effect::TerminalRun { request_id, command } => {
            let _ = term_tx
                .send(terminal::TerminalRequest::RunShell { request_id, command })
                .await;
            false
        }
    }
}

fn spawn_input_thread(event_tx: mpsc::Sender<CoreEvent>) {
    std::thread::spawn(move || loop {
        let ev = match crossterm::event::read() {
            Ok(ev) => ev,
            Err(_) => return,
        };
        let Some(mapped) = map_event(ev) else {
            continue;
        };
        let core_ev = match mapped {
            InputEvent::Key(k) => CoreEvent::Key(k),
            InputEvent::Resize { cols, rows } => CoreEvent::Resize { cols, rows },
        };
        if event_tx.blocking_send(core_ev).is_err() {
            return;
        }
    });
}

fn map_fs_response(res: fs::FsResponse) -> ServiceResult {
    match res {
        fs::FsResponse::ReadOk {
            request_id,
            path,
            contents,
        } => ServiceResult::FsReadOk {
            request_id,
            path,
            contents,
        },
        fs::FsResponse::WriteOk { request_id, path } => ServiceResult::FsWriteOk { request_id, path },
        fs::FsResponse::Error { request_id, message } => ServiceResult::FsError { request_id, message },
    }
}

fn map_term_response(res: terminal::TerminalResponse) -> ServiceResult {
    match res {
        terminal::TerminalResponse::Ok { request_id, output } => ServiceResult::TerminalOk { request_id, output },
        terminal::TerminalResponse::Error { request_id, message } => {
            ServiceResult::TerminalError { request_id, message }
        }
    }
}
