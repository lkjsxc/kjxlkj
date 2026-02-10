//! JP-* IME composition tests for kjxlkj.

use kjxlkj_core_mode::{HandleResult, ModeAction, ModeState};
use kjxlkj_core_mode::insert::dispatch_insert;
use kjxlkj_core_text::grapheme_width;
use kjxlkj_core_types::{Key, KeyEvent, Mode, Modifiers, SpecialKey};

fn key(c: char) -> KeyEvent {
    KeyEvent { key: Key::Char(c), modifiers: Modifiers::NONE }
}

/// JP-03: Space during IME composition doesn't trigger leader
/// 
/// Verifies that when IME is composing, space key is consumed by IME
/// and does not leak through to trigger leader mappings.
#[test]
fn jp_03_space_during_ime_composition_no_leader() {
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

/// JP-06: IME backspace removes from preedit without deleting buffer
#[test]
fn jp_06_ime_backspace_removes_preedit() {
    let mut state = ModeState::new();
    state.enter_insert();
    state.ime_start();
    state.ime_update("あいう");
    
    assert_eq!(state.ime_preedit, "あいう");
    
    let backspace_key = KeyEvent {
        key: Key::Special(SpecialKey::Backspace),
        modifiers: Modifiers::NONE,
    };
    let result = dispatch_insert(&mut state, &backspace_key);
    
    match result {
        HandleResult::Consumed(actions) => {
            // Should NOT produce DeleteAtCursor action while preedit has content
            assert!(actions.is_empty(), "Backspace should only modify preedit, not delete from buffer");
        }
        _ => panic!("Expected Consumed"),
    }
    
    // One character should be removed from preedit
    assert_eq!(state.ime_preedit, "あい");
    assert!(state.is_composing());
}

/// JP-07: Character accumulation during IME composition
#[test]
fn jp_07_char_accumulation_in_preedit() {
    let mut state = ModeState::new();
    state.enter_insert();
    state.ime_start();
    
    // Simulate typing characters during composition
    let _ = dispatch_insert(&mut state, &key('n'));
    assert_eq!(state.ime_preedit, "n");
    
    let _ = dispatch_insert(&mut state, &key('i'));
    assert_eq!(state.ime_preedit, "ni");
    
    let _ = dispatch_insert(&mut state, &key('h'));
    assert_eq!(state.ime_preedit, "nih");
    
    let _ = dispatch_insert(&mut state, &key('o'));
    assert_eq!(state.ime_preedit, "niho");
    
    let _ = dispatch_insert(&mut state, &key('n'));
    assert_eq!(state.ime_preedit, "nihon");
    
    assert!(state.is_composing());
}
