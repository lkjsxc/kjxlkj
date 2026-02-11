use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn key_trace_01_shift_a_normalizes_to_upper_a_before_dispatch() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    session.resize(80, 20).expect("resize should succeed");
    session
        .send_symbolic_key("A")
        .expect("sending Shift+a key should succeed");
    let output = session
        .wait_for_pattern("normalized_key=A", Duration::from_secs(2))
        .expect("KEY-TRACE-01 expected normalized A trace");
    assert!(
        output.contains("resolved_action=EnterInsertAtEol"),
        "KEY-TRACE-01 expected dispatch to A semantics. Output:\n{output}"
    );
    let final_frame = session.quit().expect("quit should succeed");
    assert!(
        final_frame.contains("FINAL"),
        "final frame should contain FINAL summary. Output:\n{final_frame}"
    );
}

#[test]
fn wr_01r_shift_a_has_append_at_eol_semantics() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    session
        .send_raw(b"AZ\x1Bq")
        .expect("input sequence should succeed");
    let output = session
        .wait_for_pattern("FINAL", Duration::from_secs(2))
        .expect("WR-01R expected FINAL summary");
    let frame = session.capture_frame();
    assert!(
        output.contains("line=abcZ"),
        "WR-01R expected append-at-EOL result line=abcZ. Output:\n{output}"
    );
    assert!(
        frame.contains("normalized_key=A"),
        "WR-01R expected trace to include normalized A. Output:\n{frame}"
    );
}

#[test]
fn key_trace_03_explorer_command_and_leader_routes_emit_trace_actions() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw(b":Explorer\r e")
        .expect("explorer command and leader sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("normalized_key=:"),
        "KEY-TRACE-03 expected command prefix in trace. Output:\n{output}"
    );
    assert!(
        count_occurrences(&output, "resolved_action=WinSplitExplorer") >= 2,
        "KEY-TRACE-03 expected command and leader explorer actions. Output:\n{output}"
    );
}

#[test]
fn key_trace_04_terminal_command_and_leader_routes_emit_trace_actions() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw(b":terminal\r t\r")
        .expect("terminal command and leader sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("normalized_key=:"),
        "KEY-TRACE-04 expected command prefix in trace. Output:\n{output}"
    );
    assert!(
        count_occurrences(&output, "resolved_action=WinSplitTerminal") >= 2,
        "KEY-TRACE-04 expected command and leader terminal actions. Output:\n{output}"
    );
}

#[test]
fn key_trace_05_recent_events_capture_raw_and_normalized_paths() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw(b":Explorer\r")
        .expect("explorer command sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("recent_events=")
            && output.contains("0x3A::->Ignore")
            && output.contains("0x0D:Enter->WinSplitExplorer"),
        "KEY-TRACE-05 expected raw byte + normalized key + resolved action tuples. Output:\n{output}"
    );
}

#[test]
fn key_trace_06_multibyte_utf8_input_decodes_to_single_normalized_char() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw("あ".as_bytes())
        .expect("multibyte UTF-8 sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("normalized_key=あ"),
        "KEY-TRACE-06 expected decoded UTF-8 normalized key entry. Output:\n{output}"
    );
    assert!(
        !output.contains("normalized_key=Unknown(227)")
            && !output.contains("normalized_key=Unknown(129)")
            && !output.contains("normalized_key=Unknown(130)"),
        "KEY-TRACE-06 expected no per-byte unknown spam for valid UTF-8. Output:\n{output}"
    );
}

#[test]
fn key_trace_07_trace_includes_layout_summary_and_frame_excerpt_fields() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw(b":Explorer\r")
        .expect("explorer command sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("layout_summary=") && output.contains("frame_excerpt="),
        "KEY-TRACE-07 expected layout and frame excerpt fields in trace diagnostics. Output:\n{output}"
    );
}

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}
