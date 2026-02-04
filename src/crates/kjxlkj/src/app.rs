//! Main application logic.

use anyhow::Result;
use crossterm::event::{self};
use kjxlkj_core::{
    BufferName, EditorState, Intent, IntentKind, Key, Motion, MotionKind,
};
use kjxlkj_host::TerminalHost;
use kjxlkj_input::{convert_event, InputEvent};
use kjxlkj_render::Renderer;
use kjxlkj_service_fs::FsService;
use std::path::PathBuf;
use std::time::Duration;

/// Run the TUI application.
pub fn run(args: Vec<String>) -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("kjxlkj=debug")
        .with_writer(std::io::stderr)
        .init();

    // Create editor state
    let mut editor = EditorState::new();

    // Load file if specified
    if args.len() > 1 {
        let path = PathBuf::from(&args[1]);
        if path.exists() {
            let content = FsService::read_file_sync(&path)?;
            editor.open_file(path, &content);
        } else {
            // New file
            editor.open_buffer(BufferName::new(&args[1]), "");
        }
    }

    // Initialize terminal
    let mut host = TerminalHost::new();
    host.init()?;

    // Get terminal size
    let (width, height) = host.size()?;
    editor.resize(width, height);

    // Create renderer
    let mut renderer = Renderer::new(width, height);

    // Main loop
    let result = main_loop(&mut editor, &mut host, &mut renderer);

    // Restore terminal
    host.restore()?;

    result
}

/// Main event loop.
fn main_loop(
    editor: &mut EditorState,
    host: &mut TerminalHost,
    renderer: &mut Renderer,
) -> Result<()> {
    loop {
        // Render current state
        let snapshot = editor.snapshot();
        renderer.render(host.stdout_mut(), &snapshot)?;
        host.flush()?;

        // Check for quit
        if editor.should_quit {
            break;
        }

        // Wait for input
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;

            if let Some(input) = convert_event(event) {
                match input {
                    InputEvent::Key(key) => {
                        process_key(editor, key)?;
                    }
                    InputEvent::Resize(w, h) => {
                        editor.resize(w, h);
                        renderer.resize(w, h);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

/// Process a key press.
fn process_key(editor: &mut EditorState, key: Key) -> Result<()> {
    let intent = editor.mode_state.process_key(key);

    // Clear any previous message on new input
    editor.clear_message();

    apply_intent(editor, intent)
}

/// Apply an intent to the editor state.
fn apply_intent(editor: &mut EditorState, intent: Intent) -> Result<()> {
    match intent.kind {
        IntentKind::Noop | IntentKind::Pending => {}

        // Mode transitions
        IntentKind::EnterInsert { after } => {
            if after {
                editor.active_buffer_mut().move_cursor_cols(1);
            }
        }
        IntentKind::EnterInsertLine { below } => {
            let buf = editor.active_buffer_mut();
            if below {
                // Open line below
                let line = buf.cursor.position.line;
                let line_len = buf.text.line_len_chars(line).unwrap_or(0);
                buf.cursor.position.col = line_len;
                buf.insert_at_cursor("\n");
            } else {
                // Open line above
                buf.cursor.position.col = 0;
                buf.insert_at_cursor("\n");
                buf.move_cursor_lines(-1);
            }
        }
        IntentKind::ExitToNormal => {
            // Adjust cursor when exiting insert mode
            let buf = editor.active_buffer_mut();
            if buf.cursor.position.col > 0 {
                buf.move_cursor_cols(-1);
            }
            editor.command_buffer.clear();
        }
        IntentKind::EnterVisual
        | IntentKind::EnterVisualLine
        | IntentKind::EnterVisualBlock
        | IntentKind::EnterCommand
        | IntentKind::EnterReplace => {
            // Mode already changed in mode_state
        }

        // Cursor movement
        IntentKind::Move(motion) => {
            apply_motion(editor, &motion, intent.count);
        }

        // Insert text
        IntentKind::InsertChar(c) => {
            let s = c.to_string();
            for _ in 0..intent.count {
                editor.active_buffer_mut().insert_at_cursor(&s);
            }
        }

        // Delete operations
        IntentKind::DeleteChar => {
            for _ in 0..intent.count {
                editor.active_buffer_mut().delete_at_cursor();
            }
        }
        IntentKind::DeleteCharBefore => {
            for _ in 0..intent.count {
                editor.active_buffer_mut().delete_before_cursor();
            }
        }
        IntentKind::NewlineBelow => {
            editor.active_buffer_mut().insert_at_cursor("\n");
        }
        IntentKind::NewlineAbove => {
            let buf = editor.active_buffer_mut();
            buf.cursor.position.col = 0;
            buf.insert_at_cursor("\n");
            buf.move_cursor_lines(-1);
        }
        IntentKind::JoinLines => {
            // TODO: implement join lines
        }

        // Operator + motion/text object
        IntentKind::OperatorMotion { operator: _, motion } => {
            // For now, just apply the motion
            // TODO: implement operator application
            apply_motion(editor, &motion, intent.count);
        }
        IntentKind::OperatorTextObject { operator: _, object: _ } => {
            // TODO: implement text object selection and operator
        }
        IntentKind::OperatorLine { operator } => {
            // TODO: implement line-wise operator (dd, yy, cc)
            match operator.kind {
                kjxlkj_core::OperatorKind::Delete => {
                    // Delete current line
                    let buf = editor.active_buffer_mut();
                    let line = buf.cursor.position.line;
                    if let Some(line_start) = buf.text.linecol_to_char(kjxlkj_core::LineCol::new(line, 0)) {
                        let next_line = kjxlkj_core::LineCol::new(line + 1, 0);
                        let line_end = buf.text.linecol_to_char(next_line)
                            .unwrap_or(kjxlkj_core::CharOffset::new(buf.text.len_chars()));
                        buf.text.delete(line_start, line_end);
                        buf.version = buf.version.next();
                        buf.modified = true;
                        buf.clamp_cursor();
                    }
                }
                _ => {}
            }
        }

        // Undo/redo
        IntentKind::Undo => {
            // TODO: implement undo
            editor.set_message("Undo not yet implemented");
        }
        IntentKind::Redo => {
            // TODO: implement redo
            editor.set_message("Redo not yet implemented");
        }

        // Repeat
        IntentKind::RepeatLast => {
            // TODO: implement dot repeat
        }

        // Search
        IntentKind::SearchForward | IntentKind::SearchBackward => {
            // TODO: implement search
        }
        IntentKind::SearchNext | IntentKind::SearchPrev => {
            // TODO: implement search navigation
        }

        // Scrolling
        IntentKind::ScrollUp(n) => {
            editor.viewport.scroll_up(n * intent.count);
        }
        IntentKind::ScrollDown(n) => {
            let max = editor.active_buffer().line_count();
            editor.viewport.scroll_down(n * intent.count, max);
        }
        IntentKind::ScrollHalfUp => {
            editor.viewport.scroll_half_up();
        }
        IntentKind::ScrollHalfDown => {
            let max = editor.active_buffer().line_count();
            editor.viewport.scroll_half_down(max);
        }

        // Commands
        IntentKind::ExecuteCommand(cmd) => {
            execute_command(editor, &cmd)?;
        }
    }

    // Ensure cursor is visible after any action
    editor.ensure_cursor_visible();

    Ok(())
}

/// Apply a motion to move the cursor.
fn apply_motion(editor: &mut EditorState, motion: &Motion, count: usize) {
    let buf = editor.active_buffer_mut();
    let total_count = count * motion.count;

    match motion.kind {
        MotionKind::Char => {
            if motion.forward {
                buf.move_cursor_cols(total_count as isize);
            } else {
                buf.move_cursor_cols(-(total_count as isize));
            }
        }
        MotionKind::Line => {
            if motion.forward {
                buf.move_cursor_lines(total_count as isize);
            } else {
                buf.move_cursor_lines(-(total_count as isize));
            }
        }
        MotionKind::Word => {
            // Simple word motion: move by word boundaries
            // TODO: implement proper word motion
            if motion.forward {
                buf.move_cursor_cols(5 * total_count as isize);
            } else {
                buf.move_cursor_cols(-(5 * total_count as isize));
            }
        }
        _ => {}
    }
}

/// Execute an Ex command.
fn execute_command(editor: &mut EditorState, cmd: &str) -> Result<()> {
    let cmd = cmd.trim();

    match cmd {
        "q" | "quit" => {
            if editor.active_buffer().modified {
                editor.set_message("No write since last change (add ! to override)");
            } else {
                editor.should_quit = true;
            }
        }
        "q!" | "quit!" => {
            editor.should_quit = true;
        }
        "w" | "write" => {
            if let Some(path) = editor.active_buffer().path.clone() {
                let content = editor.active_buffer().text.to_string();
                FsService::write_file_sync(&path, &content)?;
                editor.active_buffer_mut().modified = false;
                editor.set_message(format!("\"{}\" written", path.display()));
            } else {
                editor.set_message("No file name");
            }
        }
        "wq" | "x" => {
            if let Some(path) = editor.active_buffer().path.clone() {
                let content = editor.active_buffer().text.to_string();
                FsService::write_file_sync(&path, &content)?;
                editor.should_quit = true;
            } else {
                editor.set_message("No file name");
            }
        }
        _ => {
            // Check for write with filename
            if cmd.starts_with("w ") {
                let filename = cmd[2..].trim();
                let path = PathBuf::from(filename);
                let content = editor.active_buffer().text.to_string();
                FsService::write_file_sync(&path, &content)?;
                editor.active_buffer_mut().path = Some(path.clone());
                editor.active_buffer_mut().modified = false;
                editor.set_message(format!("\"{}\" written", path.display()));
            } else {
                editor.set_message(format!("Not an editor command: {}", cmd));
            }
        }
    }

    Ok(())
}
