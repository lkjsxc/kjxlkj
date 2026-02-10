//! Signal handling.

use tokio::signal;
use tokio::sync::broadcast;
use tracing::info;

/// Set up signal handlers.
pub async fn setup_signals(quit_tx: broadcast::Sender<()>) {
    tokio::spawn(async move {
        let ctrl_c = signal::ctrl_c();

        #[cfg(unix)]
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register SIGTERM handler");

        #[cfg(unix)]
        tokio::select! {
            _ = ctrl_c => {
                info!("Received SIGINT");
                let _ = quit_tx.send(());
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM");
                let _ = quit_tx.send(());
            }
        }

        #[cfg(not(unix))]
        {
            let _ = ctrl_c.await;
            info!("Received SIGINT");
            let _ = quit_tx.send(());
        }
    });
}
