//! Main application loop.

use anyhow::Result;
use crossterm::event::Event;
use kjxlkj_core::EditorState;
use kjxlkj_host::TerminalHost;
use kjxlkj_input::convert_event;
use kjxlkj_render::Renderer;
use std::path::Path;
use std::time::Duration;

use crate::handler::handle_key;

/// Run the editor application.
pub async fn run(file: Option<&str>) -> Result<()> {
    let mut host = TerminalHost::new()?;
    let mut renderer = Renderer::new();
    let mut editor = EditorState::new();

    // Load file if specified.
    if let Some(path) = file {
        load_file(&mut editor, path).await?;
    }

    // Set initial viewport.
    let (width, height) = host.size()?;
    editor.set_viewport_size(height as usize, width as usize);

    // Initial render.
    let snapshot = editor.snapshot();
    renderer.render(host.stdout(), &snapshot)?;

    // Event loop.
    loop {
        // Poll for events (event-driven, no busy loop).
        if let Some(event) = host.poll_event(Duration::from_millis(100))? {
            match event {
                Event::Key(_) => {
                    if let Some(key) = convert_event(event) {
                        handle_key(&mut editor, key).await;
                    }
                }
                Event::Resize(w, h) => {
                    editor.set_viewport_size(h as usize, w as usize);
                }
                _ => {}
            }

            // Render after input.
            editor.scroll_to_cursor();
            let snapshot = editor.snapshot();
            renderer.render(host.stdout(), &snapshot)?;

            if editor.should_quit {
                break;
            }
        }
    }

    host.restore()?;
    Ok(())
}

async fn load_file(editor: &mut EditorState, path: &str) -> Result<()> {
    let path_obj = Path::new(path);
    if path_obj.exists() {
        let content = kjxlkj_service_fs::FsService::read_file(path_obj).await?;
        editor.buffer = kjxlkj_core::state::BufferState::from_file(
            kjxlkj_core::types::BufferId::new(0),
            path.to_string(),
            &content,
        );
    } else {
        // New file.
        editor.buffer.path = Some(path.to_string());
    }
    Ok(())
}
