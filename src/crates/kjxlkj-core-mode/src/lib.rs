//! Mode key dispatch and state machines.

mod command;
mod dispatch;
mod insert;
mod key_motion;
mod mode_tests;
mod normal;
mod normal_tests;
mod visual;

pub use dispatch::{DispatchResult, ModeDispatcher};
