use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPos {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Result<Self, RangeError> {
        if start > end {
            return Err(RangeError::StartAfterEnd { start, end });
        }
        Ok(Self { start, end })
    }
}

#[derive(Debug, Error)]
pub enum RangeError {
    #[error("invalid range: start({start}) > end({end})")]
    StartAfterEnd { start: usize, end: usize },
}

