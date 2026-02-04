//! Interactive application module.

mod intents;
mod keys;
mod motion;
mod operator;
mod commands;

use crate::args::Args;
use anyhow::Result;
use kjxlkj_core::{EditorState, TextBuffer};
use kjxlkj_host::TerminalHost;
use kjxlkj_input::InputEvent;
use kjxlkj_render::Renderer;
use kjxlkj_services::fs::FsService;
use std::path::PathBuf;

pub use intents::apply_intent;
pub use keys::process_key;
pub use motion::apply_motion;
pub use operator::apply_operator;
pub use commands::execute_command;

/// Run the interactive application.
pub fn run(args: &Args) -> Result<()> {
    let mut host = TerminalHost::new()?;
    host.enter()?;

    let result = run_editor(&mut host, args);

    host.leave()?;
    result
}

fn run_editor(host: &mut TerminalHost, args: &Args) -> Result<()> {
    let mut state = EditorState::new();

    // Set initial viewport size
    let (width, height) = host.size()?;
    state.viewport.resize(width, height.saturating_sub(1));

    // Load file if specified
    if let Some(ref path) = args.file {
        load_file(&mut state, path)?;
    }

    let renderer = Renderer::new();
    let mut stdout = std::io::stdout();

    loop {
        // Render
        let snapshot = state.snapshot();
        renderer.render(&mut stdout, &snapshot)?;

        // Wait for input
        if let Some(event) = host.poll_event(100)? {
            match event {
                InputEvent::Key(key) => {
                    process_key(&mut state, key);
                }
                InputEvent::Resize(w, h) => {
                    state.viewport.resize(w, h.saturating_sub(1));
                }
                _ => {}
            }
        }

        if state.should_quit {
            break;
        }
    }

    Ok(())
}

/// Load a file into the editor state.
pub fn load_file(state: &mut EditorState, path: &str) -> Result<()> {
    let path_buf = PathBuf::from(path);
    if FsService::exists(&path_buf) {
        let content = FsService::read_file(&path_buf)?;
        state.buffer = TextBuffer::from_str(state.buffer.id(), &content);
        state.buffer.set_path(path_buf);
        state.buffer.mark_saved();
    } else {
        state.buffer.set_path(path_buf);
    }
    state.clamp_cursor();
    Ok(())
}
