//! Host terminal lifecycle and signal management.

mod setup;

pub use setup::{enter_raw_mode, leave_raw_mode, terminal_size};
