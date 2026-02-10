//! File explorer service.
//!
//! Provides file tree navigation and state management.

mod node;
mod service;
mod state;

pub use node::{ClipboardState, NodeId, TreeNode};
pub use service::{ExplorerError, ExplorerService};
pub use state::ExplorerState;
