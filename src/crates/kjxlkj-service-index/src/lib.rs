//! Index/finder service for file search and navigation.
//!
//! Provides fuzzy file finding, live grep, buffer listing,
//! and symbol search with configurable matching strategies.

mod service;
mod types;

#[cfg(test)]
mod index_tests;

pub use service::IndexService;
pub use types::{
    FinderConfig, FinderItem, FinderQuery, FinderResult, MatchScore, SearchMatch, SearchQuery,
};
