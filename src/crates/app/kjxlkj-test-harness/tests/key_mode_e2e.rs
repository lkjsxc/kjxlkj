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
    session
        .send_raw(b"A")
        .expect("sending Shift+a byte should succeed");
    let output = session
        .wait_for_pattern("normalized_key=A", Duration::from_secs(2))
        .expect("KEY-TRACE-01 expected normalized A trace");
    assert!(
        output.contains("resolved_action=EnterInsertAtEol"),
        "KEY-TRACE-01 expected dispatch to A semantics. Output:\n{output}"
    );
    session.send_raw(b"\x1Bq").expect("quit should succeed");
    let _ = session.wait_for_pattern("FINAL", Duration::from_secs(2));
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
    assert!(
        output.contains("line=abcZ"),
        "WR-01R expected append-at-EOL result line=abcZ. Output:\n{output}"
    );
}
