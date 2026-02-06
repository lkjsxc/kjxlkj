use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Intent, Size};

fn setup() -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("hello\nworld");
    s.create_window(bid);
    s
}

#[test]
fn split_creates_new_window() {
    let mut s = setup();
    assert_eq!(s.windows.len(), 1);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 2);
}

#[test]
fn close_prevents_last_window() {
    let mut s = setup();
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowClose);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn close_removes_window() {
    let mut s = setup();
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 2);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowClose);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn only_removes_others() {
    let mut s = setup();
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    assert_eq!(s.windows.len(), 3);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowOnly);
    assert_eq!(s.windows.len(), 1);
}

#[test]
fn focus_next_cycles() {
    let mut s = setup();
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    let first = s.active_window;
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowFocusNext);
    assert_ne!(s.active_window, first);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowFocusNext);
    assert_eq!(s.active_window, first);
}

#[test]
fn equal_size_distributes() {
    let mut s = setup();
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowSplitHorizontal);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::WindowEqualSize);
    let heights: Vec<_> = s.windows.values().map(|w| w.height).collect();
    assert!(heights.iter().all(|h| *h == heights[0]));
}
