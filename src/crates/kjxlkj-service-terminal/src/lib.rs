//! Terminal/PTY service: escape parsing, PTY spawn, screen buffer.

mod escape_parser;
mod pty;
mod screen;
mod service;

pub use screen::ScreenBuffer;
pub use service::TerminalService;
