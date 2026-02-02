//! kjxlkj-service-index - File indexing service.
//!
//! This crate provides file and symbol indexing for fuzzy finding.

mod index;
mod fuzzy;

pub use index::{FileIndex, IndexEntry};
pub use fuzzy::{FuzzyMatcher, MatchResult};
