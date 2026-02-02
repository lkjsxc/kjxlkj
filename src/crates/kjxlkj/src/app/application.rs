use kjxlkj_core::{Buffer, Mode};
use kjxlkj_core_types::{intent::Operator, BufferId, VisualMode};
use kjxlkj_render::Terminal;
use std::{io, path::Path};

use super::command_mode::{CommandModeAction, CommandResult};
use super::insert_mode::InsertAction;
use super::normal_mode::NormalAction;
use super::operator_pending::{OperatorPendingAction, apply_operator_motion};
use super::replace_mode::ReplaceAction;
use super::visual_mode::{VisualAction, VisualSelection};

/// The main application state and event loop.
pub struct Application {
    pub(super) terminal: Terminal,
    pub(super) should_quit: bool,
    pub(super) mode: Mode,
    pub(super) buffer: Buffer,
    pub(super) scroll_offset: usize,
    pub(super) command_line: String,
    pub(super) message: Option<String>,
    pub(super) visual_anchor: Option<VisualSelection>,
    pub(super) pending_operator: Option<Operator>,
    pub(super) pending_count: Option<usize>,
}

impl Application {
    /// Creates a new application.
    pub fn new() -> io::Result<Self> {
        let terminal = Terminal::new()?;
        let buffer = Buffer::from_text(BufferId::new(1), "Welcome to kjxlkj!\n\nPress 'i' to insert, 'v' for visual.\nPress ':w' to save, ':q' to quit.\n");
        Ok(Self::with_buffer(terminal, buffer))
    }

    /// Creates application and opens a file.
    pub fn with_file(path: &Path) -> io::Result<Self> {
        let terminal = Terminal::new()?;
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let mut buffer = Buffer::from_text(BufferId::new(1), &content);
        buffer.set_path(path.to_path_buf());
        Ok(Self::with_buffer(terminal, buffer))
    }

    fn with_buffer(terminal: Terminal, buffer: Buffer) -> Self {
        Self {
            terminal,
            should_quit: false,
            mode: Mode::Normal,
            buffer,
            scroll_offset: 0,
            command_line: String::new(),
            message: None,
            visual_anchor: None,
            pending_operator: None,
            pending_count: None,
        }
    }

    /// Runs the main event loop.
    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.enter_raw_mode()?;
        while !self.should_quit {
            self.render()?;
            self.handle_events()?;
        }
        self.terminal.exit_raw_mode()?;
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        use crossterm::event::{self, Event, KeyEventKind};
        self.message = None;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key(key_event.code);
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: crossterm::event::KeyCode) {
        match self.mode {
            Mode::Normal => self.handle_normal(code),
            Mode::Insert => self.handle_insert(code),
            Mode::Visual(vm) => self.handle_visual(code, vm),
            Mode::Command(_) => self.handle_command(code),
            Mode::Replace => self.handle_replace(code),
            Mode::OperatorPending => self.handle_operator_pending(code),
        }
    }

    fn handle_normal(&mut self, code: crossterm::event::KeyCode) {
        match super::normal_mode::handle_normal_key(code, &mut self.buffer) {
            NormalAction::Quit => self.should_quit = true,
            NormalAction::ChangeMode(m) => {
                if let Mode::Visual(_) = m {
                    self.visual_anchor = Some(VisualSelection {
                        anchor_line: self.buffer.cursor_line(),
                        anchor_col: self.buffer.cursor_col(),
                    });
                }
                self.mode = m;
            }
            NormalAction::StartOperator(op) => {
                self.pending_operator = Some(op);
                self.mode = Mode::OperatorPending;
            }
            NormalAction::None => {}
        }
    }

    fn handle_insert(&mut self, code: crossterm::event::KeyCode) {
        if let InsertAction::ExitToNormal = super::insert_mode::handle_insert_key(code, &mut self.buffer) {
            self.mode = Mode::Normal;
        }
    }

    fn handle_visual(&mut self, code: crossterm::event::KeyCode, vm: VisualMode) {
        match super::visual_mode::handle_visual_key(code, &mut self.buffer, vm) {
            VisualAction::ExitToNormal | VisualAction::Delete | VisualAction::Yank => {
                self.visual_anchor = None;
                self.mode = Mode::Normal;
            }
            VisualAction::ExitToInsert | VisualAction::Change => {
                self.visual_anchor = None;
                self.mode = Mode::Insert;
            }
            VisualAction::SwitchVariant(new_vm) => {
                if self.mode == Mode::Visual(new_vm) {
                    self.visual_anchor = None;
                    self.mode = Mode::Normal;
                } else {
                    self.mode = Mode::Visual(new_vm);
                }
            }
            VisualAction::None => {}
        }
    }

    fn handle_command(&mut self, code: crossterm::event::KeyCode) {
        match super::command_mode::handle_command_key(code, &mut self.command_line) {
            CommandModeAction::Exit => self.mode = Mode::Normal,
            CommandModeAction::Execute(cmd) => {
                match super::command_mode::execute_command(&cmd, &mut self.buffer) {
                    CommandResult::Quit => self.should_quit = true,
                    CommandResult::Message(m) => self.message = Some(m),
                    CommandResult::Continue => {}
                }
                self.mode = Mode::Normal;
            }
            CommandModeAction::Continue => {}
        }
    }

    fn handle_replace(&mut self, code: crossterm::event::KeyCode) {
        if let ReplaceAction::ExitToNormal = super::replace_mode::handle_replace_key(code, &mut self.buffer) {
            self.mode = Mode::Normal;
        }
    }

    fn handle_operator_pending(&mut self, code: crossterm::event::KeyCode) {
        if let Some(op) = self.pending_operator {
            match super::operator_pending::handle_operator_pending_key(
                code,
                &mut self.buffer,
                op,
                self.pending_count,
            ) {
                OperatorPendingAction::Complete(motion) => {
                    apply_operator_motion(&mut self.buffer, &motion);
                    if motion.operator == Operator::Change {
                        self.mode = Mode::Insert;
                    } else {
                        self.mode = Mode::Normal;
                    }
                    self.pending_operator = None;
                    self.pending_count = None;
                }
                OperatorPendingAction::Cancel => {
                    self.mode = Mode::Normal;
                    self.pending_operator = None;
                    self.pending_count = None;
                }
                OperatorPendingAction::Continue => {}
            }
        } else {
            self.mode = Mode::Normal;
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        let _ = self.terminal.exit_raw_mode();
    }
}