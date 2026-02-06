//! Text model wrapping ropey with grapheme-aware operations.

mod buffer;
mod grapheme;
mod snapshot;
mod word;

pub use buffer::TextBuffer;
pub use grapheme::{grapheme_width, next_grapheme_boundary, prev_grapheme_boundary};
pub use snapshot::BufferSnapshot;
pub use word::{
    is_word_char, word_end_forward, word_start_backward, word_start_forward,
};
