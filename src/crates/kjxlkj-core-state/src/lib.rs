//! Editor state aggregation for kjxlkj editor.
//!
//! This crate aggregates all editor state and produces snapshots.

mod abbreviation;
mod arglist;
mod autocmd;
mod buffer;
mod change;
mod changelist;
mod clipboard;
mod editor;
mod editor_buffer;
mod editor_window;
mod history;
mod history_types;
mod jump;
mod jumplist;
mod location;
mod macros;
mod marks;
mod quickfix;
mod register_types;
mod registers;
mod repeat;
mod session;
mod tabpage;
mod tag_types;
mod tagstack;
mod watch_types;
mod watcher;
mod window;
mod workspace;

#[cfg(test)]
mod tests;

pub use abbreviation::{Abbreviation, AbbrMode, AbbrStore};
pub use arglist::ArgList;
pub use autocmd::{AutoCmd, AutoCmdManager, AutoEvent};
pub use buffer::BufferState;
pub use change::{ChangeState, ChangeTracker, FileChange, FileTracker};
pub use clipboard::{ClipboardManager, ClipboardProvider, PrimarySelection, SystemClipboard};
pub use editor::EditorState;
pub use editor_buffer::BufferInfo;
pub use history::{History, HistoryList};
pub use history_types::{HistoryEntry, HistoryType};
pub use jump::Jump;
pub use jumplist::JumpList;
pub use changelist::ChangeList;
pub use location::{LocationEntry, LocationList};
pub use macros::{Macro, MacroStore, RecordingState};
pub use marks::{Mark, MarkStore, MarkType};
pub use quickfix::{QuickfixEntry, QuickfixKind, QuickfixList};
pub use registers::Registers;
pub use repeat::{RepeatKind, RepeatState};
pub use session::{SavedTab, SavedWindow, Session, SessionManager};
pub use tabpage::{TabId, TabManager, TabPage};
pub use tag_types::{TagDef, TagEntry, TagLocation, TagMatch};
pub use tagstack::TagStack;
pub use watch_types::{PendingEvent, WatchConfig, WatchEvent};
pub use watcher::{WatchManager, WatchState};
pub use window::WindowState;
pub use workspace::{RootMarker, Workspace, WorkspaceManager};
