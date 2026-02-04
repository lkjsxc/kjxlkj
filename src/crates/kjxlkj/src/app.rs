//! Interactive application loop.

use crate::handlers;
use anyhow::Result;
use crossterm::event::Event;
use kjxlkj_core::EditorState;
use kjxlkj_host::TerminalHost;
use kjxlkj_input::convert_event;
use kjxlkj_render::Renderer;
use kjxlkj_services::Services;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

/// Run the editor in interactive mode.
pub fn run_interactive(file_path: Option<PathBuf>) -> Result<()> {
    let mut host = TerminalHost::new();
    host.enter()?;

    let result = run_editor_loop(&mut host, file_path);

    host.leave()?;
    result
}

fn run_editor_loop(host: &mut TerminalHost, file_path: Option<PathBuf>) -> Result<()> {
    let mut state = EditorState::new();
    let services = Services::new();
    let mut renderer = Renderer::new(io::stdout());

    // Set up viewport size
    let (cols, rows) = host.size()?;
    state.viewport.width = cols as usize;
    state.viewport.height = rows as usize;

    // Open file if provided
    if let Some(path) = file_path {
        if path.exists() {
            state.open_file(&path)?;
        } else {
            state.file_path = Some(path);
        }
    }

    // Initial render
    let snapshot = state.snapshot();
    renderer.render(&snapshot)?;

    loop {
        // Wait for event
        if let Some(event) = host.poll_event(Duration::from_millis(100))? {
            // Handle resize
            if let Event::Resize(cols, rows) = event {
                state.viewport.width = cols as usize;
                state.viewport.height = rows as usize;
            }

            // Convert and handle key event
            if let Some(key) = convert_event(event) {
                let action = handlers::handle_key(&mut state, key, &services);
                if let handlers::Action::Quit = action {
                    break;
                }
            }

            // Ensure cursor is visible and valid
            state.ensure_cursor_valid();
            state.scroll_to_cursor();

            // Render
            let snapshot = state.snapshot();
            renderer.render(&snapshot)?;
        }
    }

    Ok(())
}
