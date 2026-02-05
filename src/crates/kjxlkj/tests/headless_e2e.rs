use std::process::Command;

use tempfile::NamedTempFile;

fn run_headless(script_json: &str, file: Option<&std::path::Path>) -> std::process::Output {
    let mut script = NamedTempFile::new().expect("create temp script");
    std::io::Write::write_all(&mut script, script_json.as_bytes()).expect("write script");
    let script_path = script.path().to_path_buf();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kjxlkj"));
    cmd.arg("--headless").arg("--script").arg(&script_path);
    if let Some(file) = file {
        cmd.arg(file);
    }

    cmd.output().expect("run kjxlkj headless")
}

#[test]
fn headless_mode_starts_and_quits() {
    let script = r#"
[
  {"kind":"keys","keys":":q"},
  {"kind":"key","code":"Enter"},
  {"kind":"assert_mode","mode":"normal"}
]
"#;

    let out = run_headless(script, None);
    assert!(
        out.status.success(),
        "stderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn headless_insert_and_assert_line() {
    let script = r#"
[
  {"kind":"assert_mode","mode":"normal"},
  {"kind":"key","code":"i"},
  {"kind":"assert_mode","mode":"insert"},
  {"kind":"keys","keys":"abc"},
  {"kind":"key","code":"Esc"},
  {"kind":"assert_mode","mode":"normal"},
  {"kind":"assert_line","line":0,"content":"abc"}
]
"#;

    let out = run_headless(script, None);
    assert!(
        out.status.success(),
        "stderr:\n{}\nstdout:\n{}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
}
