//! Editor state: single mutable owner of all core state.
//!
//! See /docs/spec/architecture/runtime.md: EditorState has a single
//! mutable owner — the core task.
//!
//! Module split per /docs/spec/architecture/source-layout.md:
//! - editor.rs: struct + new + handle_key
//! - editor_action.rs: apply_action dispatch
//! - editor_edit.rs: text editing operations
//! - editor_snapshot.rs: snapshot construction
//! - editor_window.rs: window management

mod editor;
mod editor_action;
mod editor_edit;
mod editor_snapshot;
mod editor_window;
mod window_state;

pub use editor::EditorState;
pub use window_state::WindowState;
