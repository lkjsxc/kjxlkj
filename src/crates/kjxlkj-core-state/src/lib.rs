//! kjxlkj-core-state: editor state aggregation and snapshot production.
//!
//! This crate is the central coordinator, holding all editor state in
//! `EditorState` and producing `EditorSnapshot` for the rendering layer.

pub mod change_list;
pub mod command_dispatch;
pub mod command_dispatch_ext;
pub mod command_line_state;
pub mod commands;
pub mod commands_parse;
pub mod commands_parse_ext;
pub mod completion_engine;
pub mod completion_engine_ext;
pub mod config_store;
pub mod editor_state;
pub mod editor_state_buffers;
pub mod explorer;
pub mod file_commands;
pub mod jump_list;
pub mod macro_state;
pub mod marks;
pub mod options;
pub mod options_apply;
pub mod range_parser;
pub mod registers;
pub mod search;
pub mod search_regex;
pub mod session;
pub mod snapshot_producer;
pub mod syntax_cmd;
pub mod viewport;
pub mod window_state;

// Re-exports for convenience.
pub use change_list::{ChangeList, ChangeListEntry};
pub use command_dispatch::dispatch_command;
pub use command_line_state::{CmdlineAction, CommandHistory, CommandLineState};
pub use commands::ExCommand;
pub use commands_parse::parse_command;
pub use completion_engine::{CompletionSource, CompletionState};
pub use config_store::{build_defaults, ConfigStore, OptionDef, OptionScope};
pub use editor_state::EditorState;
pub use explorer::{ExplorerTree, GitBadge, TreeNode, TreeNodeKind};
pub use file_commands::FileCommand;
pub use jump_list::{JumpEntry, JumpList};
pub use macro_state::MacroState;
pub use marks::{MarkEntry, MarkRegistry, MarkScope};
pub use options::{EditorOptions, SetAction};
pub use range_parser::{Address, Range};
pub use registers::{RegisterEntry, DEFAULT_REGISTER, YANK_REGISTER};
pub use search::SearchState;
pub use search_regex::compile_pattern;
pub use session::{HistoryKind, SessionState};
pub use snapshot_producer::produce_snapshot;
pub use syntax_cmd::SyntaxAction;
pub use viewport::ViewportState;
pub use window_state::WindowState;
