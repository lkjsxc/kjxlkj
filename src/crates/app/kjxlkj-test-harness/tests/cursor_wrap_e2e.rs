use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn wrap_11r_long_ascii_line_stays_in_bounds() {
    let line = "a".repeat(10_000);
    let output = run_script(
        b"",
        &[
            ("KJXLKJ_INITIAL_LINE", line.as_str()),
            ("KJXLKJ_START_CURSOR", "0"),
        ],
    );
    assert!(
        !output.contains("render_bounds_ok=false"),
        "WRAP-11R expected in-bounds rendering for long ASCII line. Output:\n{output}"
    );
}

#[test]
fn wrap_12r_long_cjk_line_stays_in_bounds_without_continuation_target() {
    let line = "漢".repeat(10_000);
    let output = run_script(
        b"",
        &[
            ("KJXLKJ_INITIAL_LINE", line.as_str()),
            ("KJXLKJ_START_CURSOR", "0"),
        ],
    );
    assert!(
        !output.contains("render_bounds_ok=false"),
        "WRAP-12R expected in-bounds rendering for long CJK line. Output:\n{output}"
    );
    assert!(
        !output.contains("cursor_continuation=true"),
        "WRAP-12R expected cursor to avoid continuation cells. Output:\n{output}"
    );
}

#[test]
fn wrap_13r_same_input_keeps_wrap_signature_stable() {
    let line = "abc漢字".repeat(512);
    let env = &[
        ("KJXLKJ_INITIAL_LINE", line.as_str()),
        ("KJXLKJ_START_CURSOR", "0"),
    ];
    let first = run_script(b"", env);
    let second = run_script(b"", env);
    let first_sig = extract_final_value(&first, "wrap_sig=").expect("first wrap_sig should exist");
    let second_sig =
        extract_final_value(&second, "wrap_sig=").expect("second wrap_sig should exist");
    assert_eq!(first_sig, second_sig);
}

#[test]
fn cur_07r_cursor_stays_visible_across_mode_and_focus_churn() {
    let output = run_script(
        b"iZ\x1B\x17s\x17v\x17w\x17W",
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "1")],
    );
    assert!(
        !output.contains("cursor_visible=false"),
        "CUR-07R expected cursor to stay visible through churn. Output:\n{output}"
    );
}

#[test]
fn cur_09r_cursor_never_targets_continuation_cell() {
    let output = run_script(
        b"",
        &[
            ("KJXLKJ_INITIAL_LINE", "a漢b"),
            ("KJXLKJ_START_CURSOR", "1"),
        ],
    );
    assert!(
        !output.contains("cursor_continuation=true"),
        "CUR-09R expected cursor to never target continuation cell. Output:\n{output}"
    );
}

#[test]
fn cur_11r_focus_switching_keeps_cursor_visibility_deterministic() {
    let output = run_script(
        b"\x17s\x17v\x17w\x17W\x17t\x17b",
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    );
    assert!(
        !output.contains("cursor_visible=false"),
        "CUR-11R expected cursor visibility under rapid focus switching. Output:\n{output}"
    );
}

fn run_script(commands: &[u8], extra_env: &[(&str, &str)]) -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut env = vec![("KJXLKJ_ROWS", "20"), ("KJXLKJ_COLS", "80")];
    env.extend_from_slice(extra_env);
    let mut session = PtySession::spawn(&binary, 100, 30, &env).expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    if !commands.is_empty() {
        session.send_raw(commands).expect("script should send");
    }
    session.quit().expect("quit should succeed")
}

fn extract_final_value(output: &str, marker: &str) -> Option<String> {
    let line = output.lines().find(|line| line.starts_with("FINAL "))?;
    let pos = line.find(marker)?;
    let suffix = &line[pos + marker.len()..];
    Some(suffix.split_whitespace().next()?.to_string())
}
