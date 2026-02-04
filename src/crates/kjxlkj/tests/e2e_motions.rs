//! E2E tests for cursor motions.

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
    let script_path = format!("/tmp/kjxlkj_e2e_motions_{}_{}.json", pid, id);
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
fn test_headless_cursor_movement() {
    let s = r#"[{"kind":"key","code":"j","ctrl":false},{"kind":"key","code":"k","ctrl":false},
        {"kind":"key","code":"l","ctrl":false},{"kind":"key","code":"h","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_word_motions() {
    let s = r#"[{"kind":"key","code":"w","ctrl":false},{"kind":"key","code":"b","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_line_motions() {
    let s = r#"[{"kind":"key","code":"$","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"^","ctrl":false},{"kind":"key","code":":","ctrl":false},
        {"kind":"keys","keys":"q"},{"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_file_motions() {
    let s = r#"[{"kind":"keys","keys":"G"},{"kind":"keys","keys":"gg"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_word_end_motion() {
    let s = r#"[{"kind":"key","code":"e","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_paragraph_motion() {
    let s = r#"[{"kind":"keys","keys":"}"},{"kind":"keys","keys":"{"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_screen_motion() {
    let s = r#"[{"kind":"keys","keys":"H"},{"kind":"keys","keys":"M"},{"kind":"keys","keys":"L"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_find_char_motion() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"f","ctrl":false},{"kind":"key","code":"l","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_percent_motion() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"()"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"%","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_word_motion_count() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"one two three four"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"2","ctrl":false},{"kind":"key","code":"w","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_till_char_motion() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"t","ctrl":false},{"kind":"key","code":"l","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_first_non_blank() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"   hello"},
        {"kind":"key","code":"Escape","ctrl":false},{"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":"^","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_goto_line_start() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"key","code":"0","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_backward_find() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"keys","keys":"F"},{"kind":"key","code":"e","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_backward_till() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"hello"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"keys","keys":"T"},{"kind":"key","code":"e","ctrl":false},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}

#[test]
fn test_headless_goto_first_line() {
    let s = r#"[{"kind":"key","code":"i","ctrl":false},{"kind":"keys","keys":"line1"},
        {"kind":"key","code":"Enter","ctrl":false},{"kind":"keys","keys":"line2"},
        {"kind":"key","code":"Escape","ctrl":false},
        {"kind":"keys","keys":"gg"},
        {"kind":"key","code":":","ctrl":false},{"kind":"keys","keys":"q!"},
        {"kind":"key","code":"Enter","ctrl":false}]"#;
    assert!(run_headless(s).is_ok());
}
