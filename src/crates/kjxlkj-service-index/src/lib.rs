//! kjxlkj-service-index: Fuzzy finding and symbol indexing.

pub mod fuzzy;
pub mod symbol_index;

pub use fuzzy::{fuzzy_match, normalize_score, rank_candidates, FuzzyMatch};
pub use symbol_index::{SymbolEntry, SymbolIndex, SymbolKind};
