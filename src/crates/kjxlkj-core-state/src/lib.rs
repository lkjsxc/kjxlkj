/// Central editor state: single mutable owner in the core task.
///
/// `EditorState` owns buffers, windows, mode, registers,
/// and produces `EditorSnapshot` for rendering.
pub mod buffer;
mod buffer_list;
#[cfg(test)]
mod case_ops_tests;
mod cmdline;
mod cmdline_completion;
mod cmdline_completion_ctx;
mod cmdline_dispatch;
#[cfg(test)]
mod completion_range_tests;
mod config_loader;
pub mod contracts;
mod cursor_ops;
mod cursor_ops_findchar;
mod cursor_ops_lists;
mod cursor_ops_scroll;
pub mod editing_helpers;
pub mod editing_helpers_surround;
#[cfg(test)]
mod editing_helpers_tests;
mod editing_ops_insert;
mod editing_ops_modify;
mod editing_ops_ranges;
mod editing_ops_yank;
pub mod editor;
mod editor_actions;
#[cfg(test)]
mod editor_actions_tests;
mod editor_mode_dispatch;
mod editor_modes;
mod editor_search_marks;
#[cfg(test)]
mod editor_tests_basic;
#[cfg(test)]
mod editor_tests_ext;
pub mod events;
#[cfg(test)]
mod events_tests;
pub mod events_types;
mod ex_buffer_cmds;
mod ex_dispatch;
mod ex_jump_noh;
pub(crate) mod ex_map;
mod ex_parse;
mod ex_parse_ranges;
pub mod ex_parse_substitute;
#[cfg(test)]
mod ex_parse_tests;
mod ex_scripting;
mod ex_session_cmds;
mod ex_sort;
mod ex_substitute;
mod ex_substitute_confirm;
pub(crate) mod expr_eval;
#[cfg(test)]
mod findchar_tests;
mod format_ops;
mod incsearch;
mod insert_register;
pub(crate) mod key_notation;
mod macros;
#[cfg(test)]
mod macros_tests;
mod mappings;
#[cfg(test)]
mod mappings_tests;
pub mod marks;
#[cfg(test)]
mod marks_tests;
mod notify;
mod op_pending;
mod op_pending_helpers;
#[cfg(test)]
mod op_pending_tests;
pub mod options;
#[cfg(test)]
mod options_tests;
#[cfg(test)]
mod regex_register_tests;
pub(crate) mod regex_translate;
pub mod registers;
#[cfg(test)]
mod registers_tests;
pub mod search_engine;
#[cfg(test)]
mod search_engine_tests;
pub mod search_types;
pub mod session;
#[cfg(test)]
mod session_tests;
pub mod snippets;
pub(crate) mod text_objects;
pub(crate) mod text_objects_argument;
pub(crate) mod text_objects_class;
pub(crate) mod text_objects_delim;
pub(crate) mod text_objects_sentence;
pub(crate) mod text_objects_tag;
pub mod user_commands;
pub mod user_commands_parse;
pub mod user_functions;
#[cfg(test)]
mod user_commands_tests;
mod visual_block_insert;
mod visual_ops;
#[cfg(test)]
mod visual_ops_tests;
mod visual_paste;
mod visual_replace;
#[cfg(test)]
mod wave10_tests;
mod wave11_tests;
#[cfg(test)]
mod wave12_tests;
#[cfg(test)]
mod wave13_tests;
#[cfg(test)]
mod wave14_tests;
#[cfg(test)]
mod wave15_tests;
#[cfg(test)]
mod wave16_tests;
#[cfg(test)]
mod wave17_tests;
#[cfg(test)]
mod wave18_tests;
#[cfg(test)]
mod wave19_tests;
#[cfg(test)]
mod wave20_tests;
#[cfg(test)]
mod wave7_tests;
#[cfg(test)]
mod wave8_tests;
#[cfg(test)]
mod wave9_tests;
mod window_tree;
mod window_tree_layout;

pub use buffer_list::BufferList;
pub use cmdline::CmdlineHandler;
pub use contracts::{ContractChecker, ContractKind};
pub use editing_helpers::{AutoPair, CommentConfig};
pub use editor::EditorState;
pub use events::EventRegistry;
pub use events_types::{EventData, EventKind};
pub use mappings::{KeyMapping, MapMode, MappingLookup, MappingTable};
pub use marks::{MarkFile, MarkPosition};
pub use registers::{RegisterContent, RegisterFile};
pub use search_types::{CaseMode, SearchDirection, SearchMatch, SearchOffset, SearchState};
pub use session::{SessionData, SessionManager};
pub use user_commands::{UserCommand, UserCommandRegistry};
pub use user_functions::{FunctionRegistry, UserFunction};
pub use window_tree::WindowTree;
