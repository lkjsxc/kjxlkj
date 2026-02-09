/// Rope-based text model with grapheme-aware indexing.
///
/// Provides O(log n) insert/delete, line-index lookup,
/// and efficient snapshot cloning via structural sharing.
mod grapheme;
mod rope_ext;

pub use grapheme::{grapheme_count, grapheme_to_byte_offset, nth_grapheme};
pub use rope_ext::RopeExt;

// Re-export ropey for convenience.
pub use ropey::Rope;
