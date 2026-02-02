//! Terminal input decoding for kjxlkj editor.
//!
//! This crate converts key events into intents.

mod cmdline;
mod key;
mod mapping;
mod parser;
mod validation;

pub use cmdline::CommandLine;
pub use key::{Key, KeyCodeWrapper, KeySequence, Modifiers};
pub use mapping::KeyMap;
pub use parser::InputParser;
pub use validation::{
    validate_buffer_name, validate_column, validate_count, validate_file_path,
    validate_line_number, validate_mark, validate_pattern, validate_register, validate_split_ratio,
    validate_tab_width, ValidationResult,
};
