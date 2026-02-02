//! kjxlkj-input - Input handling.
//!
//! This crate provides keyboard input parsing and key mapping.

mod key;
mod parser;
mod mapper;
mod command;
mod normal_parser;
mod normal_prefix;
mod normal_chars;
mod normal_types;
mod normal_other;
mod normal_pending;
mod insert_parser;
mod visual_parser;
mod ex_command;
mod ex_helpers;
mod ex_parser;

pub use key::{Key, KeyModifiers, KeyEvent};
pub use parser::KeyParser;
pub use mapper::KeyMapper;
pub use command::{
    Command, FoldCommand, InsertVariant, ScrollCommand, VisualVariant,
};
pub use normal_parser::NormalParser;
pub use insert_parser::{InsertParser, InsertResult};
pub use visual_parser::{VisualParser, VisualResult};
pub use ex_command::{
    BufferCommand, BufferTarget, ExCommand, TabCommand, WindowCommand,
};
pub use ex_parser::ExParser;
