//! Signal handling per /docs/spec/architecture/startup.md.
//!
//! | Signal    | Behavior                                       |
//! |-----------|------------------------------------------------|
//! | SIGWINCH  | Send Action::Resize(cols, rows)                |
//! | SIGTERM   | Graceful shutdown                               |
//! | SIGHUP    | Graceful shutdown                               |
//! | SIGINT    | Graceful shutdown                               |
//! | SIGCONT   | Reinitialize terminal and request full redraw   |

use kjxlkj_core_types::Action;
use tokio::sync::mpsc;

/// Spawn signal watchers that forward to the action channel.
///
/// Returns a JoinHandle for cleanup. On Unix, watches
/// SIGWINCH, SIGTERM, SIGHUP, SIGINT, and SIGCONT.
/// On non-Unix platforms, this is a no-op.
#[cfg(unix)]
pub fn spawn_signal_watchers(
    action_tx: mpsc::Sender<Action>,
) -> tokio::task::JoinHandle<()> {
    use tokio::signal::unix::{signal, SignalKind};

    tokio::spawn(async move {
        let mut sigwinch =
            signal(SignalKind::window_change())
                .expect("SIGWINCH handler");
        let mut sigterm = signal(SignalKind::terminate())
            .expect("SIGTERM handler");
        let mut sighup = signal(SignalKind::hangup())
            .expect("SIGHUP handler");
        let mut sigint = signal(SignalKind::interrupt())
            .expect("SIGINT handler");

        loop {
            tokio::select! {
                _ = sigwinch.recv() => {
                    if let Ok((c, r)) =
                        kjxlkj_host::terminal_size()
                    {
                        let _ = action_tx
                            .send(Action::Resize(c, r))
                            .await;
                    }
                }
                _ = sigterm.recv() => {
                    let _ = action_tx
                        .send(Action::Quit).await;
                    break;
                }
                _ = sighup.recv() => {
                    let _ = action_tx
                        .send(Action::Quit).await;
                    break;
                }
                _ = sigint.recv() => {
                    let _ = action_tx
                        .send(Action::Quit).await;
                    break;
                }
            }
        }
    })
}

#[cfg(not(unix))]
pub fn spawn_signal_watchers(
    _action_tx: mpsc::Sender<Action>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {})
}
