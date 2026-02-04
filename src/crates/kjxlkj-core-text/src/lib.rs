//! Text model and rope-based buffer storage.

mod buffer;
mod grapheme;
mod line;
mod word;

pub use buffer::TextBuffer;
pub use grapheme::{grapheme_width, next_grapheme_boundary, prev_grapheme_boundary};
pub use line::{line_end_col, line_len};
pub use word::{find_word_end, find_word_start, next_word_boundary, prev_word_boundary};
