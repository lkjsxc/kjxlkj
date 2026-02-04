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

    #[test]
    fn headless_mode_check() {
        let state = EditorState::new();
        // Normal mode should be default
        let mode = state.mode();
        let _ = format!("{:?}", mode);
    }

    #[test]
    fn headless_editor_clamp() {
        let mut state = EditorState::new();
        state.clamp_cursor();
        assert_eq!(state.cursor.col(), 0);
    }

    #[test]
    fn headless_status_message_default() {
        let state = EditorState::new();
        assert!(state.status_message.is_none());
    }

    #[test]
    fn headless_ensure_visible() {
        let mut state = EditorState::new();
        state.ensure_cursor_visible();
        // No panic is success
    }

    #[test]
    fn headless_viewport_size() {
        let mut state = EditorState::new();
        state.viewport.resize(80, 24);
        assert_eq!(state.viewport.width, 80);
    }

    #[test]
    fn headless_buffer_line() {
        let state = EditorState::new();
        let _ = state.buffer.line(0);
    }

    #[test]
    fn headless_cursor_position() {
        let state = EditorState::new();
        let pos = state.cursor.position;
        assert_eq!(pos.line, 0);
    }

    #[test]
    fn headless_selection_none() {
        let state = EditorState::new();
        assert!(state.selection.is_none());
    }

    #[test]
    fn headless_registers_exist() {
        let state = EditorState::new();
        let _ = &state.registers;
    }

    #[test]
    fn headless_marks_exist() {
        let state = EditorState::new();
        let _ = &state.marks;
    }

    #[test]
    fn headless_undo_exists() {
        let state = EditorState::new();
        let _ = &state.undo;
    }

    #[test]
    fn headless_mode_state_exists() {
        let state = EditorState::new();
        let _ = &state.mode_state;
    }

    #[test]
    fn headless_viewport_exists() {
        let state = EditorState::new();
        let _ = &state.viewport;
    }

    #[test]
    fn headless_set_status() {
        let mut state = EditorState::new();
        state.set_status("hello");
        assert!(state.status_message.is_some());
    }

    #[test]
    fn headless_clear_status() {
        let mut state = EditorState::new();
        state.set_status("hello");
        state.clear_status();
        assert!(state.status_message.is_none());
    }
}
