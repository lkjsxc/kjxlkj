//! Tests for editor state.

use crate::EditorState;

#[test]
fn test_editor_new() {
    let state = EditorState::new();
    assert_eq!(state.buffer_count(), 1);
    assert!(state.active_buffer().is_some());
}

#[test]
fn test_create_buffer() {
    let mut state = EditorState::new();
    let id = state.create_buffer();
    assert_eq!(state.buffer_count(), 2);
    assert!(state.buffer(id).is_some());
}

#[test]
fn test_create_buffer_from() {
    let mut state = EditorState::new();
    let id = state.create_buffer_from("hello\nworld");
    let buf = state.buffer(id).unwrap();
    assert_eq!(buf.line_count(), 2);
}

#[test]
fn test_switch_buffer() {
    let mut state = EditorState::new();
    let id = state.create_buffer();
    assert!(state.switch_buffer(id));
    assert_eq!(state.active_buffer().unwrap().id, id);
}

#[test]
fn test_next_buffer() {
    let mut state = EditorState::new();
    let first = state.active_buffer().unwrap().id;
    let _second = state.create_buffer();
    
    assert!(state.next_buffer());
    let current = state.active_buffer().unwrap().id;
    assert_ne!(current, first);
}

#[test]
fn test_prev_buffer() {
    let mut state = EditorState::new();
    let first = state.active_buffer().unwrap().id;
    let _second = state.create_buffer();
    
    state.next_buffer();
    state.prev_buffer();
    let current = state.active_buffer().unwrap().id;
    assert_eq!(current, first);
}

#[test]
fn test_delete_buffer() {
    let mut state = EditorState::new();
    let first = state.active_buffer().unwrap().id;
    let second = state.create_buffer();
    
    assert!(state.delete_buffer(first));
    assert_eq!(state.buffer_count(), 1);
    assert!(state.buffer(second).is_some());
}

#[test]
fn test_cannot_delete_last_buffer() {
    let mut state = EditorState::new();
    let id = state.active_buffer().unwrap().id;
    assert!(!state.delete_buffer(id));
    assert_eq!(state.buffer_count(), 1);
}

#[test]
fn test_buffer_list() {
    let mut state = EditorState::new();
    state.create_buffer();
    
    let list = state.buffer_list();
    assert_eq!(list.len(), 2);
    assert!(list.iter().any(|b| b.active));
}

#[test]
fn test_buffer_ids() {
    let mut state = EditorState::new();
    state.create_buffer();
    state.create_buffer();
    
    let ids = state.buffer_ids();
    assert_eq!(ids.len(), 3);
}
