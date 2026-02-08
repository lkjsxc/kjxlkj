//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod action_dispatch2;
pub mod after_dir;
pub mod audio;
pub mod auto_session;
pub mod autocmd;
mod buffer;
pub mod buffer_options;
#[cfg(test)]
mod buffer_tests;
pub mod cjk_support;
mod command_dispatch;
mod command_dispatch_ext;
mod command_dispatch_tabs;
mod command_parse;
pub mod completion;
pub mod dap;
pub mod digraphs;
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
pub mod file_explorer;
pub mod filetype;
pub mod flash_jump;
pub mod floating;
pub mod folds_advanced;
pub mod git_features;
mod editor_global_sort;
pub mod ime;
mod editor_insert;
mod editor_jumplist;
mod editor_key_dispatch;
pub mod keybinding_dsl;
pub mod line_wrap;
pub mod live_grep;
pub mod lsp_features;
mod editor_macros;
mod editor_op_resolve;
mod editor_marks;
pub mod mappings;
pub mod mouse;
pub mod multicursor;
pub mod notifications;
mod editor_operators;
pub mod persistence;
pub mod popup;
mod editor_quickfix;
mod editor_range_cmds;
mod editor_range_dispatch;
pub mod regex_engine;
pub mod remote;
mod editor_scroll;
mod editor_search_ops;
pub mod session_features;
mod editor_shell;
mod editor_snapshot;
pub mod snippets;
pub mod spell;
pub mod statusline_dsl;
mod editor_substitute;
pub mod editor_tabs;
pub mod text_object_function;
pub mod theming;
pub mod tmux;
pub mod treesitter_objects;
pub mod unicode_input;
pub mod user_functions;
mod editor_window_adv;
mod editor_window_ops;
mod editor_window_resize;
pub mod view_management;
pub mod window_layouts;
pub mod wm_integration;
pub mod search;
mod session;
pub mod tags;
pub mod user_commands;
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
