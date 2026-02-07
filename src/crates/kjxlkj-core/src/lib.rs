//! kjxlkj-core: facade crate re-exporting core APIs and implementing
//! the main action dispatch loop.

pub mod command_mode;
pub mod core_loop;
pub mod core_loop_dispatch;
pub mod digraph;
pub mod digraph_lookup;
pub mod insert_mode;
pub mod insert_mode_ctrl;
pub mod lsp_features;
pub mod normal_mode;
pub mod normal_mode_ext;
pub mod normal_mode_ops;
pub mod normal_mode_scroll;
pub mod replace_mode;
pub mod visual_mode;
pub mod visual_mode_ops;

// Re-export sub-crates for consumer convenience.
pub use kjxlkj_core_edit as edit;
pub use kjxlkj_core_mode as mode;
pub use kjxlkj_core_state as state;
pub use kjxlkj_core_text as text;
pub use kjxlkj_core_types as types;
pub use kjxlkj_core_ui as ui;
pub use kjxlkj_core_undo as undo;

// Re-export key types at crate root.
pub use kjxlkj_core_state::EditorState;
pub use kjxlkj_core_types::{
    EditorAction, EditorEvent, KeyCode, KeyEvent, Mode, Modifiers, Motion, Operator,
    OperatorTarget, Position, Range,
};
pub use kjxlkj_core_ui::snapshot::EditorSnapshot;

pub use core_loop::CoreProcessor;
pub use lsp_features::{
    CodeAction, CodeActionKind, CompletionItem, CompletionKind, Diagnostic, DiagnosticSeverity,
    DiagnosticStore, HoverInfo,
};
