//! kjxlkj-render: rendering pipeline – converts editor snapshots to terminal frames.

pub mod cursor_overlay;
pub mod golden_snapshots;
pub mod highlight;
pub mod long_line_fixtures;
pub mod message_area;
pub mod notification;
pub mod popup_menu;
pub mod renderer;
pub mod status_line;
pub mod theme_full;
pub mod viewport_integrity;

pub use cursor_overlay::{
    cursor_in_viewport, effective_overlay, matching_bracket, resolve_cursor_col, BoundaryAction,
    HighlightRegion, OverlayPriority,
};
pub use golden_snapshots::{
    build_nowrap_test, build_wrap_test, compare_snapshot, render_snapshot, SnapshotConfig,
    SnapshotMode,
};
pub use highlight::{
    default_highlight_styles, highlight_line, token_to_group, HighlightGroup, HighlightSpan,
};
pub use long_line_fixtures::{
    all_fixtures, generate_fixture, verify_fixture, FixtureKind, LineFixture,
};
pub use message_area::{format_error, format_info, render_message_area, MessageArea, MessageKind};
pub use notification::{
    max_visible_notifications, render_notification, wrap_text, NotifPosition, RenderedNotif,
};
pub use popup_menu::{compute_rect, HoverTooltip, PopupAnchor, PopupMenu};
pub use renderer::Renderer;
pub use status_line::{
    render_segment, render_status_line, vim_default, StatusContext, StatusLineLayout,
    StatusSection, StatusSegment,
};
pub use theme_full::{index_to_rgb, resolve_color, Face, Rgb, ThemeColor};
pub use viewport_integrity::{
    is_long_line, truncate_line, validate_viewport, wrap_line, DisplayCell, DisplayRow,
};
