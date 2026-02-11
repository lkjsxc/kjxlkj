//! Async entry point.
//!
//! See /docs/spec/architecture/startup.md "Async Initialization"
//! and /docs/spec/architecture/runtime.md "Event Loop".

use anyhow::Result;
use crossterm::event::{Event, EventStream};
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, Key, KeyModifiers};
use kjxlkj_host;
use kjxlkj_input::{decode_crossterm_event, InputEvent};
use kjxlkj_render;
use tokio_stream::StreamExt;

/// Async initialization and core event loop.
pub async fn run(args: Vec<String>) -> Result<()> {
    // Step 1: Detect terminal capabilities.
    let (cols, rows) = kjxlkj_host::terminal_size()?;

    // Step 2: Initialize editor state.
    let mut state = EditorState::new(cols, rows);

    // Step 3: Open CLI files or create scratch buffer.
    // (Scratch buffer is created by default in EditorState::new)

    // Step 5: Enter raw mode and alternate screen.
    kjxlkj_host::setup_terminal()?;

    // Step 10: Initial render.
    let snapshot = state.snapshot();
    kjxlkj_render::render(&snapshot)?;

    // Step 11: Core event loop.
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().await;
        match event {
            Some(Ok(evt)) => {
                let input = decode_crossterm_event(evt);
                match input {
                    InputEvent::Key(key, mods) => {
                        state.handle_key(&key, &mods);
                    }
                    InputEvent::Resize(c, r) => {
                        state.apply_action(Action::Resize(c, r));
                    }
                    InputEvent::Paste(text) => {
                        // Handle paste as insert chars.
                        for ch in text.chars() {
                            state.apply_action(
                                Action::InsertChar(ch),
                            );
                        }
                    }
                    InputEvent::FocusGained
                    | InputEvent::FocusLost => {}
                    InputEvent::Ignored => {}
                }

                if state.quit_requested {
                    break;
                }

                // Render after each state mutation.
                let snapshot = state.snapshot();
                kjxlkj_render::render(&snapshot)?;
            }
            Some(Err(e)) => {
                // Log error but continue.
                tracing::error!("input error: {}", e);
            }
            None => break,
        }
    }

    // Shutdown: restore terminal.
    kjxlkj_host::restore_terminal()?;
    Ok(())
}
