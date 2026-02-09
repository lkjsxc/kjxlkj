/// Host terminal lifecycle and signals.
///
/// Manages raw mode, alternate screen, bracketed paste,
/// focus reporting, and signal handling.
mod terminal;

pub use terminal::{HostTerminal, TerminalGuard};
