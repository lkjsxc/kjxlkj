//! Index/navigation service: file scanning, fuzzy matching.

mod fuzzy;
mod scanner;
mod service;

pub use fuzzy::fuzzy_match;
pub use service::IndexService;
