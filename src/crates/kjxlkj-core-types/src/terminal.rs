//! Terminal identity type.

use serde::{Deserialize, Serialize};

/// Stable unique terminal identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalId(pub u64);
