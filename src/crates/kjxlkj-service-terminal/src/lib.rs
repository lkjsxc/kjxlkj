//! Terminal/PTY service for kjxlkj editor.
//!
//! This crate provides integrated terminal support.

mod pty;
mod service;

#[cfg(test)]
mod tests;

pub use pty::Pty;
pub use service::TerminalService;
