//! Text model for kjxlkj editor.
//!
//! This crate provides the rope-based text buffer implementation.

mod char_class;
mod grapheme;
mod manipulation;
mod paragraph;
mod rope_ext;
mod syntax_region;
mod text_buffer;
mod width;
mod word;

#[cfg(test)]
mod tests;

pub use grapheme::{GraphemeIter, grapheme_count, grapheme_width};
pub use manipulation::{
    byte_count, char_count, collapse_blank_lines, duplicate_line, ensure_final_newline,
    join_lines, line_count, normalize_line_endings, remove_blank_lines, reverse_line,
    reverse_lines, sort_lines, sort_lines_reverse, split_lines, strip_trailing_whitespace,
    uniq_lines, word_count,
};
pub use paragraph::{is_blank_line, next_paragraph, paragraph_count, paragraph_range, prev_paragraph};
pub use rope_ext::RopeExt;
pub use syntax_region::{MatchedRegion, SyntaxDefinition, SyntaxPattern, SyntaxRegion};
pub use text_buffer::TextBuffer;
pub use width::{byte_for_column, char_width, str_width, truncate_to_width, width_to_byte, CharWidth};
pub use word::{
    bigword_end, next_bigword_start, next_word_start, prev_bigword_start, prev_word_start,
    word_end, CharClass,
};
