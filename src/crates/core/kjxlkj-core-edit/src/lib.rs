#![forbid(unsafe_code)]

use kjxlkj_core_types::TextRange;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Edit {
    Insert { at: usize, text: String },
    Delete { range: TextRange, deleted: String },
}

impl Edit {
    pub fn inverse(&self) -> Option<Edit> {
        match self {
            Edit::Insert { at, text } => {
                let start = *at;
                let end = start.saturating_add(text.chars().count());
                Some(Edit::Delete { range: TextRange { start, end }, deleted: text.clone() })
            }
            Edit::Delete { range, deleted } => Some(Edit::Insert { at: range.start, text: deleted.clone() }),
        }
    }
}

