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
//! - command_parse.rs: ex command parser
//! - search.rs: search state and execution

mod command_parse;
mod editor;
mod editor_action;
mod editor_buffer;
mod editor_cmdline;
mod editor_edit;
mod editor_explorer;
mod editor_ext;
mod editor_ops;
mod editor_snapshot;
mod editor_visual;
mod editor_window;
mod register;
mod search;
pub(crate) mod search_util;
mod window_state;

pub use editor::EditorState;
pub use register::RegisterStore;
pub use search::{SearchDirection, SearchState};
pub use window_state::WindowState;

#[cfg(test)]
mod editor_boundary_tests;
#[cfg(test)]
mod editor_buffer_tests;
#[cfg(test)]
mod editor_race_tests;
#[cfg(test)]
mod editor_search_tests;
#[cfg(test)]
mod editor_stage03_tests;
#[cfg(test)]
mod editor_stage03_edit_tests;
#[cfg(test)]
mod editor_textobj_tests;
#[cfg(test)]
mod editor_visual_tests;
#[cfg(test)]
mod editor_stage04_tests;
#[cfg(test)]
mod editor_stage04b_tests;
#[cfg(test)]
mod editor_stage04c_tests;
#[cfg(test)]
mod editor_wincmd_tests;
