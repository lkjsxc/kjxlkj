//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod action_dispatch2;
pub(crate) mod autocmd;
mod buffer;
#[cfg(test)]
mod buffer_tests;
mod command_dispatch;
mod command_dispatch_ext;
mod command_dispatch_tabs;
mod command_parse;
pub(crate) mod completion;
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
mod editor_config;
mod editor_file_ops;
pub(crate) mod file_explorer;
pub(crate) mod filetype;
pub(crate) mod floating;
mod editor_global_sort;
mod editor_insert;
mod editor_jumplist;
mod editor_key_dispatch;
mod editor_macros;
mod editor_op_resolve;
mod editor_marks;
pub(crate) mod mappings;
mod editor_operators;
mod editor_quickfix;
mod editor_range_cmds;
mod editor_range_dispatch;
mod editor_scroll;
mod editor_search_ops;
mod editor_shell;
mod editor_snapshot;
mod editor_substitute;
pub(crate) mod editor_tabs;
mod editor_window_adv;
mod editor_window_ops;
mod editor_window_resize;
mod search;
mod session;
pub(crate) mod tags;
pub(crate) mod user_commands;
mod viewport;
mod window;

pub use buffer::{BufferState, LineEnding};
pub use command_dispatch::dispatch_command;
pub use editor::{EditorState, MarkEntry, QuickfixEntry};
pub use session::{load_session, save_session, SessionData, SessionLayout};
pub use viewport::ViewportState;
pub use window::{WindowContent, WindowOptions, WindowState};

/// Convert char to RegisterName.
pub(crate) fn register_name_from_char(
    c: char,
) -> Option<kjxlkj_core_types::RegisterName> {
    kjxlkj_core_types::RegisterName::from_char(c)
}
