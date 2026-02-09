/// Central editor state: single mutable owner in the core task.
///
/// `EditorState` owns buffers, windows, mode, registers,
/// and produces `EditorSnapshot` for rendering.
pub mod buffer;
mod buffer_list;
mod cmdline;
pub mod contracts;
mod cursor_ops;
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
mod editor_modes;
mod editor_search_marks;
#[cfg(test)]
mod editor_tests_basic;
#[cfg(test)]
mod editor_tests_ext;
#[cfg(test)]
mod search_engine_tests;
pub mod events;
pub mod events_types;
#[cfg(test)]
mod events_tests;
mod ex_buffer_cmds;
mod ex_dispatch;
pub(crate) mod ex_map;
mod ex_parse;
pub mod ex_parse_substitute;
#[cfg(test)]
mod ex_parse_tests;
mod ex_scripting;
mod ex_substitute;
pub(crate) mod key_notation;
mod mappings;
#[cfg(test)]
mod mappings_tests;
pub mod marks;
#[cfg(test)]
mod marks_tests;
mod notify;
mod op_pending;
#[cfg(test)]
mod op_pending_tests;
pub mod registers;
#[cfg(test)]
mod registers_tests;
pub mod search_engine;
pub mod search_types;
pub mod session;
#[cfg(test)]
mod session_tests;
pub mod user_commands;
pub mod user_commands_parse;
#[cfg(test)]
mod user_commands_tests;
mod visual_ops;
#[cfg(test)]
mod visual_ops_tests;
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
pub use search_types::{CaseMode, SearchDirection, SearchMatch, SearchState};
pub use session::{SessionData, SessionManager};
pub use user_commands::{UserCommand, UserCommandRegistry};
pub use window_tree::WindowTree;
