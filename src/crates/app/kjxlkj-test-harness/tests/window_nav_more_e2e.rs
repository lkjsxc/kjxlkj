use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn win_02r_directional_focus_trace_is_deterministic() {
    let output = run_window_script(b"svhjl");
    assert!(
        output.contains("resolved_action=WinFocusLeft"),
        "WIN-02R expected left focus action. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinFocusDown"),
        "WIN-02R expected down focus action. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinFocusRight"),
        "WIN-02R expected right focus action. Output:\n{output}"
    );
    assert!(
        !output.contains("geometry_ok=false"),
        "WIN-02R requires geometry invariants to hold. Output:\n{output}"
    );
}

#[test]
fn win_03r_mixed_window_navigation_visits_all_window_types() {
    let output = run_window_script(b"ETwlpb");
    assert!(
        output.contains("focused_window_type=Buffer"),
        "WIN-03R expected buffer windows in trace. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Explorer"),
        "WIN-03R expected explorer windows in trace. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Terminal"),
        "WIN-03R expected terminal windows in trace. Output:\n{output}"
    );
}

#[test]
fn win_04r_resize_storm_keeps_geometry_invariants_true() {
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
        .send_raw(b"\x17s\x17v\x17E")
        .expect("script should send");
    session
        .resize(80, 20)
        .expect("resize to 80x20 should succeed");
    session
        .resize(120, 40)
        .expect("resize to 120x40 should succeed");
    session
        .resize(50, 16)
        .expect("resize to 50x16 should succeed");
    session
        .send_raw(b"\x17w\x17W\x17t\x17b")
        .expect("navigation should send");
    let output = session.quit().expect("quit should succeed");
    assert!(
        !output.contains("geometry_ok=false"),
        "WIN-04R expected geometry invariants to remain true. Output:\n{output}"
    );
}

#[test]
fn winnav_01r_golden_cycle_order_is_replay_stable() {
    let script = b"svEwWptb";
    let run_one = extract_focus_ids(&run_window_script(script));
    let run_two = extract_focus_ids(&run_window_script(script));
    assert_eq!(run_one, run_two);
}

#[test]
fn winnav_02r_interleaved_directional_and_cyclic_is_replay_stable() {
    let script = b"svEThwlWjpkb";
    let run_one = extract_focus_ids(&run_window_script(script));
    let run_two = extract_focus_ids(&run_window_script(script));
    assert_eq!(run_one, run_two);
}

#[test]
fn winnav_03r_previous_focus_after_churn_remains_valid() {
    let output = run_window_script(b"svwpccp");
    assert!(
        output.contains("resolved_action=WinPrevious"),
        "WINNAV-03R expected previous-focus command. Output:\n{output}"
    );
    assert!(
        output.contains("FINAL"),
        "WINNAV-03R expected deterministic final summary. Output:\n{output}"
    );
}

#[test]
fn winnav_04r_top_and_bottom_targets_are_replay_stable() {
    let script = b"svEtb";
    let run_one = extract_focus_ids(&run_window_script(script));
    let run_two = extract_focus_ids(&run_window_script(script));
    assert_eq!(run_one, run_two);
}

fn run_window_script(commands: &[u8]) -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut session = PtySession::spawn(
        &binary,
        100,
        30,
        &[("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")],
    )
    .expect("PTY session should spawn");
    std::thread::sleep(Duration::from_millis(120));
    let mut bytes = Vec::with_capacity(commands.len() * 2);
    for command in commands {
        bytes.push(0x17);
        bytes.push(*command);
    }
    session
        .send_raw(&bytes)
        .expect("window command script should send");
    session.quit().expect("quit should succeed")
}

fn extract_focus_ids(output: &str) -> Vec<u64> {
    output
        .lines()
        .filter_map(|line| {
            let marker = "focused_window_id=";
            let pos = line.find(marker)?;
            let suffix = &line[pos + marker.len()..];
            let id_text = suffix.split_whitespace().next()?;
            id_text.parse::<u64>().ok()
        })
        .collect()
}
