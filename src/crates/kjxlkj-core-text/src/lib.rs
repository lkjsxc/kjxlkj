//! kjxlkj-core-text: rope-based text storage and manipulation.

pub mod buffer;
pub mod buffer_features;
pub mod buffer_full;
pub mod grapheme;
pub mod large_buffer;
pub mod manipulation;
pub mod segment;
pub mod streaming;

pub use buffer::TextBuffer;
pub use buffer_features::{AutoCmdRegistry, BufEvent, BufferLocalOptions, BufferVariables, FileFormat};
pub use buffer_full::{
    filter_listed, find_by_name, modified_count, AlternateTracker, BufferFlags, BufferInfo,
    BufferType,
};
pub use grapheme::{
    char_to_col, col_to_char, display_width, first_non_blank, is_word_char, last_non_blank,
    prev_word_end, word_end_forward, word_start_backward, word_start_forward,
};
pub use large_buffer::{
    build_line_index, choose_strategy, compute_chunks, extract_line_range, LoadStrategy,
};
pub use manipulation::{
    convert_case, indent_level, join_lines, reindent, reverse_chars, sort_lines, trim_trailing,
    CaseKind,
};
pub use segment::{is_wide_char, safe_slice, segment_line, MAX_RENDER_COLS};
pub use streaming::{
    estimate_line_count, validate_file_size, ReadChunk, StreamConfig, StreamReader, StreamState,
};
