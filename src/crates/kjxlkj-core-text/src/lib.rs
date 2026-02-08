//! Text model for the editor: rope wrapper, grapheme decomposition,
//! display width computation, and line operations.
//!
//! This crate wraps `ropey::Rope` and provides grapheme-cluster-aware
//! indexing, bidirectional column mapping, and line-level operations
//! as specified in /docs/spec/editor/buffers.md.

mod buffer_content;
mod display_width;
mod grapheme;
mod line_ops;
mod word;

pub use buffer_content::BufferContent;
pub use display_width::{grapheme_display_width, str_display_width};
pub use grapheme::{GraphemeIter, LineGraphemes};
pub use line_ops::{LineEnding, detect_line_ending};
pub use word::{WordKind, classify_word_char, find_word_boundary};
