use std::path::PathBuf;

use anyhow::Context;
use kjxlkj_core::{CoreEvent, EditorState, Key};
use kjxlkj_services::{fs, terminal};
use serde::Deserialize;
use tokio::sync::{mpsc, watch};

use crate::{core_loop, map_fs_response, map_term_response};

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum HeadlessStep {
    Key { key: Key },
    WaitStatus { contains: String, timeout_ms: u64 },
}

pub async fn run(script: Option<PathBuf>) -> anyhow::Result<()> {
    let Some(script) = script else {
        let state = EditorState::default();
        println!("{}", state.snapshot().buffers[0].lines.join(""));
        return Ok(());
    };

    let contents = tokio::fs::read_to_string(&script)
        .await
        .with_context(|| format!("read script {}", script.display()))?;
    let steps = parse_steps(&contents).context("parse headless script")?;

    let (event_tx, event_rx) = mpsc::channel::<CoreEvent>(1024);

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

    for step in steps {
        match step {
            HeadlessStep::Key { key } => {
                event_tx.send(CoreEvent::Key(key)).await.context("send key")?;
            }
            HeadlessStep::WaitStatus { contains, timeout_ms } => {
                wait_for_status(&mut snap_rx, &contains, timeout_ms).await?;
            }
        }
    }
    drop(event_tx);

    let _ = core_task.await;
    Ok(())
}

fn parse_steps(input: &str) -> anyhow::Result<Vec<HeadlessStep>> {
    match serde_json::from_str::<Vec<HeadlessStep>>(input) {
        Ok(steps) => Ok(steps),
        Err(_) => {
            let keys = serde_json::from_str::<Vec<Key>>(input)?;
            Ok(keys.into_iter().map(|key| HeadlessStep::Key { key }).collect())
        }
    }
}

async fn wait_for_status(
    snap_rx: &mut watch::Receiver<kjxlkj_core::EditorSnapshot>,
    contains: &str,
    timeout_ms: u64,
) -> anyhow::Result<()> {
    if snap_rx.borrow().status.contains(contains) {
        return Ok(());
    }
    let timeout = std::time::Duration::from_millis(timeout_ms);
    tokio::time::timeout(timeout, async {
        loop {
            snap_rx.changed().await?;
            if snap_rx.borrow().status.contains(contains) {
                return Ok::<(), watch::error::RecvError>(());
            }
        }
    })
    .await
    .context("timeout waiting for status")?
    .context("watch closed")?;
    Ok(())
}
