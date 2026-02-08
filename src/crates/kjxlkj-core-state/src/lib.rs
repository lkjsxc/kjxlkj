//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod action_dispatch2;
pub(crate) mod after_dir;
pub(crate) mod audio;
pub(crate) mod auto_session;
pub(crate) mod autocmd;
mod buffer;
pub(crate) mod buffer_options;
#[cfg(test)]
mod buffer_tests;
pub(crate) mod cjk_support;
mod command_dispatch;
mod command_dispatch_ext;
mod command_dispatch_tabs;
mod command_parse;
pub(crate) mod completion;
pub(crate) mod dap;
pub(crate) mod digraphs;
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
pub(crate) mod flash_jump;
pub(crate) mod floating;
pub(crate) mod folds_advanced;
pub(crate) mod git_features;
mod editor_global_sort;
pub(crate) mod ime;
mod editor_insert;
mod editor_jumplist;
mod editor_key_dispatch;
pub(crate) mod keybinding_dsl;
pub(crate) mod line_wrap;
pub(crate) mod live_grep;
pub(crate) mod lsp_features;
mod editor_macros;
mod editor_op_resolve;
mod editor_marks;
pub(crate) mod mappings;
pub(crate) mod mouse;
pub(crate) mod multicursor;
pub(crate) mod notifications;
mod editor_operators;
pub(crate) mod persistence;
pub(crate) mod popup;
mod editor_quickfix;
mod editor_range_cmds;
mod editor_range_dispatch;
pub(crate) mod regex_engine;
pub(crate) mod remote;
mod editor_scroll;
mod editor_search_ops;
pub(crate) mod session_features;
mod editor_shell;
mod editor_snapshot;
pub(crate) mod snippets;
pub(crate) mod spell;
pub(crate) mod statusline_dsl;
mod editor_substitute;
pub(crate) mod editor_tabs;
pub(crate) mod text_object_function;
pub(crate) mod theming;
pub(crate) mod tmux;
pub(crate) mod treesitter_objects;
pub(crate) mod unicode_input;
pub(crate) mod user_functions;
mod editor_window_adv;
mod editor_window_ops;
mod editor_window_resize;
pub(crate) mod view_management;
pub(crate) mod window_layouts;
pub(crate) mod wm_integration;
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
