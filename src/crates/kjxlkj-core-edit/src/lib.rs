//! kjxlkj-core-edit: editing primitives, operators, text objects, and UI components.

pub mod autopairs;
pub mod block_ops;
pub mod cmdline_window;
pub mod completion;
pub mod operators;
pub mod text_objects;
pub mod text_objects_ext;
pub mod ui_components;
pub mod ui_views;
pub mod visual;

pub use autopairs::{default_pairs, should_auto_close, should_skip_over, AutoPairConfig};
pub use block_ops::{build_block_edits, extend_to_eol, BlockEdit, BlockOp, BlockSelection};
pub use cmdline_window::{
    close as cmdline_close, edit_line, follow_cmdline_cursor, move_cursor as cmdline_move_cursor,
    open as cmdline_open, render_cmdline_window, CmdlineViewport, CmdlineWindowState,
};
pub use completion::{
    collect_buffer_words, collect_line_completions, CompletionItem, CompletionMenu,
    CompletionSource,
};
pub use operators::{
    apply_operator, change_range, delete_range, indent_range, lower_case_range, outdent_range,
    toggle_case_range, upper_case_range, yank_range, OperatorResult,
};
pub use text_objects::find_text_object;
pub use text_objects_ext::{
    find_argument, find_entire_buffer, find_indent_level, find_number, TextRange,
};
pub use ui_components::{component_at, layout_frame, Component, ComponentKind};
pub use ui_views::{TabPage, View, ViewKind, ViewManager};
pub use visual::{VisualKind, VisualSelection};
