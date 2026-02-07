//! kjxlkj-core-ui: UI model types and snapshot structures.

pub mod cursor_full;
pub mod floating;
pub mod layout;
pub mod line_numbers;
pub mod search_highlight;
pub mod snapshot;
pub mod theme;
pub mod theme_builtin;
pub mod view_tree;

pub use cursor_full::{check_cursor_in_viewport, check_transition_visibility, cursor_for_mode};
pub use floating::{
    compute_preset_rects, FloatAnchor, FloatBorder, FloatConfig, LayoutPreset, ResizeOp,
    WinCmd, ZoomState,
};
pub use layout::{standard_layout, ComponentKind, LayoutNode, Rect};
pub use line_numbers::{format_line_number, format_mode_indicator, LineNumberStyle, ModeIndicatorFormat};
pub use search_highlight::{SearchHighlights, SearchMatch};
pub use snapshot::{BufferSnapshot, CursorHint, CursorShape, CursorState, EditorSnapshot, WindowSnap};
pub use theme::{Theme, ThemePalette, ThemeRegistry, ThemeStyle};
pub use theme_builtin::{theme_dark, theme_gruvbox, theme_light};
pub use view_tree::{FocusTarget, ViewNode, ViewTree};
