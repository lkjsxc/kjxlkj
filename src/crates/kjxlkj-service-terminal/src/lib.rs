//! Terminal service: PTY management and VT parsing.

pub mod cell;
pub mod csi;
pub mod parser;
mod parser_esc;
mod parser_ground;
pub mod screen;
mod screen_ops;
pub mod sgr;
mod task;
#[cfg(test)]
mod terminal_tests;

pub use screen::Screen;
pub use task::TerminalService;
