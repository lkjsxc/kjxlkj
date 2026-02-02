//! Application logic.

use crate::cli::Args;
use anyhow::Result;
use kjxlkj_core::{Action, CoreTask};
use kjxlkj_core_types::Mode;
use kjxlkj_core_ui::Dimensions;
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
        if self.args.dump {
            self.run_dump().await
        } else {
            self.run_interactive().await
        }
    }

    /// Runs in dump mode for debugging.
    async fn run_dump(self) -> Result<()> {
        let dims = Dimensions::new(80, 24);
        let (core_task, core_handle) = CoreTask::new();
        let core_future = tokio::spawn(core_task.run());

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

        // Give core time to process
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Render and dump
        let mut renderer = Renderer::new(dims);
        let snapshot = core_handle.snapshot();
        let frame = renderer.render(&snapshot);
        println!("{}", frame.dump());

        // Cleanup
        core_handle.send(Action::ForceQuit).await?;
        let _ = core_future.await;
        Ok(())
    }

    /// Runs in interactive mode.
    async fn run_interactive(self) -> Result<()> {
        let host = TerminalHost::new()?;
        let dims = host.dimensions()?;

        let (core_task, core_handle) = CoreTask::new();
        let core_future = tokio::spawn(core_task.run());

        let mut renderer = Renderer::new(dims);
        let mut parser = InputParser::new();
        let mut events = HostEventStream::new();

        core_handle.send(Action::resize(dims.width, dims.height)).await?;

        for file in &self.args.files {
            if let Some(path_str) = file.to_str() {
                core_handle.send(Action::OpenFile {
                    path: path_str.to_string(),
                }).await?;
            }
        }

        loop {
            let mut snapshot = core_handle.snapshot();
            
            // Only show command line if in command mode and cmdline is open
            if snapshot.mode == Mode::Command {
                if let Some(display) = parser.cmdline().display() {
                    snapshot.command_line = Some(display);
                }
            }

            let frame = renderer.render(&snapshot);
            frame.render()?;

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

        core_handle.send(Action::ForceQuit).await?;
        let _ = core_future.await;
        drop(host);
        Ok(())
    }
}
