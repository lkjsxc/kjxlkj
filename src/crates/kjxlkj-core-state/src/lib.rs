//! Editor state aggregation and snapshot production.

mod action_dispatch;
mod buffer;
mod command_dispatch;
mod editor;
mod editor_actions;
mod editor_actions2;
mod session;
mod viewport;
mod window;

pub use buffer::{BufferState, LineEnding};
pub use command_dispatch::dispatch_command;
pub use editor::EditorState;
pub use session::{load_session, save_session, SessionData, SessionLayout};
pub use viewport::ViewportState;
pub use window::{WindowContent, WindowOptions, WindowState};
