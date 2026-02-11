use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn perf_01r_profile_emits_required_metrics_for_input_burst() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        80,
        20,
        &[("KJXLKJ_PROFILE", "1"), ("KJXLKJ_INITIAL_LINE", "abc")],
    )
    .expect("PTY session should spawn");
    session
        .send_raw(b"iabc\x1Bq")
        .expect("burst input sequence should send");
    let output = session
        .wait_for_pattern("FINAL", Duration::from_secs(2))
        .expect("PERF-01R expected FINAL summary");
    let profile = extract_profile_line(&output).expect("PERF-01R expected PROFILE line");

    assert!(
        profile.contains("events_processed=")
            && profile.contains("core_update_count=")
            && profile.contains("snapshot_duration_ns=")
            && profile.contains("render_duration_ns=")
            && profile.contains("snapshot_materialized_lines_max="),
        "PERF-01R missing required profiling metrics. Output:\n{output}"
    );
    assert!(
        metric_u64(profile, "events_processed=").unwrap_or(0) >= 1,
        "PERF-01R expected at least one processed event. Output:\n{output}"
    );
}

#[test]
fn perf_02r_large_file_probe_reports_viewport_bounded_materialization() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let large = (0..512)
        .map(|idx| format!("line_{idx}"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut session = PtySession::spawn(
        &binary,
        30,
        8,
        &[
            ("KJXLKJ_PROFILE", "1"),
            ("KJXLKJ_INITIAL_LINE", large.as_str()),
            ("KJXLKJ_ROWS", "8"),
            ("KJXLKJ_COLS", "30"),
            ("KJXLKJ_START_CURSOR", "0"),
        ],
    )
    .expect("PTY session should spawn");
    session.send_raw(b"q").expect("quit key should send");
    let output = session
        .wait_for_pattern("FINAL", Duration::from_secs(2))
        .expect("PERF-02R expected FINAL summary");
    let profile = extract_profile_line(&output).expect("PERF-02R expected PROFILE line");

    assert!(
        profile.contains("materialized_bound_ok=true")
            && profile.contains("large_file_probe_ok=true"),
        "PERF-02R expected viewport-bounded materialization probes. Output:\n{output}"
    );
    assert!(
        metric_u64(profile, "snapshot_materialized_lines_max=").unwrap_or(u64::MAX) <= 9,
        "PERF-02R expected materialized lines <= rows + margin. Output:\n{output}"
    );
}

#[test]
fn perf_03r_idle_probe_reports_no_busy_loop_without_input() {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        80,
        20,
        &[("KJXLKJ_PROFILE", "1"), ("KJXLKJ_INITIAL_LINE", "abc")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(250));
    session.send_raw(b"q").expect("quit key should send");
    let output = session
        .wait_for_pattern("FINAL", Duration::from_secs(2))
        .expect("PERF-03R expected FINAL summary");
    let profile = extract_profile_line(&output).expect("PERF-03R expected PROFILE line");

    assert!(
        profile.contains("idle_busy_loop=false") && profile.contains("idle_redraw_count=0"),
        "PERF-03R expected idle probe to report no busy loop. Output:\n{output}"
    );
}

fn extract_profile_line(output: &str) -> Option<&str> {
    output.lines().find(|line| line.starts_with("PROFILE "))
}

fn metric_u64(line: &str, marker: &str) -> Option<u64> {
    let pos = line.find(marker)?;
    let suffix = &line[pos + marker.len()..];
    let token = suffix.split_whitespace().next()?;
    token.parse::<u64>().ok()
}
