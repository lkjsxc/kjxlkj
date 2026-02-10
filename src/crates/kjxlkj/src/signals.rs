//! Process signal orchestration.

use tokio::sync::broadcast;

/// Install signal handlers that trigger a quit on SIGINT/SIGTERM.
pub async fn wait_for_signal(quit_tx: broadcast::Sender<()>) {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("SIGINT");
        let mut sigterm = signal(SignalKind::terminate()).expect("SIGTERM");
        tokio::select! {
            _ = sigint.recv() => {
                tracing::info!("Received SIGINT");
            }
            _ = sigterm.recv() => {
                tracing::info!("Received SIGTERM");
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.expect("ctrl_c handler");
        tracing::info!("Received Ctrl+C");
    }

    let _ = quit_tx.send(());
}
