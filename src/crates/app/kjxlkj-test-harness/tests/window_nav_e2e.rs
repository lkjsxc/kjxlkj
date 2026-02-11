use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn win_01r_split_close_only_keeps_valid_focus() {
    let output = run_window_script(b"svco");
    assert!(
        output.contains("resolved_action=WinSplitHorizontal"),
        "WIN-01R expected split trace. Output:\n{output}"
    );
    assert!(
        output.contains("resolved_action=WinOnly"),
        "WIN-01R expected only trace. Output:\n{output}"
    );
    assert!(
        output.contains("FINAL mode=Normal"),
        "WIN-01R expected normal-mode final summary. Output:\n{output}"
    );
}

#[test]
fn winnav_06r_replay_is_deterministic() {
    let script = b"svhlwWptbco";
    let run_one = focus_trace(&run_window_script(script));
    let run_two = focus_trace(&run_window_script(script));
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
    let final_output = session.quit().expect("quit should succeed");
    session
        .wait_for_pattern("FINAL", Duration::from_secs(2))
        .expect("final output should be captured");
    final_output
}

fn focus_trace(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|line| {
            line.contains("resolved_action=Win")
                && !line.contains("resolved_action=WinIgnore")
                && !line.contains("resolved_action=Ignore")
        })
        .map(|line| line.to_string())
        .collect()
}
