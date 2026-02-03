#![forbid(unsafe_code)]

use std::collections::HashSet;

use anyhow::Context;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub enum FsRequest {
    Read { request_id: u64, path: String },
    Write {
        request_id: u64,
        path: String,
        contents: String,
    },
    Cancel { request_id: u64 },
}

#[derive(Clone, Debug)]
pub enum FsResponse {
    ReadOk {
        request_id: u64,
        path: String,
        contents: String,
    },
    WriteOk { request_id: u64, path: String },
    Error { request_id: u64, message: String },
}

pub fn spawn(mut rx: mpsc::Receiver<FsRequest>, tx: mpsc::Sender<FsResponse>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut cancelled = HashSet::<u64>::new();
        while let Some(req) = rx.recv().await {
            match req {
                FsRequest::Cancel { request_id } => {
                    cancelled.insert(request_id);
                }
                FsRequest::Read { request_id, path } => {
                    if cancelled.contains(&request_id) {
                        continue;
                    }
                    let res = tokio::fs::read_to_string(&path).await.context("read file");
                    let _ = match res {
                        Ok(contents) => {
                            tx.send(FsResponse::ReadOk { request_id, path, contents }).await
                        }
                        Err(err) => {
                            tx.send(FsResponse::Error { request_id, message: err.to_string() }).await
                        }
                    };
                }
                FsRequest::Write {
                    request_id,
                    path,
                    contents,
                } => {
                    if cancelled.contains(&request_id) {
                        continue;
                    }
                    let res = tokio::fs::write(&path, contents).await.context("write file");
                    let _ = match res {
                        Ok(()) => tx.send(FsResponse::WriteOk { request_id, path }).await,
                        Err(err) => {
                            tx.send(FsResponse::Error { request_id, message: err.to_string() }).await
                        }
                    };
                }
            }
        }
    })
}
