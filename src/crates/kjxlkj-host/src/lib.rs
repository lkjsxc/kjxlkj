//! Terminal host integration for kjxlkj.
//!
//! Manages the terminal lifecycle and main event loop.

mod terminal;
mod headless;

pub use terminal::*;
pub use headless::*;
