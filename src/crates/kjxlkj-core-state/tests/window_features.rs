//! Window management, resize, float, and layout preset tests.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Intent, KeyEvent, MotionKind, Size};

fn editor() -> EditorState {
    let mut ed = EditorState::new(Size::new(80, 24));
    let bid = ed.create_buffer_from_text("line1\nline2\nline3\nline4\n");
    ed.create_window(bid);
    ed
}

fn win_count(ed: &EditorState) -> usize { ed.windows.len() }

#[test]
fn split_horizontal_creates_window() {
    let mut ed = editor();
    assert_eq!(win_count(&ed), 1);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    assert_eq!(win_count(&ed), 2);
}

#[test]
fn split_vertical_creates_window() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitVertical);
    assert_eq!(win_count(&ed), 2);
}

#[test]
fn close_removes_window() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    assert_eq!(win_count(&ed), 2);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowClose);
    assert_eq!(win_count(&ed), 1);
}

#[test]
fn close_last_window_refused() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowClose);
    assert_eq!(win_count(&ed), 1);
    assert!(ed.message.as_deref().unwrap().contains("Cannot close"));
}

#[test]
fn only_closes_all_but_active() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    assert_eq!(win_count(&ed), 3);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowOnly);
    assert_eq!(win_count(&ed), 1);
}

#[test]
fn focus_next_cycles() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    let first = ed.active_window;
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowFocusNext);
    let second = ed.active_window;
    assert_ne!(first, second);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowFocusNext);
    assert_eq!(ed.active_window, first);
}

#[test]
fn focus_prev_cycles() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    let first_focus = ed.active_window; // After split, active is new window
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowFocusPrev);
    assert_ne!(ed.active_window, first_focus);
}

#[test]
fn focus_direction_down_goes_next() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    let after_split = ed.active_window;
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowFocusDirection(MotionKind::Down));
    assert_ne!(ed.active_window, after_split);
}

#[test]
fn equal_size_distributes() {
    let mut ed = editor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowEqualSize);
    let heights: Vec<usize> = ed.windows.values().map(|w| w.height).collect();
    assert_eq!(heights[0], heights[1]);
}

#[test]
fn rotate_swaps_buffers() {
    let mut ed = editor();
    let bid1 = ed.create_buffer_from_text("buffer2\n");
    ed.create_window(bid1);
    let bufs_before: Vec<_> = ed.windows.values().map(|w| w.buffer_id).collect();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowRotate);
    let bufs_after: Vec<_> = ed.windows.values().map(|w| w.buffer_id).collect();
    assert_ne!(bufs_before, bufs_after);
}

#[test]
fn split_inherits_cursor() {
    let mut ed = editor();
    // Move cursor to line 2
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::Motion(MotionKind::Down, 1));
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::Motion(MotionKind::Down, 1));
    let cursor_before = ed.cursor();
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    let new_wid = ed.active_window.unwrap();
    let new_win = ed.windows.get(&new_wid).unwrap();
    assert_eq!(new_win.cursor_line, cursor_before.line);
}

#[test]
fn multiple_splits_then_only() {
    let mut ed = editor();
    for _ in 0..5 {
        kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    }
    assert_eq!(win_count(&ed), 6);
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowOnly);
    assert_eq!(win_count(&ed), 1);
}

#[test]
fn split_reduces_height() {
    let mut ed = editor();
    let orig_height = ed.active_window_state().unwrap().height;
    kjxlkj_core_state::dispatch_intent(&mut ed, Intent::WindowSplitHorizontal);
    // Both windows should have reduced height
    for win in ed.windows.values() {
        assert!(win.height < orig_height);
    }
}
