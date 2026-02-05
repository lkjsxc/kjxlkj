//! Host main loop.

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use kjxlkj_core_state::Editor;
use kjxlkj_input::decode_event;
use kjxlkj_render::Renderer;
use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;

/// Run the editor.
pub fn run(file: Option<PathBuf>) -> Result<()> {
    // Enter raw mode
    terminal::enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let result = run_main_loop(file);

    // Restore terminal
    execute!(stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}

fn run_main_loop(file: Option<PathBuf>) -> Result<()> {
    let (width, height) = terminal::size()?;
    let mut editor = Editor::new(width, height);
    let mut renderer = Renderer::new();

    // Open file if provided
    if let Some(path) = file {
        if path.exists() {
            editor.open_file(&path)?;
        }
    }

    // Initial render
    renderer.render(&editor.snapshot())?;

    // Main loop
    while !editor.should_quit() {
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;

            // Handle Ctrl+C
            if let Event::Key(key) = &event {
                if key.code == event::KeyCode::Char('c')
                    && key.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    break;
                }
            }

            if let Some(editor_event) = decode_event(event) {
                editor.process_event(editor_event);
            }

            renderer.render(&editor.snapshot())?;
        }
    }

    Ok(())
}
