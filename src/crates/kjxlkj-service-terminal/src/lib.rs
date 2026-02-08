//! Terminal/PTY service: escape parsing, PTY spawn, screen buffer.

mod escape_parser;
#[cfg(test)]
mod escape_parser_tests;
mod pty;
mod screen;
mod screen_ops;
mod screen_scroll;
mod service;

pub use screen::ScreenBuffer;
pub use service::TerminalService;
