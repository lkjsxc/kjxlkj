//! E2E tests for mode transitions.

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
    let script_path = format!("/tmp/kjxlkj_e2e_modes_{}_{}.json", pid, id);
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
fn test_headless_mode_starts() {
    let s = r#"[{"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_insert_mode() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_command_mode() {
    let s = r#"[{"kind":"key","code":":","ctrl":false},{"kind":"assert_mode","mode":"command"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_visual_mode() {
    let s = r#"[{"kind":"key","code":"v","ctrl":false},{"kind":"assert_mode","mode":"visual"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_visual_line_mode() {
    let s = r#"[{"kind":"keys","keys":"V"},{"kind":"assert_mode","mode":"visual_line"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_replace_mode() {
    let s = r#"[{"kind":"keys","keys":"R"},{"kind":"assert_mode","mode":"replace"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_visual_block_mode() {
    let s = r#"[{"kind":"key","code":"v","ctrl":true},{"kind":"assert_mode","mode":"visual_block"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_append_mode() {
    let s = r#"[{"kind":"key","code":"a","ctrl":false},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_append_end_of_line() {
    let s = r#"[{"kind":"keys","keys":"A"},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_insert_start_of_line() {
    let s = r#"[{"kind":"keys","keys":"I"},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
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
    let s = r#"[{"kind":"keys","keys":"O"},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_visual_to_normal() {
    let s = r#"[{"kind":"keys","keys":"v"},{"kind":"assert_mode","mode":"visual"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"assert_mode","mode":"normal"},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_insert_then_normal() {
    let s = r#"[{"kind":"keys","keys":"i"},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"assert_mode","mode":"normal"},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_command_enter() {
    let s = r#"[{"kind":"key","code":":","ctrl":false},{"kind":"assert_mode","mode":"command"},
        {"kind":"key","code":"Enter","ctrl":false},{"kind":"assert_mode","mode":"normal"},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_multiple_mode_switches() {
    let s = r#"[{"kind":"keys","keys":"i"},{"kind":"assert_mode","mode":"insert"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"keys","keys":"v"},{"kind":"assert_mode","mode":"visual"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_insert_and_escape() {
    let s = r#"[{"kind":"keys","keys":"i"},{"kind":"keys","keys":"test"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"assert_mode","mode":"normal"},
        {"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q!"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}
