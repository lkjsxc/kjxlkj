use kjxlkj_service_terminal::{map_tmux_key, scrollback_capacity, TmuxAction, TmuxState};

// --- Tmux types ---

#[test]
fn tmux_state_fields() {
    let state = TmuxState {
        session_name: "dev".into(),
        windows: vec!["main".into(), "build".into()],
        attached: true,
    };
    assert!(state.attached);
    assert_eq!(state.windows.len(), 2);
}

#[test]
fn tmux_action_variants() {
    let actions = vec![
        TmuxAction::NewWindow,
        TmuxAction::CloseWindow,
        TmuxAction::SplitH,
        TmuxAction::SplitV,
        TmuxAction::SelectPane(1),
        TmuxAction::ResizePane(80, 24),
        TmuxAction::SendKeys("ls".into()),
        TmuxAction::DetachClient,
    ];
    assert_eq!(actions.len(), 8);
}

// --- Tmux key mapping ---

#[test]
fn tmux_key_enter() {
    assert_eq!(map_tmux_key("Enter"), "Enter");
    assert_eq!(map_tmux_key("enter"), "Enter");
}

#[test]
fn tmux_key_backspace() {
    assert_eq!(map_tmux_key("Backspace"), "BSpace");
    assert_eq!(map_tmux_key("backspace"), "BSpace");
}

#[test]
fn tmux_key_passthrough() {
    assert_eq!(map_tmux_key("x"), "x");
    assert_eq!(map_tmux_key("C-a"), "C-a");
}

// --- Scrollback capacity ---

#[test]
fn scrollback_normal() {
    assert_eq!(scrollback_capacity(24), 240);
}

#[test]
fn scrollback_capped() {
    assert_eq!(scrollback_capacity(2000), 10_000);
}

#[test]
fn scrollback_small() {
    assert_eq!(scrollback_capacity(10), 100);
}
