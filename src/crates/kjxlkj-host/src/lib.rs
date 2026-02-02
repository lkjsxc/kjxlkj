//! kjxlkj-host - Host environment abstraction.
//!
//! This crate provides filesystem, clipboard, and configuration.

#![allow(dead_code)]

mod clipboard;
mod config;
mod fs;
mod keymap;
mod options;
mod theme;

pub use clipboard::Clipboard;
pub use config::{Config, EditorConfig, FilesConfig, UiConfig};
pub use fs::FileSystem;
pub use keymap::Keymap;
pub use options::{OptionScope, OptionValue, Options};
pub use theme::{Color, Highlights, Palette, Style, Theme};
