//! Signal handling.

use tokio::sync::broadcast;

/// Handles Unix signals relevant to the editor.
pub struct SignalHandler {
    quit_tx: broadcast::Sender<()>,
}

impl SignalHandler {
    pub fn new(quit_tx: broadcast::Sender<()>) -> Self {
        Self { quit_tx }
    }

    /// Run signal handling loop.
    pub async fn run(self) {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sigterm = signal(SignalKind::terminate()).expect("SIGTERM");
            let mut sigint = signal(SignalKind::interrupt()).expect("SIGINT");
            let mut sighup = signal(SignalKind::hangup()).expect("SIGHUP");

            tokio::select! {
                _ = sigterm.recv() => {
                    let _ = self.quit_tx.send(());
                }
                _ = sigint.recv() => {
                    let _ = self.quit_tx.send(());
                }
                _ = sighup.recv() => {
                    let _ = self.quit_tx.send(());
                }
            }
        }

        #[cfg(not(unix))]
        {
            tokio::signal::ctrl_c().await.expect("Ctrl-C handler");
            let _ = self.quit_tx.send(());
        }
    }
}

/// Setup SIGWINCH handler that sends resize actions.
#[cfg(unix)]
pub async fn watch_sigwinch(action_tx: tokio::sync::mpsc::Sender<kjxlkj_core_types::Action>) {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sig = signal(SignalKind::window_change()).expect("SIGWINCH");

    loop {
        sig.recv().await;
        if let Ok((cols, rows)) = crossterm::terminal::size() {
            let _ = action_tx
                .send(kjxlkj_core_types::Action::Resize(cols, rows))
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_handler_creation() {
        let (tx, _rx) = broadcast::channel(1);
        let _handler = SignalHandler::new(tx);
    }
}
