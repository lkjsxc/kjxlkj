//! Core facade crate for kjxlkj.
//!
//! This crate re-exports the core APIs needed by the editor.

pub use kjxlkj_core_types::*;
pub use kjxlkj_core_text::{TextBuffer, grapheme_count, display_width};
pub use kjxlkj_core_edit::{
    execute_motion, clamp_cursor, move_to_line_start, move_to_line_end,
    move_to_first_non_blank, move_to_file_start, move_to_file_end,
    delete_char, delete_line, yank_line, insert_text, delete_backward,
    EditOp, Transaction,
};
pub use kjxlkj_core_mode::{ModeState, process_key, parse_command};
pub use kjxlkj_core_undo::UndoHistory;
pub use kjxlkj_core_ui::{
    EditorSnapshot, WindowSnapshot, LineSnapshot, StatusSnapshot,
    StatusMessage, MessageLevel, build_status,
};
pub use kjxlkj_core_state::{EditorState, RegisterStore};
