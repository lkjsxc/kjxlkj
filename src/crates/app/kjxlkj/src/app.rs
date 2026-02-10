//! Main application.

use anyhow::Result;
use crossterm::terminal;
use kjxlkj_core::CoreTask;
use kjxlkj_core_types::InputAction;
use kjxlkj_host::TerminalHost;
use kjxlkj_input::InputTask;
use kjxlkj_render::RenderTask;
use kjxlkj_services::Services;
use std::io::stdout;
use tracing::info;

use crate::channels::Channels;
use crate::cli::Args;
use crate::signals;

/// Main application.
pub struct App {
    /// CLI arguments.
    #[allow(dead_code)]
    args: Args,
    /// Channels.
    channels: Channels,
    /// Services.
    services: Services,
    /// Terminal host.
    #[allow(dead_code)]
    host: TerminalHost,
}

impl App {
    /// Create a new application.
    pub fn new(args: Args) -> Result<Self> {
        let channels = Channels::new();
        let services = Services::new();
        let host = TerminalHost::init(stdout())?;

        Ok(Self {
            args,
            channels,
            services,
            host,
        })
    }

    /// Run the application.
    pub async fn run(mut self) -> Result<()> {
        info!("Running application");

        // Set up signal handlers.
        signals::setup_signals(self.channels.quit_tx.clone()).await;

        // Get terminal size for initial resize action.
        let (cols, rows) = terminal::size()?;

        // Start services.
        self.services.init().await;

        // Take action receiver.
        let action_rx = self
            .channels
            .take_action_rx()
            .expect("action_rx already taken");

        // Spawn core task.
        let core_task = CoreTask::new(
            action_rx,
            self.channels.snapshot_tx.clone(),
            self.channels.quit_tx.clone(),
        );

        // Spawn input task.
        let input_task = InputTask::new(
            self.channels.action_tx.clone(),
            self.channels.quit_rx(),
        );

        // Spawn render task.
        let render_task = RenderTask::new(
            self.channels.snapshot_rx.clone(),
            self.channels.quit_rx(),
        );

        // Send initial resize.
        let action_tx = self.channels.action_tx.clone();
        let _ = action_tx.send(InputAction::Resize(cols, rows)).await;

        // Run all tasks.
        let core_handle = tokio::spawn(async move { core_task.run().await });
        let input_handle = tokio::spawn(async move { input_task.run().await });
        let render_handle = tokio::spawn(async move { render_task.run().await });

        // Wait for quit signal.
        let mut quit_rx = self.channels.quit_rx();
        let _ = quit_rx.recv().await;

        // Wait for tasks to finish.
        let _ = tokio::join!(core_handle, input_handle, render_handle);

        // Shutdown services.
        self.services.shutdown().await;

        info!("Application stopped");
        Ok(())
    }
}
