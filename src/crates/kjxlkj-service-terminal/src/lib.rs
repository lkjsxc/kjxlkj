//! kjxlkj-service-terminal - Embedded terminal service.
//!
//! This crate provides embedded terminal emulation.

mod pty;
mod term;

pub use pty::PtyProcess;
pub use term::Terminal;
