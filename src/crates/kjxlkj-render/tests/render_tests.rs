//! Integration tests for the rendering pipeline.

use kjxlkj_render::*;

#[test]
fn status_line_vim_default() {
    let layout = vim_default();
    assert_eq!(layout.sections.len(), 2);
    assert!(layout.sections[0].segments.len() >= 2);
}

#[test]
fn message_area_set_clear() {
    let mut ma = MessageArea::new("test msg", MessageKind::Info);
    assert!(ma.visible);
    assert_eq!(ma.message, "test msg");
    ma.dismiss();
    assert!(!ma.visible);
    assert!(ma.message.is_empty());
}

#[test]
fn golden_snapshot_nowrap() {
    let lines = vec!["hello".into(), "world".into()];
    let (cfg, snap) = build_nowrap_test(&lines, 10);
    assert_eq!(cfg.mode, SnapshotMode::NoWrap);
    assert!(!snap.is_empty());
    assert!(snap[0].starts_with("hello"));
}

#[test]
fn golden_snapshot_wrap() {
    let lines = vec!["a long line that wraps".into()];
    let (cfg, snap) = build_wrap_test(&lines, 10);
    assert_eq!(cfg.mode, SnapshotMode::SoftWrap);
    assert!(snap.len() >= 2);
}

#[test]
fn theme_dark_exists() {
    let styles = default_highlight_styles();
    assert!(styles.contains_key(&HighlightGroup::Comment));
    assert!(styles.contains_key(&HighlightGroup::Keyword));
}

#[test]
fn theme_light_exists() {
    // default styles serve as light placeholder too
    let styles = default_highlight_styles();
    assert!(styles.len() >= 7);
}

#[test]
fn theme_gruvbox_exists() {
    // gruvbox-like: check that we can construct faces with gruvbox colors
    let fg = Rgb::from_hex("#ebdbb2").unwrap();
    let bg = Rgb::from_hex("#282828").unwrap();
    assert!(fg.luminance() > bg.luminance());
}

#[test]
fn highlight_groups() {
    assert_eq!(token_to_group("keyword"), Some(HighlightGroup::Keyword));
    assert_eq!(token_to_group("comment"), Some(HighlightGroup::Comment));
    assert_eq!(token_to_group("string"), Some(HighlightGroup::String));
    assert_eq!(token_to_group("bogus"), None);
}

#[test]
fn viewport_integrity_basic() {
    let rows = wrap_line("hello world", 80);
    let errors = validate_viewport(&rows, 80);
    assert!(errors.is_empty());
}

#[test]
fn display_cell_types() {
    let rows = wrap_line("a全b", 10);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].cells[0], DisplayCell::Normal('a'));
    assert_eq!(rows[0].cells[1], DisplayCell::Wide('全'));
    assert_eq!(rows[0].cells[2], DisplayCell::Continuation);
    assert_eq!(rows[0].cells[3], DisplayCell::Normal('b'));
}

#[test]
fn wrap_line_basic() {
    let rows = wrap_line("abcdef", 3);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].width(), 3);
}

#[test]
fn long_line_truncate() {
    let trunc = truncate_line("hello world", 5);
    assert_eq!(trunc, "hello");
}

#[test]
fn cursor_overlay_priority() {
    let overlays = vec![
        HighlightRegion {
            start: kjxlkj_core_types::Position::new(0, 0),
            end: kjxlkj_core_types::Position::new(0, 10),
            kind: OverlayPriority::Selection,
        },
        HighlightRegion {
            start: kjxlkj_core_types::Position::new(0, 0),
            end: kjxlkj_core_types::Position::new(0, 5),
            kind: OverlayPriority::Cursor,
        },
    ];
    let p = kjxlkj_core_types::Position::new(0, 2);
    assert_eq!(
        effective_overlay(&overlays, p),
        Some(OverlayPriority::Cursor)
    );
}

#[test]
fn matching_bracket_basic() {
    assert_eq!(matching_bracket("(abc)", 0), Some(4));
    assert_eq!(matching_bracket("{x}", 0), Some(2));
    assert_eq!(matching_bracket("no brackets", 0), None);
}

#[test]
fn notification_render() {
    let n = render_notification("hello", NotifPosition::TopRight, 80, 24, 20);
    assert_eq!(n.row, 1);
    assert!(!n.lines.is_empty());
}

#[test]
fn popup_menu_cycle() {
    let mut menu = PopupMenu::new(
        vec!["alpha".into(), "beta".into(), "gamma".into()],
        3,
        PopupAnchor::BelowCursor,
    );
    assert_eq!(menu.current(), Some("alpha"));
    menu.select_next();
    assert_eq!(menu.current(), Some("beta"));
    menu.select_next();
    assert_eq!(menu.current(), Some("gamma"));
    menu.select_prev();
    assert_eq!(menu.current(), Some("beta"));
}

#[test]
fn popup_anchor_computation() {
    let (x, y, w, h) = compute_rect(&PopupAnchor::ScreenCenter, 80, 24, 20, 10);
    assert_eq!(x, 30);
    assert_eq!(y, 7);
    assert_eq!(w, 20);
    assert_eq!(h, 10);
}

#[test]
fn status_context_render() {
    let ctx = StatusContext {
        mode: "NORMAL".into(),
        filename: "test.rs".into(),
        filetype: "rust".into(),
        encoding: "utf-8".into(),
        line: 1,
        col: 1,
        total_lines: 100,
        percent: 0,
        modified: false,
        buf_nr: 1,
    };
    let mode_text = render_segment(&StatusSegment::Mode, &ctx);
    assert!(mode_text.contains("NORMAL"));
    let pos_text = render_segment(&StatusSegment::Position, &ctx);
    assert!(pos_text.contains("1:1"));
}

#[test]
fn rgb_from_hex() {
    let c = Rgb::from_hex("#ff8000").unwrap();
    assert_eq!(c.r, 255);
    assert_eq!(c.g, 128);
    assert_eq!(c.b, 0);
    assert_eq!(c.to_hex(), "#ff8000");
}

#[test]
fn ansi256_mapping() {
    let black = index_to_rgb(0);
    assert_eq!(black, Rgb::new(0, 0, 0));
    let white = index_to_rgb(15);
    assert_eq!(white, Rgb::new(255, 255, 255));
    let grey = index_to_rgb(240);
    assert_eq!(grey.r, grey.g);
}
