//! Headless test harness for integration tests.
//!
//! See /docs/spec/technical/testing-pty-harness.md.
//! This provides the headless (T1) state harness.
//! PTY (T2) harness is deferred to later waves.

mod headless;

pub use headless::HeadlessHarness;
