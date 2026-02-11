use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn exp_05r_long_label_wrap_baseline_keeps_bounds_and_focus_identity() {
    let line = "path_segment_with_badge_漢字_".repeat(512);
    let output = run_raw_script(
        b":Explorer\r\x17w\x17W\x17w",
        &[
            ("KJXLKJ_INITIAL_LINE", line.as_str()),
            ("KJXLKJ_START_CURSOR", "0"),
            ("KJXLKJ_ROWS", "12"),
            ("KJXLKJ_COLS", "24"),
        ],
    );
    assert!(
        output.contains("resolved_action=WinSplitExplorer")
            && !output.contains("geometry_ok=false")
            && !output.contains("render_bounds_ok=false"),
        "EXP-05R expected stable explorer routing under long-label wrap pressure. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Explorer") && output.contains("FINAL"),
        "EXP-05R expected explorer focus to remain observable through churn. Output:\n{output}"
    );
}

#[test]
fn exp_06r_refresh_drift_churn_baseline_keeps_state_consistent() {
    let output = run_raw_script(
        b":Explorer\rq:Explorer\r:ExplorerClose\r:ExplorerReveal\r",
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    );
    assert!(
        count_occurrences(&output, "resolved_action=WinSplitExplorer") >= 2
            && output.contains("resolved_action=WinClose"),
        "EXP-06R expected close/reopen/reveal routes to stay reachable in drift churn. Output:\n{output}"
    );
    assert!(
        !output.contains("geometry_ok=false") && !output.contains("render_bounds_ok=false"),
        "EXP-06R expected geometry/render invariants to hold under explorer churn. Output:\n{output}"
    );
}

#[test]
fn bd_race_01_terminal_flood_explorer_refresh_resize_churn_is_deterministic() {
    let first = run_bd_race_script();
    let second = run_bd_race_script();

    assert!(
        first.contains("resolved_action=TerminalExitToNormal")
            && first.contains("resolved_action=WinSplitExplorer")
            && !first.contains("geometry_ok=false")
            && !first.contains("render_bounds_ok=false"),
        "BD-RACE-01 expected stable mixed churn semantics in first run. Output:\n{first}"
    );
    assert!(
        second.contains("resolved_action=TerminalExitToNormal")
            && second.contains("resolved_action=WinSplitExplorer")
            && !second.contains("geometry_ok=false")
            && !second.contains("render_bounds_ok=false"),
        "BD-RACE-01 expected stable mixed churn semantics in second run. Output:\n{second}"
    );

    let first_wrap = extract_final_value(&first, "wrap_sig=").expect("first wrap_sig should exist");
    let second_wrap =
        extract_final_value(&second, "wrap_sig=").expect("second wrap_sig should exist");
    let first_session =
        extract_final_value(&first, "window_session=").expect("first window_session should exist");
    let second_session = extract_final_value(&second, "window_session=")
        .expect("second window_session should exist");
    assert_eq!(first_wrap, second_wrap);
    assert_eq!(first_session, second_session);
}

fn run_bd_race_script() -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let line = "abc漢字".repeat(64);
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[
            ("KJXLKJ_INITIAL_LINE", line.as_str()),
            ("KJXLKJ_START_CURSOR", "0"),
        ],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session
        .send_raw(b":terminal\ri")
        .expect("terminal insert sequence should send");
    let flood = "漢字かなカナ".repeat(16);
    session
        .send_raw(flood.as_bytes())
        .expect("flood payload should send");
    session
        .send_raw(b"\x1C\x0E:Explorer\r\x17w\x17W\x17h\x17l")
        .expect("exit and mixed-route sequence should send");
    for (cols, rows) in [(80_u16, 20_u16), (120, 40), (60, 16), (100, 30)] {
        session.resize(cols, rows).expect("resize should succeed");
    }
    session.quit().expect("quit should succeed")
}

fn run_raw_script(script: &[u8], env: &[(&str, &str)]) -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(&binary, 100, 30, env).expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    session.send_raw(script).expect("script should send");
    session.quit().expect("quit should succeed")
}

fn extract_final_value(output: &str, marker: &str) -> Option<String> {
    let line = output.lines().find(|line| line.starts_with("FINAL "))?;
    let pos = line.find(marker)?;
    let suffix = &line[pos + marker.len()..];
    Some(suffix.split_whitespace().next()?.to_string())
}

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}
