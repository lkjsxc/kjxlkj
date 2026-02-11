use std::time::Duration;

use kjxlkj_test_harness::{ensure_kjxlkj_built, PtySession};

#[test]
fn winnav_05r_terminal_insert_and_post_exit_navigation_match() {
    let before_exit = run_terminal_mode_script(false);
    let after_exit = run_terminal_mode_script(true);

    let before_nav = extract_action_focus_pairs(&before_exit, &["WinFocusLeft", "WinFocusRight"]);
    let after_nav = extract_action_focus_pairs(&after_exit, &["WinFocusLeft", "WinFocusRight"]);
    assert_eq!(before_nav, after_nav);

    let left_line = find_action_line(&before_exit, "WinFocusLeft")
        .expect("WINNAV-05R expected left navigation action before terminal exit");
    assert!(
        left_line.contains("mode_before=TerminalInsert"),
        "WINNAV-05R expected terminal-insert navigation before exit chord. Output:\n{before_exit}"
    );
    assert!(
        after_exit.contains("resolved_action=TerminalExitToNormal"),
        "WINNAV-05R expected Ctrl-\\\\ Ctrl-n exit action. Output:\n{after_exit}"
    );
}

#[test]
fn win_05r_session_roundtrip_restores_tree_and_focus() {
    let first = run_window_script_with_env(b"sETwWtb", &[]);
    let first_focus = extract_final_u64(&first, "focused_window_id=")
        .expect("WIN-05R first run should expose final focused_window_id");
    let first_session = extract_final_value(&first, "window_session=")
        .expect("WIN-05R first run should expose window_session");

    let second =
        run_window_script_with_env(b"", &[("KJXLKJ_WINDOW_SESSION", first_session.as_str())]);
    let second_focus = extract_final_u64(&second, "focused_window_id=")
        .expect("WIN-05R restore run should expose final focused_window_id");
    let second_session = extract_final_value(&second, "window_session=")
        .expect("WIN-05R restore run should expose window_session");

    assert_eq!(second_focus, first_focus);
    assert_eq!(second_session, first_session);
    assert!(
        !second.contains("geometry_ok=false"),
        "WIN-05R restore run must preserve geometry invariants. Output:\n{second}"
    );
}

fn run_window_script_with_env(commands: &[u8], extra_env: &[(&str, &str)]) -> String {
    let binary = ensure_kjxlkj_built().expect("binary build should succeed");
    let mut env = vec![("KJXLKJ_INITIAL_LINE", "abc"), ("KJXLKJ_START_CURSOR", "0")];
    env.extend_from_slice(extra_env);
    let mut session = PtySession::spawn(&binary, 100, 30, &env).expect("PTY session should spawn");
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

fn run_terminal_mode_script(with_exit: bool) -> String {
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
        .send_raw(b"\x17T")
        .expect("terminal split command should send");
    session.send_raw(b"i").expect("terminal insert should send");
    if with_exit {
        session
            .send_raw(b"\x1C\x0E")
            .expect("Ctrl-\\\\ Ctrl-n should send");
    }
    session
        .send_raw(b"\x17h\x17l")
        .expect("left/right navigation should send");
    session.quit().expect("quit should succeed")
}

fn extract_action_focus_pairs(output: &str, names: &[&str]) -> Vec<(String, u64)> {
    output
        .lines()
        .filter(|line| {
            names
                .iter()
                .any(|name| line.contains(&format!("resolved_action={name}")))
        })
        .filter_map(|line| {
            let action = extract_value(line, "resolved_action=")?;
            let focus = extract_value(line, "focused_window_id=")?
                .parse::<u64>()
                .ok()?;
            Some((action.to_string(), focus))
        })
        .collect()
}

fn find_action_line<'a>(output: &'a str, action: &str) -> Option<&'a str> {
    output
        .lines()
        .find(|line| line.contains(&format!("resolved_action={action}")))
}

fn extract_final_line(output: &str) -> Option<&str> {
    output.lines().find(|line| line.starts_with("FINAL "))
}

fn extract_final_value(output: &str, marker: &str) -> Option<String> {
    let line = extract_final_line(output)?;
    let pos = line.find(marker)?;
    let suffix = &line[pos + marker.len()..];
    Some(suffix.split_whitespace().next()?.to_string())
}

fn extract_final_u64(output: &str, marker: &str) -> Option<u64> {
    extract_final_value(output, marker)?.parse().ok()
}

fn extract_value<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let pos = line.find(marker)?;
    let suffix = &line[pos + marker.len()..];
    suffix.split_whitespace().next()
}
