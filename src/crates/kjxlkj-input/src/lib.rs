//! kjxlkj-input - Input handling.
//!
//! This crate provides keyboard input parsing and key mapping.

mod command;
mod ex_command;
mod ex_helpers;
mod ex_parser;
mod insert_parser;
mod key;
mod mapper;
mod normal_chars;
mod normal_other;
mod normal_parser;
mod normal_pending;
mod normal_prefix;
mod normal_types;
mod parser;
mod visual_parser;

pub use command::{Command, FoldCommand, InsertVariant, ScrollCommand, VisualVariant};
pub use ex_command::{BufferCommand, BufferTarget, ExCommand, TabCommand, WindowCommand};
pub use ex_parser::ExParser;
pub use insert_parser::{InsertParser, InsertResult};
pub use key::{Key, KeyEvent, KeyModifiers};
pub use mapper::KeyMapper;
pub use normal_parser::NormalParser;
pub use parser::KeyParser;
pub use visual_parser::{VisualParser, VisualResult};
