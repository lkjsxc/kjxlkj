//! Editor state: single mutable owner of all core state.
//!
//! See /docs/spec/architecture/runtime.md: EditorState has a single
//! mutable owner — the core task.

mod editor;
mod window_state;

pub use editor::EditorState;
pub use window_state::WindowState;
