use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn exp_04r_mixed_ctrl_w_routes_keep_focus_transitions_valid() {
    let output = run_raw_script(b":Explorer\r:terminal\r\x17h\x17l\x17w");
    assert!(
        output.contains("focused_window_type=Buffer"),
        "EXP-04R expected buffer focus to appear in mixed routing trace. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Explorer"),
        "EXP-04R expected explorer focus to appear in mixed routing trace. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Terminal"),
        "EXP-04R expected terminal focus to appear in mixed routing trace. Output:\n{output}"
    );
    assert!(
        !output.contains("geometry_ok=false"),
        "EXP-04R expected geometry invariants to hold in mixed routing. Output:\n{output}"
    );
}

#[test]
fn term_05r_close_terminal_leaf_keeps_focus_and_geometry_valid() {
    let output = run_raw_script(b":terminal\r\x17c");
    assert!(
        output.contains("resolved_action=WinClose"),
        "TERM-05R expected close action on focused terminal leaf. Output:\n{output}"
    );
    assert!(
        output.contains("FINAL mode=Normal")
            && output.contains("focused_window_type=Buffer")
            && output.contains("geometry_ok=true"),
        "TERM-05R expected stable post-close final state. Output:\n{output}"
    );
}

#[test]
fn term_06r_flood_churn_baseline_stays_responsive() {
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
        .send_raw(b":terminal\ri")
        .expect("terminal and insert sequence should send");
    let flood = vec![b'x'; 4096];
    session.send_raw(&flood).expect("flood bytes should send");
    session
        .send_raw(b"\x1C\x0E\x17h\x17l")
        .expect("exit + mixed navigation sequence should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("resolved_action=TerminalExitToNormal"),
        "TERM-06R expected explicit terminal-insert exit after flood. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinFocusLeft")
            && output.contains("resolved_action=WinFocusRight"),
        "TERM-06R expected responsive Ctrl-w navigation after flood. Output:\n{output}"
    );
}

#[test]
fn term_07r_cjk_flood_baseline_remains_stable() {
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
        .send_raw(b":terminal\ri")
        .expect("terminal and insert sequence should send");
    let cjk_flood = "漢字かなカナ".repeat(512);
    session
        .send_raw(cjk_flood.as_bytes())
        .expect("cjk flood bytes should send");
    session
        .send_raw(b"\x1C\x0E")
        .expect("terminal exit chord should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        output.contains("resolved_action=TerminalExitToNormal")
            && output.contains("geometry_ok=true"),
        "TERM-07R expected stable terminal exit and geometry after CJK flood. Output:\n{output}"
    );
}

fn run_raw_script(script: &[u8]) -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session.send_raw(script).expect("script should send");
    session.quit().expect("quit should succeed")
}
