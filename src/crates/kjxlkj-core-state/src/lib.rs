//! Editor state aggregation for kjxlkj editor.
//!
//! This crate aggregates all editor state and produces snapshots.

mod abbreviation;
mod arglist;
mod autocmd;
mod buffer;
mod change;
mod clipboard;
mod editor;
mod history;
mod jumplist;
mod location;
mod macros;
mod marks;
mod quickfix;
mod registers;
mod repeat;
mod session;
mod tabpage;
mod tagstack;
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
pub use editor::{BufferInfo, EditorState};
pub use history::{History, HistoryEntry, HistoryList, HistoryType};
pub use jumplist::{ChangeList, Jump, JumpList};
pub use location::{LocationEntry, LocationList};
pub use macros::{Macro, MacroStore, RecordingState};
pub use marks::{Mark, MarkStore, MarkType};
pub use quickfix::{QuickfixEntry, QuickfixKind, QuickfixList};
pub use registers::Registers;
pub use repeat::{RepeatKind, RepeatState};
pub use session::{SavedTab, SavedWindow, Session, SessionManager};
pub use tabpage::{TabId, TabManager, TabPage};
pub use tagstack::{TagDef, TagEntry, TagLocation, TagMatch, TagStack};
pub use watcher::{PendingEvent, WatchConfig, WatchEvent, WatchManager, WatchState};
pub use window::WindowState;
pub use workspace::{RootMarker, Workspace, WorkspaceManager};
