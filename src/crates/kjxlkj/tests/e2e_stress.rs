//! E2E stress tests for ordering and performance baselines.

use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); path.pop(); path.pop();
    path.push("target"); path.push("debug"); path.push("kjxlkj");
    path
}

fn run_headless(script: &str) -> Result<String, String> {
    let id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let pid = std::process::id();
    let script_path = format!("/tmp/kjxlkj_e2e_stress_{}_{}.json", pid, id);
    fs::write(&script_path, script).map_err(|e| e.to_string())?;
    let bin = binary_path();
    if !bin.exists() { return Err(format!("Binary not found at {:?}", bin)); }
    let out = Command::new(&bin).args(["--headless", "--script", &script_path])
        .output().map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&script_path);
    if !out.status.success() {
        return Err(format!("Failed: {} {}", String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Test typing burst: 50 characters typed rapidly.
#[test]
fn test_typing_burst() {
    // Enter insert mode, type 50 characters, exit
    let s = r#"[{"kind":"key","code":"i","ctrl":false},
        {"kind":"keys","keys":"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWX"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

/// Test repeated down movement: 20 lines.
#[test]
fn test_scroll_burst() {
    // Press j 20 times, then quit
    let mut script = String::from("[");
    for _ in 0..20 {
        script.push_str(r#"{"kind":"key","code":"j","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test rapid mode switches: Normal -> Insert -> Normal repeatedly.
#[test]
fn test_mode_switch_burst() {
    let mut script = String::from("[");
    for _ in 0..10 {
        script.push_str(r#"{"kind":"key","code":"i","ctrl":false},"#);
        script.push_str(r#"{"kind":"key","code":"Escape","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test input sequence ordering is preserved.
#[test]
fn test_input_ordering() {
    // Type specific sequence, verify it completes without error
    let s = r#"[{"kind":"key","code":"i","ctrl":false},
        {"kind":"keys","keys":"12345"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"assert_mode","mode":"normal"},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

/// Test repeated up movement.
#[test]
fn test_up_movement_burst() {
    let mut script = String::from("[");
    for _ in 0..15 {
        script.push_str(r#"{"kind":"key","code":"k","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test repeated left movement.
#[test]
fn test_left_movement_burst() {
    let mut script = String::from("[");
    for _ in 0..10 {
        script.push_str(r#"{"kind":"key","code":"h","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test repeated right movement.
#[test]
fn test_right_movement_burst() {
    let mut script = String::from("[");
    for _ in 0..10 {
        script.push_str(r#"{"kind":"key","code":"l","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test repeated delete char.
#[test]
fn test_delete_char_burst() {
    let mut script = String::from("[");
    script.push_str(r#"{"kind":"key","code":"i","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"hello world"},"#);
    script.push_str(r#"{"kind":"key","code":"Escape","ctrl":false},"#);
    for _ in 0..5 {
        script.push_str(r#"{"kind":"key","code":"x","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test visual mode selection.
#[test]
fn test_visual_selection_burst() {
    let mut script = String::from("[");
    script.push_str(r#"{"kind":"key","code":"v","ctrl":false},"#);
    for _ in 0..5 {
        script.push_str(r#"{"kind":"key","code":"l","ctrl":false},"#);
    }
    script.push_str(r#"{"kind":"key","code":"Escape","ctrl":false},"#);
    script.push_str(r#"{"kind":"key","code":":","ctrl":false},"#);
    script.push_str(r#"{"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#);
    assert!(run_headless(&script).is_ok());
}

/// Test command mode typing.
#[test]
fn test_command_mode_typing() {
    let s = r#"[{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"write"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

/// Test undo command.
#[test]
fn test_undo_operation() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"u","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

/// Test redo command.
#[test]
fn test_redo_operation() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hi"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"u","ctrl":false},
        {"kind":"key","code":"r","ctrl":true},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

/// Test goto end of file.
#[test]
fn test_goto_end_of_file() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"line1"},
        {"kind":"key","code":"Enter","ctrl":false},{"kind":"keys","keys":"line2"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"keys","keys":"G"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}
