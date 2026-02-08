//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod action_dispatch2;
mod buffer;
#[cfg(test)]
mod buffer_tests;
mod command_dispatch;
mod editor;
mod editor_actions;
mod editor_actions2;
mod editor_actions3;
mod editor_auto_marks;
mod editor_auto_pairs;
mod editor_changelist;
mod editor_char_ops;
mod editor_cmdline;
mod editor_comments;
mod editor_file_ops;
mod editor_global_sort;
mod editor_insert;
mod editor_jumplist;
mod editor_key_dispatch;
mod editor_macros;
mod editor_op_resolve;
mod editor_marks;
mod editor_operators;
mod editor_range_cmds;
mod editor_range_dispatch;
mod editor_scroll;
mod editor_search_ops;
mod editor_snapshot;
mod editor_substitute;
mod editor_window_ops;
mod editor_window_resize;
mod search;
mod session;
mod viewport;
mod window;

pub use buffer::{BufferState, LineEnding};
pub use command_dispatch::dispatch_command;
pub use editor::{EditorState, MarkEntry};
pub use session::{load_session, save_session, SessionData, SessionLayout};
pub use viewport::ViewportState;
pub use window::{WindowContent, WindowOptions, WindowState};

/// Convert char to RegisterName.
pub(crate) fn register_name_from_char(
    c: char,
) -> Option<kjxlkj_core_types::RegisterName> {
    kjxlkj_core_types::RegisterName::from_char(c)
}
