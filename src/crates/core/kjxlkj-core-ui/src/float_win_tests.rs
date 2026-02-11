use super::*;
use kjxlkj_core_types::{BufferId, WindowId};

fn cfg() -> FloatConfig {
    FloatConfig::dialog(40, 10)
}

#[test]
fn open_and_close() {
    let mut fl = FloatLayer::new();
    fl.open(WindowId(10), BufferId(0), cfg());
    assert_eq!(fl.count(), 1);
    assert!(fl.close(WindowId(10)));
    assert!(fl.is_empty());
}

#[test]
fn render_order_by_zindex() {
    let mut fl = FloatLayer::new();
    let mut c1 = cfg();
    c1.zindex = 100;
    fl.open(WindowId(1), BufferId(0), c1);
    let mut c2 = cfg();
    c2.zindex = 50;
    fl.open(WindowId(2), BufferId(0), c2);
    let order = fl.render_order();
    assert_eq!(order[0].window_id, WindowId(2));
    assert_eq!(order[1].window_id, WindowId(1));
}

#[test]
fn render_order_creation_tiebreak() {
    let mut fl = FloatLayer::new();
    fl.open(WindowId(1), BufferId(0), cfg());
    fl.open(WindowId(2), BufferId(0), cfg());
    let order = fl.render_order();
    assert_eq!(order[0].window_id, WindowId(1));
    assert_eq!(order[1].window_id, WindowId(2));
}

#[test]
fn focusable_skip_non_focusable() {
    let mut fl = FloatLayer::new();
    let tc = FloatConfig::tooltip(20, 3);
    fl.open(WindowId(5), BufferId(0), tc);
    assert!(fl.focusable(WindowId(5)).is_none());
}

#[test]
fn focusable_finds_dialog() {
    let mut fl = FloatLayer::new();
    fl.open(WindowId(7), BufferId(0), cfg());
    assert!(fl.focusable(WindowId(7)).is_some());
}

#[test]
fn tooltip_defaults() {
    let tc = FloatConfig::tooltip(20, 3);
    assert!(!tc.focusable);
    assert!(!tc.enter);
    assert!(tc.close_on_focus_loss);
    assert_eq!(tc.anchor, FloatAnchor::Cursor);
}

#[test]
fn dialog_defaults() {
    let dc = FloatConfig::dialog(40, 10);
    assert!(dc.focusable);
    assert!(dc.enter);
    assert!(dc.center);
    assert_eq!(dc.border, BorderStyle::Rounded);
}
