//! Terminal host integration: lifecycle, raw mode, signals.

mod capabilities;
mod panic_handler;
mod signals;
mod terminal;

pub use capabilities::{ColorSupport, TerminalCapabilities};
pub use panic_handler::install_panic_handler;
pub use signals::SignalHandler;
#[cfg(unix)]
pub use signals::watch_sigwinch;
pub use terminal::TerminalHost;
