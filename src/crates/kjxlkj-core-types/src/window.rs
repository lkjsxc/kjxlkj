//! Window identity type.

use serde::{Deserialize, Serialize};

/// Stable unique window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);
