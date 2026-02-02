//! kjxlkj-service-index - File indexing service.
//!
//! This crate provides file and symbol indexing for fuzzy finding.

mod fuzzy;
mod index;

pub use fuzzy::{FuzzyMatcher, MatchResult};
pub use index::{FileIndex, IndexEntry};
