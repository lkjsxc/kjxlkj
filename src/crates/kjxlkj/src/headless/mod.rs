//! Headless mode for scripted testing.

mod motion;
pub(crate) mod script;
pub(crate) mod processor;

#[cfg(test)]
mod processor_tests;
#[cfg(test)]
mod script_tests;

use crate::args::Args;
use anyhow::Result;
use kjxlkj_core::{EditorState, TextBuffer};
use kjxlkj_services::fs::FsService;
use std::path::PathBuf;


/// Run in headless mode.
pub fn run(args: &Args) -> Result<()> {
    let mut state = EditorState::new();
    state.viewport.resize(80, 24);

    // Load file if specified
    if let Some(ref path) = args.file {
        load_file(&mut state, path)?;
    }

    // Run script if specified
    if let Some(ref script_path) = args.script {
        let script_content = std::fs::read_to_string(script_path)?;
        script::run_script(&mut state, &script_content)?;
    }

    Ok(())
}

fn load_file(state: &mut EditorState, path: &str) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core::Mode;

    #[test]
    fn headless_mode_creation() {
        let state = EditorState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn headless_viewport_default() {
        let mut state = EditorState::new();
        state.viewport.resize(80, 24);
        assert_eq!(state.viewport.width, 80);
        assert_eq!(state.viewport.height, 24);
    }

    #[test]
    fn headless_buffer_empty() {
        let state = EditorState::new();
        assert_eq!(state.buffer.line_count(), 1);
    }

    #[test]
    fn headless_cursor_default() {
        let state = EditorState::new();
        assert_eq!(state.cursor.line(), 0);
        assert_eq!(state.cursor.col(), 0);
    }

    #[test]
    fn headless_no_selection_default() {
        let state = EditorState::new();
        assert!(state.selection.is_none());
    }

    #[test]
    fn headless_should_quit_default() {
        let state = EditorState::new();
        assert!(!state.should_quit);
    }
}
