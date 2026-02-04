//! Buffer file operations integration tests.
//!
//! Tests for buffer open/edit/write operations as required by
//! /docs/todo/current/wave-implementation/editor/buffers/README.md

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion};
use std::path::PathBuf;

/// Test: Create a new buffer.
#[test]
fn test_buffer_creation() {
    let buffer = TextBuffer::new(BufferId::new(1));
    assert_eq!(buffer.id(), BufferId::new(1));
    assert_eq!(buffer.name().as_str(), "[No Name]");
    assert!(buffer.path().is_none());
    assert_eq!(buffer.version(), BufferVersion::INITIAL);
    assert!(!buffer.is_modified());
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.char_count(), 0);
}

/// Test: Create buffer from text.
#[test]
fn test_buffer_from_text() {
    let content = "Hello, world!\nSecond line.";
    let buffer = TextBuffer::from_text(BufferId::new(1), content);
    
    assert_eq!(buffer.to_string(), content);
    assert_eq!(buffer.line_count(), 2);
    assert!(!buffer.is_modified());
}

/// Test: Create buffer from file path.
#[test]
fn test_buffer_from_file() {
    let path = PathBuf::from("/test/path/file.txt");
    let content = "File content";
    let buffer = TextBuffer::from_file(BufferId::new(1), path.clone(), content);
    
    assert_eq!(buffer.name().as_str(), "file.txt");
    assert_eq!(buffer.path(), Some(&path));
    assert_eq!(buffer.to_string(), content);
}

/// Test: Buffer identity is stable.
#[test]
fn test_buffer_identity_stable() {
    let buffer = TextBuffer::new(BufferId::new(42));
    let id = buffer.id();
    
    // Clone and verify ID is same
    let buffer2 = buffer.clone();
    assert_eq!(buffer2.id(), id);
    
    // ID should remain stable regardless of operations
    let mut buffer3 = buffer.clone();
    buffer3.insert(0, "text");
    assert_eq!(buffer3.id(), id);
}

/// Test: Buffer version increments on edit.
#[test]
fn test_buffer_version_increment() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "initial");
    let v0 = buffer.version();
    
    buffer.insert(0, "x");
    let v1 = buffer.version();
    assert!(v1 > v0);
    
    buffer.remove(0, 1);
    let v2 = buffer.version();
    assert!(v2 > v1);
}

/// Test: Buffer modified flag.
#[test]
fn test_buffer_modified_flag() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "initial");
    assert!(!buffer.is_modified());
    
    buffer.insert(0, "x");
    assert!(buffer.is_modified());
    
    buffer.mark_saved();
    assert!(!buffer.is_modified());
}

/// Test: Basic insert operation.
#[test]
fn test_buffer_insert() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "hello");
    buffer.insert(5, " world");
    assert_eq!(buffer.to_string(), "hello world");
}

/// Test: Insert at line and column.
#[test]
fn test_buffer_insert_at() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "line 1\nline 2");
    buffer.insert_at(1, 5, " inserted");
    assert_eq!(buffer.to_string(), "line 1\nline  inserted2");
}

/// Test: Remove operation.
#[test]
fn test_buffer_remove() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
    buffer.remove(5, 11);
    assert_eq!(buffer.to_string(), "hello");
}

/// Test: Line access.
#[test]
fn test_buffer_line_access() {
    let buffer = TextBuffer::from_text(BufferId::new(1), "line 1\nline 2\nline 3");
    
    assert_eq!(buffer.line_count(), 3);
    
    let line1 = buffer.line(0).unwrap();
    assert!(line1.as_str().unwrap().starts_with("line 1"));
    
    let line2 = buffer.line(1).unwrap();
    assert!(line2.as_str().unwrap().starts_with("line 2"));
}

/// Test: Line grapheme length.
#[test]
fn test_buffer_line_grapheme_len() {
    let buffer = TextBuffer::from_text(BufferId::new(1), "hello\n世界");
    
    // "hello" = 5 graphemes
    assert_eq!(buffer.line_grapheme_len(0), 5);
    // "世界" = 2 graphemes
    assert_eq!(buffer.line_grapheme_len(1), 2);
}

/// Test: Unicode handling.
#[test]
fn test_buffer_unicode() {
    let content = "Hello 世界\n🎉 emoji\n";
    let mut buffer = TextBuffer::from_text(BufferId::new(1), content);
    
    // Verify content is preserved
    assert_eq!(buffer.to_string(), content);
    
    // Insert unicode
    buffer.insert(0, "🚀");
    assert!(buffer.to_string().starts_with("🚀"));
}

/// Test: Empty buffer handling.
#[test]
fn test_buffer_empty() {
    let buffer = TextBuffer::new(BufferId::new(1));
    assert_eq!(buffer.char_count(), 0);
    assert_eq!(buffer.line_count(), 1); // Empty buffer has one empty line
    assert_eq!(buffer.line_grapheme_len(0), 0);
}

/// Test: Large content handling.
#[test]
fn test_buffer_large_content() {
    let line = "This is a line of text.\n";
    let content = line.repeat(10000);
    let buffer = TextBuffer::from_text(BufferId::new(1), &content);
    
    assert_eq!(buffer.line_count(), 10001);
    assert_eq!(buffer.to_string(), content);
}

/// Test: Buffer name operations.
#[test]
fn test_buffer_name_operations() {
    let mut buffer = TextBuffer::new(BufferId::new(1));
    assert_eq!(buffer.name().as_str(), "[No Name]");
    
    buffer.set_name(BufferName::new("custom.txt"));
    assert_eq!(buffer.name().as_str(), "custom.txt");
}

/// Test: Buffer path operations.
#[test]
fn test_buffer_path_operations() {
    let mut buffer = TextBuffer::new(BufferId::new(1));
    assert!(buffer.path().is_none());
    
    let path = PathBuf::from("/home/user/test.txt");
    buffer.set_path(path.clone());
    assert_eq!(buffer.path(), Some(&path));
    assert_eq!(buffer.name().as_str(), "test.txt");
}

/// Test: Multiple edits.
#[test]
fn test_buffer_multiple_edits() {
    let mut buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
    
    buffer.remove(6, 11);  // "hello "
    buffer.insert(6, "Rust");  // "hello Rust"
    
    assert_eq!(buffer.to_string(), "hello Rust");
}

/// Test: Deterministic behavior.
#[test]
fn test_buffer_determinism() {
    let content = "test content";
    
    // Same operations should produce identical results
    let mut buffer1 = TextBuffer::from_text(BufferId::new(1), content);
    let mut buffer2 = TextBuffer::from_text(BufferId::new(1), content);
    
    buffer1.insert(0, "prefix: ");
    buffer2.insert(0, "prefix: ");
    
    assert_eq!(buffer1.to_string(), buffer2.to_string());
}
