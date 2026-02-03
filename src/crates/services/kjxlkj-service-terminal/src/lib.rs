#![forbid(unsafe_code)]

use std::collections::HashSet;

use anyhow::Context;
use tokio::process::Command;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum TerminalRequest {
    RunShell { request_id: u64, command: String },
    Cancel { request_id: u64 },
}

#[derive(Clone, Debug)]
pub enum TerminalResponse {
    Ok { request_id: u64, output: String },
    Error { request_id: u64, message: String },
}

pub fn spawn(
    mut rx: mpsc::Receiver<TerminalRequest>,
    tx: mpsc::Sender<TerminalResponse>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut cancelled = HashSet::<u64>::new();
        while let Some(req) = rx.recv().await {
            match req {
                TerminalRequest::Cancel { request_id } => {
                    cancelled.insert(request_id);
                }
                TerminalRequest::RunShell { request_id, command } => {
                    if cancelled.contains(&request_id) {
                        continue;
                    }
                    let out = Command::new("sh")
                        .arg("-lc")
                        .arg(command)
                        .output()
                        .await
                        .context("spawn shell");
                    let _ = match out {
                        Ok(output) => tx
                            .send(TerminalResponse::Ok {
                                request_id,
                                output: String::from_utf8_lossy(&output.stdout).to_string(),
                            })
                            .await,
                        Err(err) => {
                            tx.send(TerminalResponse::Error { request_id, message: err.to_string() }).await
                        }
                    };
                }
            }
        }
    })
}
