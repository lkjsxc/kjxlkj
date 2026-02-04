//! End-to-end integration tests using headless mode.
//!
//! These tests run the kjxlkj binary in headless mode with scripts
//! to verify the editor's behavior end-to-end.

use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Get the path to the kjxlkj binary
fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Navigate up to workspace root
    path.pop();
    path.pop();
    path.pop();
    path.push("target");
    path.push("debug");
    path.push("kjxlkj");
    path
}

/// Run the editor in headless mode with a script.
fn run_headless(script: &str) -> Result<String, String> {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let script_path = format!("/tmp/kjxlkj_e2e_{}_{}.json", pid, id);
    fs::write(&script_path, script).map_err(|e| e.to_string())?;

    let bin = binary_path();
    if !bin.exists() {
        return Err(format!("Binary not found at {:?}", bin));
    }

    let output = Command::new(&bin)
        .args(["--headless", "--script", &script_path])
        .output()
        .map_err(|e| e.to_string())?;

    let _ = fs::remove_file(&script_path);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(format!("Failed: {} {}", stdout, stderr));
    }
    Ok(stdout)
}

#[test]
fn test_headless_mode_starts() {
    // Simple test that starts and exits immediately
    let script = r#"[
        {"kind": "key", "code": ":", "ctrl": false},
        {"kind": "keys", "keys": "q"},
        {"kind": "key", "code": "Enter", "ctrl": false}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Headless mode failed: {:?}", result.err());
}

#[test]
fn test_headless_insert_mode() {
    // Test entering insert mode and typing
    let script = r#"[
        {"kind": "key", "code": "i", "ctrl": false},
        {"kind": "assert_mode", "mode": "insert"},
        {"kind": "keys", "keys": "hello"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Insert mode test failed: {:?}", result.err());
}

#[test]
fn test_headless_cursor_movement() {
    // Test cursor movement commands
    let script = r#"[
        {"kind": "key", "code": "j", "ctrl": false},
        {"kind": "key", "code": "l", "ctrl": false},
        {"kind": "key", "code": "k", "ctrl": false},
        {"kind": "key", "code": "h", "ctrl": false}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Cursor movement test failed: {:?}", result.err());
}

#[test]
fn test_headless_command_mode() {
    // Test entering command mode and executing a command
    let script = r#"[
        {"kind": "key", "code": ":", "ctrl": false},
        {"kind": "assert_mode", "mode": "command"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Command mode test failed: {:?}", result.err());
}

#[test]
fn test_headless_visual_mode() {
    // Test entering visual mode
    let script = r#"[
        {"kind": "key", "code": "v", "ctrl": false},
        {"kind": "assert_mode", "mode": "visual"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Visual mode test failed: {:?}", result.err());
}

#[test]
fn test_headless_replace_mode() {
    // Test entering replace mode with R
    let script = r#"[
        {"kind": "key", "code": "R", "shift": true},
        {"kind": "assert_mode", "mode": "replace"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Replace mode test failed: {:?}", result.err());
}

#[test]
fn test_headless_append_mode() {
    // Test entering insert mode with a (append)
    let script = r#"[
        {"kind": "key", "code": "a", "ctrl": false},
        {"kind": "assert_mode", "mode": "insert"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Append mode test failed: {:?}", result.err());
}

#[test]
fn test_headless_open_line_below() {
    // Test entering insert mode with o (open line below)
    let script = r#"[
        {"kind": "key", "code": "o", "ctrl": false},
        {"kind": "assert_mode", "mode": "insert"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Open line below test failed: {:?}", result.err());
}

#[test]
fn test_headless_open_line_above() {
    // Test entering insert mode with O (open line above)
    let script = r#"[
        {"kind": "key", "code": "O", "shift": true},
        {"kind": "assert_mode", "mode": "insert"},
        {"kind": "key", "code": "Escape", "ctrl": false},
        {"kind": "assert_mode", "mode": "normal"}
    ]"#;

    let result = run_headless(script);
    assert!(result.is_ok(), "Open line above test failed: {:?}", result.err());
}
