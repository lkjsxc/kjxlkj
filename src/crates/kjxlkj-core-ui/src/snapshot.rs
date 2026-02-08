//! Editor snapshot — the top-level render input.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::{BufferId, Mode, TerminalId};

use crate::{
    BufferSnapshot, CmdlineState, Notification, TerminalSnapshot,
    Theme, WindowLayout,
};

/// Search state for rendering highlights.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchState {
    /// Active search pattern (empty if none).
    pub pattern: String,
    /// Whether search is active.
    pub active: bool,
}

/// Complete editor snapshot for the render task.
///
/// Per /docs/spec/architecture/render-pipeline.md, this contains all
/// data needed to render without querying core or services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSnapshot {
    /// Monotonic sequence number for stale detection.
    pub sequence: u64,
    /// Window layout with rectangles.
    pub layout: WindowLayout,
    /// Buffer snapshots keyed by ID.
    pub buffers: HashMap<BufferId, BufferSnapshot>,
    /// Terminal snapshots keyed by ID.
    pub terminals: HashMap<TerminalId, TerminalSnapshot>,
    /// Current editing mode.
    pub mode: Mode,
    /// Command-line state.
    pub cmdline: CmdlineState,
    /// Active notifications.
    pub notifications: Vec<Notification>,
    /// Search state.
    pub search: SearchState,
    /// Active theme.
    pub theme: Theme,
    /// Terminal dimensions (cols, rows).
    pub terminal_size: (u16, u16),
}

impl EditorSnapshot {
    /// Create a minimal initial snapshot.
    pub fn initial(
        cols: u16,
        rows: u16,
        buf_id: BufferId,
        win_id: kjxlkj_core_types::WindowId,
    ) -> Self {
        use crate::layout::Rect;

        let mut buffers = HashMap::new();
        buffers.insert(buf_id, BufferSnapshot::empty(buf_id));

        Self {
            sequence: 0,
            layout: WindowLayout::single(
                win_id,
                Rect::new(0, 0, cols, rows.saturating_sub(1)),
            ),
            buffers,
            terminals: HashMap::new(),
            mode: Mode::Normal,
            cmdline: CmdlineState::inactive(),
            notifications: Vec::new(),
            search: SearchState::default(),
            theme: Theme::default(),
            terminal_size: (cols, rows),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::WindowId;

    #[test]
    fn initial_snapshot() {
        let snap = EditorSnapshot::initial(
            80,
            24,
            BufferId(1),
            WindowId(1),
        );
        assert_eq!(snap.sequence, 0);
        assert_eq!(snap.terminal_size, (80, 24));
        assert_eq!(snap.mode, Mode::Normal);
        assert!(snap.buffers.contains_key(&BufferId(1)));
    }
}
