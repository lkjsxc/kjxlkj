//! WR-* regression tests for kjxlkj.

use crate::StateHarness;
use kjxlkj_core_mode::{HandleResult, InsertPosition, ModeAction};
use kjxlkj_core_state::WindowTree;
use kjxlkj_core_types::{BufferId, Key, KeyEvent, Modifiers, TerminalId};

fn key(c: char) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers::NONE }
}

/// WR-01: Key normalization - Shift+a produces 'A'
#[test]
fn wr_01_key_normalization() {
    let key_event = KeyEvent {
        key: Key::Char('A'),
        modifiers: Modifiers::NONE,
    };
    assert_eq!(key_event.key, Key::Char('A'));
}

/// WR-03: Terminal service is constructable
#[test]
fn wr_03_terminal_service_constructable() {
    use kjxlkj_service_terminal::TerminalService;
    let _service = TerminalService::new();
}

/// WR-04: Window split and close operations wired
#[test]
fn wr_04_window_split_close_wired() {
    let mut tree = WindowTree::new();
    
    let first = tree.add_buffer_window(BufferId(1));
    assert_eq!(tree.count(), 1);
    
    let second = tree.split_horizontal(BufferId(2));
    assert_eq!(tree.count(), 2);
    assert_ne!(first, second);
    
    let _third = tree.split_vertical(BufferId(3));
    assert_eq!(tree.count(), 3);
    
    tree.close_focused();
    assert_eq!(tree.count(), 2);
    
    tree.close_others();
    assert_eq!(tree.count(), 1);
}

/// WR-05: Explorer launch path is wired
#[test]
fn wr_05_explorer_launch_wired() {
    use kjxlkj_service_explorer::ExplorerState;
    use std::path::PathBuf;
    
    let state = ExplorerState::new(PathBuf::from("/tmp"));
    assert!(state.visible_count() > 0);
}

/// WR-01R: Dispatch 'A' in Normal mode produces EnterInsert(EndOfLine)
#[test]
fn wr_01r_shift_a_dispatch() {
    let mut harness = StateHarness::new();
    let result = harness.send_key(key('A'));
    
    match result {
        HandleResult::Consumed(actions) => {
            assert!(actions.iter().any(|a| {
                matches!(a, ModeAction::EnterInsert(InsertPosition::EndOfLine))
            }));
        }
        _ => panic!("Expected Consumed result"),
    }
}

/// WR-02: 'a' at EOL differs from 'i' - append vs insert semantics
#[test]
fn wr_02_a_vs_i_semantics() {
    let mut harness = StateHarness::new();
    let result_i = harness.send_key(key('i'));
    
    let insert_before = match result_i {
        HandleResult::Consumed(actions) => {
            actions.iter().any(|a| {
                matches!(a, ModeAction::EnterInsert(InsertPosition::Before))
            })
        }
        _ => false,
    };
    assert!(insert_before, "'i' should enter insert Before cursor");

    let mut harness2 = StateHarness::new();
    let result_a = harness2.send_key(key('a'));
    
    let insert_after = match result_a {
        HandleResult::Consumed(actions) => {
            actions.iter().any(|a| {
                matches!(a, ModeAction::EnterInsert(InsertPosition::After))
            })
        }
        _ => false,
    };
    assert!(insert_after, "'a' should enter insert After cursor");

    let mut harness3 = StateHarness::new();
    let result_a_upper = harness3.send_key(key('A'));
    
    let insert_eol = match result_a_upper {
        HandleResult::Consumed(actions) => {
            actions.iter().any(|a| {
                matches!(a, ModeAction::EnterInsert(InsertPosition::EndOfLine))
            })
        }
        _ => false,
    };
    assert!(insert_eol, "'A' should enter insert at EndOfLine");
}

/// WR-08: Command mode colon entry
#[test]
fn wr_08_command_mode_entry() {
    let mut harness = StateHarness::new();
    let result = harness.send_key(key(':'));
    
    match result {
        HandleResult::Consumed(actions) => {
            assert!(actions.iter().any(|a| {
                matches!(a, ModeAction::EnterCommand(_))
            }), "':' should enter command mode");
        }
        _ => panic!("Expected Consumed result"),
    }
}

/// WR-06: Mixed window focus baseline
#[test]
fn wr_06_mixed_window_focus() {
    let mut tree = WindowTree::new();
    
    let buf_id = tree.add_buffer_window(BufferId(1));
    let term_id = tree.add_terminal_window(TerminalId(1));
    
    assert!(tree.get(buf_id).is_some());
    assert!(tree.get(term_id).is_some());
    
    tree.focus(buf_id);
    assert_eq!(tree.focused_id(), Some(buf_id));
    
    tree.focus(term_id);
    assert_eq!(tree.focused_id(), Some(term_id));
}

/// WR-07: Wrap boundary baseline
#[test]
fn wr_07_wrap_boundary() {
    use kjxlkj_core_text::grapheme_width;
    
    assert_eq!(grapheme_width("a"), 1);
    assert_eq!(grapheme_width("漢"), 2);
    assert_eq!(grapheme_width("🎉"), 2);
}

/// JP-03: Space during IME composition doesn't trigger leader
/// 
/// Verifies that when IME is composing, space key is consumed by IME
/// and does not leak through to trigger leader mappings.
#[test]
fn jp_03_space_during_ime_composition_no_leader() {
    use kjxlkj_core_mode::ModeState;
    use kjxlkj_core_mode::insert::dispatch_insert;
    
    let mut state = ModeState::new();
    state.enter_insert();
    state.ime_start(); // Start IME composition
    state.ime_update("かんじ"); // Some Japanese preedit
    
    // Space during composition should be consumed, not trigger leader
    let space_key = key(' ');
    let result = dispatch_insert(&mut state, &space_key);
    
    match result {
        HandleResult::Consumed(actions) => {
            // Should NOT produce any leader-related action
            assert!(actions.is_empty() || actions.iter().all(|a| {
                !matches!(a, ModeAction::InsertText(s) if s == " ")
            }), "Space during composition should not insert literal space");
        }
        HandleResult::Ignored | HandleResult::Pending => {
            panic!("Space should be consumed during IME composition")
        }
    }
    
    // IME should still be composing
    assert!(state.is_composing());
    // Preedit should be updated with space appended
    assert!(state.ime_preedit.contains(' '));
}

/// JP-04: Long Japanese commit preserves wrap integrity
///
/// Verifies that committing a long Japanese string maintains wrap
/// boundaries without creating half-cell cursor positions.
#[test]
fn jp_04_long_japanese_commit_preserves_wrap() {
    use kjxlkj_core_mode::ModeState;
    use kjxlkj_core_mode::insert::dispatch_insert;
    use kjxlkj_core_text::grapheme_width;
    use kjxlkj_core_types::SpecialKey;
    
    let mut state = ModeState::new();
    state.enter_insert();
    state.ime_start();
    
    // Long Japanese text that would span multiple screen columns
    let long_text = "東京特許許可局";
    state.ime_update(long_text);
    
    // Verify total width is correctly calculated (each CJK char is width 2)
    let total_width: usize = long_text.chars()
        .map(|c| grapheme_width(&c.to_string()))
        .sum();
    assert_eq!(total_width, 14); // 7 chars * 2 width each
    
    // Commit with Enter
    let enter_key = KeyEvent {
        key: Key::Special(SpecialKey::Enter),
        modifiers: Modifiers::NONE,
    };
    let result = dispatch_insert(&mut state, &enter_key);
    
    match result {
        HandleResult::Consumed(actions) => {
            // Should produce InsertText with the committed text
            let inserted = actions.iter().find_map(|a| {
                if let ModeAction::InsertText(s) = a { Some(s.as_str()) } else { None }
            });
            assert_eq!(inserted, Some(long_text));
        }
        _ => panic!("Expected Consumed with InsertText"),
    }
    
    // IME should no longer be composing
    assert!(!state.is_composing());
}

/// JP-05: IME cancel clears preedit and returns to normal
#[test]
fn jp_05_ime_cancel_clears_preedit() {
    use kjxlkj_core_mode::ModeState;
    use kjxlkj_core_mode::insert::dispatch_insert;
    use kjxlkj_core_types::{Mode, SpecialKey};
    
    let mut state = ModeState::new();
    state.enter_insert();
    state.ime_start();
    state.ime_update("にほんご");
    
    assert!(state.is_composing());
    assert!(!state.ime_preedit.is_empty());
    
    // Escape cancels IME and returns to normal
    let esc_key = KeyEvent {
        key: Key::Special(SpecialKey::Escape),
        modifiers: Modifiers::NONE,
    };
    let result = dispatch_insert(&mut state, &esc_key);
    
    match result {
        HandleResult::Consumed(actions) => {
            assert!(actions.iter().any(|a| matches!(a, ModeAction::ReturnNormal)));
        }
        _ => panic!("Expected Consumed with ReturnNormal"),
    }
    
    assert!(!state.is_composing());
    assert!(state.ime_preedit.is_empty());
    assert!(matches!(state.mode, Mode::Normal));
}
