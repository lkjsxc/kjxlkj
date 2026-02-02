//! Indexing service for kjxlkj editor.
//!
//! This crate provides file indexing and search.

mod finder;
pub mod index;
mod service;

#[cfg(test)]
mod tests;

pub use finder::Finder;
pub use index::Index;
pub use service::IndexService;
