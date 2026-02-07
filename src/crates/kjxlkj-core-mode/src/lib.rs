//! kjxlkj-core-mode: modal state machine and mode-related utilities.

pub mod cursor_rendering;
pub mod mode_keybindings;
pub mod mode_state;
pub mod popup_overlay;
pub mod transitions;

pub use cursor_rendering::{
    blink_state, cursor_for_mode, cursor_shape_escape, cursor_visible, BlinkState, CursorShape,
    ModeCursorConfig, ModeCursorMap,
};
pub use mode_keybindings::{
    build_normal_bindings, check_mode_coverage, ModeBindingTable, UxBinding, UxMode,
};
pub use mode_state::{ChangeRecord, ModeState};
pub use popup_overlay::{OverlayManager, PopupAnchor, PopupKind, PopupState};
pub use transitions::{can_transition, validate_transition, TransitionError};
