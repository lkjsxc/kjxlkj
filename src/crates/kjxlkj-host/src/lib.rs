//! kjxlkj-host - Host environment abstraction.
//!
//! This crate provides filesystem, clipboard, and configuration.

#![allow(dead_code)]

mod fs;
mod clipboard;
mod config;
mod keymap;
mod theme;
mod options;

pub use fs::FileSystem;
pub use clipboard::Clipboard;
pub use config::{Config, EditorConfig, UiConfig, FilesConfig};
pub use keymap::Keymap;
pub use theme::{Theme, Palette, Color, Highlights, Style};
pub use options::{Options, OptionValue, OptionScope};
