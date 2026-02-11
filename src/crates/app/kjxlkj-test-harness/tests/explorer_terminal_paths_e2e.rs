use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn exp_01r_colon_explorer_opens_explorer_and_focuses_it() {
    let output = run_raw_script(b":Explorer\r");
    assert!(
        output.contains("resolved_action=WinSplitExplorer"),
        "EXP-01R expected :Explorer route to split explorer. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Explorer"),
        "EXP-01R expected explorer focus in trace. Output:\n{output}"
    );
}

#[test]
fn exp_02r_leader_e_and_upper_e_routes_are_reachable() {
    let output = run_raw_script(b" e E");
    assert!(
        count_occurrences(&output, "resolved_action=WinSplitExplorer") >= 2,
        "EXP-02R expected <leader>e and <leader>E routes to trigger explorer actions. Output:\n{output}"
    );
}

#[test]
fn exp_03r_open_targets_current_vertical_and_horizontal_are_reachable() {
    let current = run_raw_script(b":Explorer\r\r");
    assert!(
        current.contains("resolved_action=ExplorerOpenCurrent"),
        "EXP-03R expected Enter open-target route. Output:\n{current}"
    );
    assert!(
        current.contains("focused_window_type=Buffer"),
        "EXP-03R current-target route should focus buffer window. Output:\n{current}"
    );

    let vertical = run_raw_script(b":Explorer\rv");
    assert!(
        vertical.contains("resolved_action=ExplorerOpenVertical"),
        "EXP-03R expected v open-target route. Output:\n{vertical}"
    );
    assert!(
        vertical.contains("focused_window_type=Buffer"),
        "EXP-03R vertical-target route should focus buffer window. Output:\n{vertical}"
    );

    let horizontal = run_raw_script(b":Explorer\rs");
    assert!(
        horizontal.contains("resolved_action=ExplorerOpenHorizontal"),
        "EXP-03R expected s open-target route. Output:\n{horizontal}"
    );
    assert!(
        horizontal.contains("focused_window_type=Buffer"),
        "EXP-03R horizontal-target route should focus buffer window. Output:\n{horizontal}"
    );
}

#[test]
fn term_01r_colon_terminal_opens_terminal_and_focuses_it() {
    let output = run_raw_script(b":terminal\r");
    assert!(
        output.contains("resolved_action=WinSplitTerminal"),
        "TERM-01R expected :terminal route to split terminal. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Terminal"),
        "TERM-01R expected terminal focus in trace. Output:\n{output}"
    );
}

#[test]
fn term_02r_leader_terminal_routes_are_reachable() {
    let output = run_raw_script(b" t\r th tv");
    assert!(
        output.contains("resolved_action=WinSplitTerminal"),
        "TERM-02R expected <leader>t route to split terminal. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinSplitTerminalHorizontal"),
        "TERM-02R expected <leader>th route to split terminal horizontally. Output:\n{output}"
    );
    assert!(
        count_occurrences(&output, "resolved_action=WinSplitTerminal") >= 2,
        "TERM-02R expected <leader>t and <leader>tv to route terminal actions. Output:\n{output}"
    );
}

#[test]
fn term_03r_ctrl_w_navigation_remains_mixed_window_consistent() {
    let output = run_raw_script(b":terminal\r\x17h\x17l");
    assert!(
        output.contains("focused_window_type=Terminal"),
        "TERM-03R expected terminal focus in trace. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Buffer"),
        "TERM-03R expected buffer focus in trace. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinFocusLeft")
            && output.contains("resolved_action=WinFocusRight"),
        "TERM-03R expected left/right Ctrl-w navigation actions. Output:\n{output}"
    );
}

#[test]
fn term_04r_resize_with_terminal_focus_keeps_geometry_invariants() {
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
        .send_raw(b":terminal\r")
        .expect("terminal command should send");
    session
        .resize(80, 20)
        .expect("resize to 80x20 should succeed");
    session
        .resize(120, 40)
        .expect("resize to 120x40 should succeed");
    session
        .resize(60, 16)
        .expect("resize to 60x16 should succeed");
    let output = session.quit().expect("quit should succeed");
    assert!(
        !output.contains("geometry_ok=false"),
        "TERM-04R expected geometry invariants to stay true during terminal resizes. Output:\n{output}"
    );
    assert!(
        output.contains("focused_window_type=Terminal"),
        "TERM-04R expected terminal focus to stay observable in trace. Output:\n{output}"
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

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}
