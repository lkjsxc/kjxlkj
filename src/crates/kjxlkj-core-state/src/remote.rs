//! Remote editing support via SSH/rsync.
//!
//! Provides state for remote file operations and connection
//! management for editing files on remote machines.

/// Connection method for remote editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteMethod {
    /// SSH connection (host, port).
    Ssh { host: String, port: u16 },
    /// Rsync-based synchronization.
    Rsync { host: String },
    /// SCP file transfer.
    Scp { host: String },
}

/// Status of a remote connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteStatus {
    /// Not connected.
    Disconnected,
    /// Connecting in progress.
    Connecting,
    /// Connection established.
    Connected,
    /// Connection failed.
    Failed,
}

/// State for remote editing features.
#[derive(Debug, Clone)]
pub struct RemoteState {
    /// Current connection method.
    pub method: Option<RemoteMethod>,
    /// Connection status.
    pub status: RemoteStatus,
    /// Remote working directory.
    pub remote_cwd: Option<String>,
    /// Local cache directory for remote files.
    pub cache_dir: Option<String>,
    /// Username for connection.
    pub username: Option<String>,
    /// Whether to use agent forwarding.
    pub agent_forwarding: bool,
}

impl RemoteState {
    pub fn new() -> Self {
        Self {
            method: None,
            status: RemoteStatus::Disconnected,
            remote_cwd: None,
            cache_dir: None,
            username: None,
            agent_forwarding: false,
        }
    }

    /// Parse a remote URI like `ssh://user@host:port/path`.
    pub fn parse_uri(uri: &str) -> Option<(RemoteMethod, String)> {
        if let Some(rest) = uri.strip_prefix("ssh://") {
            let (hostpart, path) = rest.split_once('/').unwrap_or((rest, ""));
            let (userhost, port) = if hostpart.contains(':') {
                let (h, p) = hostpart.rsplit_once(':').unwrap();
                (h, p.parse::<u16>().unwrap_or(22))
            } else {
                (hostpart, 22)
            };
            let host = if let Some((_user, h)) = userhost.split_once('@') {
                h.to_string()
            } else {
                userhost.to_string()
            };
            Some((RemoteMethod::Ssh { host, port }, format!("/{}", path)))
        } else if let Some(rest) = uri.strip_prefix("scp://") {
            let (host, path) = rest.split_once(':').unwrap_or((rest, ""));
            Some((
                RemoteMethod::Scp {
                    host: host.to_string(),
                },
                path.to_string(),
            ))
        } else {
            None
        }
    }

    /// Build the SSH command for opening a remote file.
    pub fn ssh_command(&self, remote_path: &str) -> Option<String> {
        match &self.method {
            Some(RemoteMethod::Ssh { host, port }) => {
                let user = self.username.as_deref().unwrap_or("root");
                Some(format!(
                    "ssh -p {} {}@{} cat {}",
                    port, user, host, remote_path
                ))
            }
            _ => None,
        }
    }

    /// Connect to a remote host.
    pub fn connect(&mut self, method: RemoteMethod) {
        self.method = Some(method);
        self.status = RemoteStatus::Connecting;
    }

    /// Mark connection as established.
    pub fn on_connected(&mut self) {
        self.status = RemoteStatus::Connected;
    }

    /// Disconnect.
    pub fn disconnect(&mut self) {
        self.status = RemoteStatus::Disconnected;
        self.method = None;
        self.remote_cwd = None;
    }
}

impl Default for RemoteState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ssh_uri() {
        let (method, path) = RemoteState::parse_uri("ssh://user@host:2222/tmp/file.rs").unwrap();
        match method {
            RemoteMethod::Ssh { host, port } => {
                assert_eq!(host, "host");
                assert_eq!(port, 2222);
            }
            _ => panic!("expected Ssh"),
        }
        assert_eq!(path, "/tmp/file.rs");
    }

    #[test]
    fn connect_disconnect() {
        let mut state = RemoteState::new();
        state.connect(RemoteMethod::Ssh {
            host: "example.com".into(),
            port: 22,
        });
        assert_eq!(state.status, RemoteStatus::Connecting);
        state.on_connected();
        assert_eq!(state.status, RemoteStatus::Connected);
        state.disconnect();
        assert_eq!(state.status, RemoteStatus::Disconnected);
    }

    #[test]
    fn ssh_command_build() {
        let mut state = RemoteState::new();
        state.method = Some(RemoteMethod::Ssh {
            host: "srv".into(),
            port: 22,
        });
        state.username = Some("deploy".into());
        let cmd = state.ssh_command("/etc/config").unwrap();
        assert!(cmd.contains("deploy@srv"));
    }
}
