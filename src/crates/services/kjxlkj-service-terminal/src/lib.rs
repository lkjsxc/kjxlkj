//! Terminal service.
//!
//! This crate provides PTY-backed terminal emulation.

mod parser;
mod pty;
mod screen;
mod service;

pub use parser::*;
pub use pty::*;
pub use screen::*;
pub use service::*;
