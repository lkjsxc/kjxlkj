//! Text model wrapping ropey with grapheme-aware operations.

mod buffer; mod grapheme; mod sentence;
mod snapshot; mod word; mod word_extra;
mod large_buffer;
mod text_objects_full;
mod buffer_full;

pub use buffer::TextBuffer;
pub use grapheme::{
    grapheme_width, next_grapheme_boundary, prev_grapheme_boundary,
    display_width_to_col, line_display_width,
};
pub use snapshot::BufferSnapshot;
pub use word::{
    is_word_char, word_end_forward, word_end_backward,
    word_start_backward, word_start_forward, CharClass,
};
pub use word_extra::{
    big_word_start_forward, big_word_start_backward,
    big_word_end_forward, big_word_end_backward,
};
pub use sentence::{next_sentence, prev_sentence};
