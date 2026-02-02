//! Message bus for service communication.

use kjxlkj_core_types::BufferId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Message types for the bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// File system events.
    FsEvent(FsEvent),
    /// LSP events.
    LspEvent(LspEvent),
    /// Git events.
    GitEvent(GitEvent),
    /// Index events.
    IndexEvent(IndexEvent),
    /// Terminal events.
    TerminalEvent(TerminalEvent),
}

/// File system events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FsEvent {
    /// File changed.
    FileChanged(PathBuf),
    /// File created.
    FileCreated(PathBuf),
    /// File deleted.
    FileDeleted(PathBuf),
    /// Read complete.
    ReadComplete { path: PathBuf, content: String },
    /// Write complete.
    WriteComplete { path: PathBuf },
}

/// LSP events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LspEvent {
    /// Server started.
    Started { language: String },
    /// Diagnostics received.
    Diagnostics { buffer_id: BufferId },
    /// Completion received.
    Completion { buffer_id: BufferId },
}

/// Git events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitEvent {
    /// Status changed.
    StatusChanged,
    /// Hunks updated.
    HunksUpdated { buffer_id: BufferId },
}

/// Index events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexEvent {
    /// Indexing complete.
    IndexComplete,
    /// Search results.
    SearchResults { query: String },
}

/// Terminal events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalEvent {
    /// Output received.
    Output { id: u64, data: String },
    /// Process exited.
    Exited { id: u64, code: i32 },
}

/// Message bus for service communication.
#[derive(Clone)]
pub struct MessageBus {
    /// Sender for messages.
    tx: mpsc::Sender<Message>,
}

impl MessageBus {
    /// Creates a new message bus.
    pub fn new() -> (Self, mpsc::Receiver<Message>) {
        let (tx, rx) = mpsc::channel(256);
        (Self { tx }, rx)
    }

    /// Sends a message.
    pub async fn send(&self, msg: Message) -> Result<(), mpsc::error::SendError<Message>> {
        self.tx.send(msg).await
    }

    /// Clones the sender.
    pub fn sender(&self) -> mpsc::Sender<Message> {
        self.tx.clone()
    }
}
