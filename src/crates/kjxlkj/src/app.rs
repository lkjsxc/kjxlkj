//! Application logic.

use crate::cli::Args;
use anyhow::Result;
use kjxlkj_core::{Action, CoreTask};
use kjxlkj_host::{HostEvent, HostEventStream, TerminalHost};
use kjxlkj_input::{InputParser, Key};
use kjxlkj_render::Renderer;

/// Main application.
pub struct App {
    /// CLI arguments.
    args: Args,
}

impl App {
    /// Creates a new application.
    pub fn new(args: Args) -> Result<Self> {
        Ok(Self { args })
    }

    /// Runs the application.
    pub async fn run(self) -> Result<()> {
        // Initialize terminal
        let host = TerminalHost::new()?;
        let dims = host.dimensions()?;

        // Initialize core
        let (core_task, core_handle) = CoreTask::new();
        let core_future = tokio::spawn(core_task.run());

        // Initialize renderer
        let mut renderer = Renderer::new(dims);

        // Initialize input
        let mut parser = InputParser::new();
        let mut events = HostEventStream::new();

        // Send initial resize
        core_handle.send(Action::resize(dims.width, dims.height)).await?;

        // Open files from command line
        for file in &self.args.files {
            if let Some(path_str) = file.to_str() {
                core_handle.send(Action::OpenFile {
                    path: path_str.to_string(),
                }).await?;
            }
        }

        // Main loop
        loop {
            // Render current state
            let snapshot = core_handle.snapshot();
            let frame = renderer.render(&snapshot);
            frame.render()?;

            // Wait for event
            if let Some(event) = events.next().await {
                match event {
                    HostEvent::Key(key_event) => {
                        let key = Key::from(key_event);
                        if let Some(intent) = parser.parse(key, snapshot.mode) {
                            use kjxlkj_core_mode::IntentKind;
                            if matches!(intent.kind, IntentKind::Quit) {
                                break;
                            }
                            core_handle.send(Action::Intent(intent)).await?;
                        }
                    }
                    HostEvent::Resize(new_dims) => {
                        core_handle.send(Action::resize(new_dims.width, new_dims.height)).await?;
                    }
                    HostEvent::FocusGained | HostEvent::FocusLost => {}
                }
            }
        }

        // Cleanup
        core_handle.send(Action::ForceQuit).await?;
        let _ = core_future.await;

        drop(host);
        Ok(())
    }
}
