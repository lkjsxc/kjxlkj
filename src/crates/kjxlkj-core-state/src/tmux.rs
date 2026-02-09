//! Tmux integration: multiplexer detection, session
//! management, and passthrough escape sequences.

/// Tmux session info detected from environment.
#[derive(Debug, Clone, Default)]
pub struct TmuxState {
    /// Whether running inside tmux.
    pub inside_tmux: bool,
    /// tmux socket path from $TMUX.
    pub socket_path: Option<String>,
    /// tmux session name.
    pub session_name: Option<String>,
    /// Whether to use passthrough escapes for nested terminals.
    pub passthrough: bool,
}

impl TmuxState {
    /// Detect tmux from environment variables.
    pub fn detect() -> Self {
        let tmux_env = std::env::var("TMUX").ok();
        let inside = tmux_env.is_some();
        let (socket, session) = if let Some(ref val) = tmux_env {
            let parts: Vec<&str> = val.split(',').collect();
            let sock = parts.first().map(|s| s.to_string());
            let sess = parts.get(1).map(|s| s.to_string());
            (sock, sess)
        } else {
            (None, None)
        };
        Self {
            inside_tmux: inside,
            socket_path: socket,
            session_name: session,
            passthrough: inside,
        }
    }

    /// Wrap an escape sequence for tmux passthrough.
    /// In tmux, DCS sequences must be wrapped:
    /// `\x1bPtmux;\x1b{inner}\x1b\\`
    pub fn wrap_escape(&self, inner: &str) -> String {
        if self.passthrough && self.inside_tmux {
            format!("\x1bPtmux;\x1b{}\x1b\\", inner)
        } else {
            inner.to_string()
        }
    }

    /// Build a tmux send-keys command.
    pub fn send_keys_cmd(&self, target: &str, keys: &str) -> Option<String> {
        if !self.inside_tmux {
            return None;
        }
        Some(format!("tmux send-keys -t {} {}", target, keys))
    }

    /// Build a tmux split-window command.
    pub fn split_cmd(&self, horizontal: bool) -> Option<String> {
        if !self.inside_tmux {
            return None;
        }
        let flag = if horizontal { "-h" } else { "-v" };
        Some(format!("tmux split-window {}", flag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_escape_not_in_tmux() {
        let state = TmuxState::default();
        assert_eq!(state.wrap_escape("\x1b[2J"), "\x1b[2J");
    }

    #[test]
    fn wrap_escape_in_tmux() {
        let state = TmuxState {
            inside_tmux: true,
            passthrough: true,
            ..Default::default()
        };
        let wrapped = state.wrap_escape("\x1b[2J");
        assert!(wrapped.starts_with("\x1bPtmux;"));
        assert!(wrapped.ends_with("\x1b\\"));
    }

    #[test]
    fn send_keys_cmd() {
        let state = TmuxState {
            inside_tmux: true,
            ..Default::default()
        };
        let cmd = state.send_keys_cmd("0", "ls Enter");
        assert!(cmd.is_some());
        assert!(cmd.unwrap().contains("send-keys"));
    }
}
