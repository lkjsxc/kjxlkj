//! E2E tests for editing operations.

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
    let script_path = format!("/tmp/kjxlkj_e2e_editing_{}_{}.json", pid, id);
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

#[test]
fn test_headless_append_mode() {
    let s = r#"[{"kind":"key","code":"a","ctrl":false},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_open_line_below() {
    let s = r#"[{"kind":"key","code":"o","ctrl":false},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_open_line_above() {
    let s = r#"[{"kind":"key","code":"O","shift":true},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_text_insert_delete() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"x","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_insert_newline() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"line1"},
        {"kind":"key","code":"Enter","ctrl":false},{"kind":"keys","keys":"line2"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_cursor_start_of_line() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_delete_word() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello world"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"d","ctrl":false},
        {"kind":"key","code":"w","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_yank_line() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"test"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"keys","keys":"yy"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_change_word() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello world"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"c","ctrl":false},{"kind":"key","code":"w","ctrl":false},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_replace_char() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"abc"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"r","ctrl":false},{"kind":"key","code":"x","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_delete_to_end() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello world"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"keys","keys":"D"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_change_to_end() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello world"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"keys","keys":"C"},{"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_join_lines() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Enter","ctrl":false},{"kind":"keys","keys":"world"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"k","ctrl":false},
        {"kind":"keys","keys":"J"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_indent_line() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"keys","keys":">>"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}
