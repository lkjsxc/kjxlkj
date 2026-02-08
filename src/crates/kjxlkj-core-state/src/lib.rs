//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod buffer;
#[cfg(test)]
mod buffer_tests;
mod command_dispatch;
mod editor;
mod editor_actions;
mod editor_actions2;
mod editor_file_ops;
mod editor_insert;
mod editor_marks;
mod editor_scroll;
mod editor_search_ops;
mod editor_snapshot;
mod editor_window_ops;
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
