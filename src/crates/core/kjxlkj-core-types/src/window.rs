//! Window content type descriptors.

use crate::{BufferId, ExplorerStateId, TerminalId};
use serde::{Deserialize, Serialize};

/// Content source for a window leaf.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentKind {
    Buffer(BufferId),
    Explorer(ExplorerStateId),
    Terminal(TerminalId),
}

/// Window type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowType {
    Buffer,
    Explorer,
    Terminal,
}

impl ContentKind {
    pub fn window_type(&self) -> WindowType {
        match self {
            Self::Buffer(_) => WindowType::Buffer,
            Self::Explorer(_) => WindowType::Explorer,
            Self::Terminal(_) => WindowType::Terminal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_kind_type_mapping() {
        let c = ContentKind::Buffer(BufferId(0));
        assert_eq!(c.window_type(), WindowType::Buffer);
    }
}
