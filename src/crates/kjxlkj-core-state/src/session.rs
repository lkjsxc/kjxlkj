//! Session persistence: save and load editor state as JSON.
//!
//! Sessions serialize the layout tree, buffer references, cursor
//! positions, and viewport state. Terminal windows are persisted
//! as layout nodes but their process state is not saved.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level session data (matches spec schema version 1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub version: u32,
    pub cwd: String,
    pub timestamp: String,
    pub tabs: Vec<SessionTab>,
    pub active_tab: usize,
    pub buffers: Vec<BufferRef>,
}

/// A serialized tab page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTab {
    pub layout: SessionLayoutNode,
    pub focused_window: usize,
}

/// Recursive layout node for session serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLayoutNode {
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SessionLayoutNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowRef>,
}

/// A serialized window reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowRef {
    pub content_type: String,
    pub buffer_path: Option<String>,
    pub cursor_line: usize,
    pub cursor_grapheme: usize,
    pub top_line: usize,
    pub left_col: usize,
    pub wrap: bool,
}

/// A serialized buffer reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferRef {
    pub path: String,
    pub encoding: String,
    pub modified: bool,
}

impl SessionData {
    /// Create a new session with the given working directory.
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            version: 1,
            cwd: cwd.to_string_lossy().to_string(),
            timestamp: String::new(),
            tabs: Vec::new(),
            active_tab: 0,
            buffers: Vec::new(),
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl SessionLayoutNode {
    /// Create a leaf node.
    pub fn leaf(window: WindowRef) -> Self {
        Self {
            node_type: "leaf".to_string(),
            children: None,
            weights: None,
            window: Some(window),
        }
    }

    /// Create a horizontal split node.
    pub fn hsplit(children: Vec<Self>, weights: Vec<f32>) -> Self {
        Self {
            node_type: "hsplit".to_string(),
            children: Some(children),
            weights: Some(weights),
            window: None,
        }
    }

    /// Create a vertical split node.
    pub fn vsplit(children: Vec<Self>, weights: Vec<f32>) -> Self {
        Self {
            node_type: "vsplit".to_string(),
            children: Some(children),
            weights: Some(weights),
            window: None,
        }
    }
}

impl WindowRef {
    /// Create a buffer window reference.
    pub fn buffer(
        path: Option<String>,
        cur: (usize, usize),
        vp: (usize, usize),
        wrap: bool,
    ) -> Self {
        Self {
            content_type: "buffer".to_string(),
            buffer_path: path,
            cursor_line: cur.0,
            cursor_grapheme: cur.1,
            top_line: vp.0,
            left_col: vp.1,
            wrap,
        }
    }

    /// Create a terminal window reference.
    pub fn terminal() -> Self {
        Self {
            content_type: "terminal".to_string(),
            buffer_path: None,
            cursor_line: 0,
            cursor_grapheme: 0,
            top_line: 0,
            left_col: 0,
            wrap: false,
        }
    }
}
