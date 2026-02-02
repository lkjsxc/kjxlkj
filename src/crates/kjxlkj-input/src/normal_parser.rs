//! Command parser for normal mode key sequences.

use crate::command::Command;
use crate::key::{Key, KeyEvent, KeyModifiers};
use crate::normal_other::parse_other_char;
use crate::normal_pending::{handle_char_pending, handle_mark_pending, handle_scope_pending};
use crate::normal_types::{CharPendingKind, MarkPendingKind};
use kjxlkj_core_types::{
    ids::RegisterId, motion::Motion, operator::Operator, text_object::TextObjectScope,
};

/// Parser state for normal mode.
#[derive(Debug, Default, Clone)]
pub struct NormalParser {
    count: Option<usize>,
    register: Option<RegisterId>,
    operator: Option<Operator>,
    g_prefix: bool,
    z_prefix: bool,
    char_pending: Option<CharPendingKind>,
    mark_pending: Option<MarkPendingKind>,
    scope_pending: Option<TextObjectScope>,
}

impl NormalParser {
    /// Creates a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the parser state.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Process a key event.
    pub fn parse(&mut self, event: KeyEvent) -> Command {
        let result = self.parse_inner(event);
        match &result {
            Command::Incomplete => {}
            _ => self.reset(),
        }
        result
    }

    fn parse_inner(&mut self, event: KeyEvent) -> Command {
        // Handle char pending
        if let Some(kind) = self.char_pending.take() {
            return handle_char_pending(
                kind,
                event.key,
                self.count.take(),
                self.operator.take(),
                self.register.take(),
            );
        }

        // Handle mark pending
        if let Some(kind) = self.mark_pending.take() {
            return handle_mark_pending(kind, event.key);
        }

        // Handle scope pending (text object)
        if let Some(scope) = self.scope_pending.take() {
            return handle_scope_pending(
                scope,
                event.key,
                self.count.take(),
                self.operator.take(),
                self.register.take(),
            );
        }

        // Handle z prefix
        if self.z_prefix {
            self.z_prefix = false;
            return self.parse_z(event);
        }

        // Handle g prefix
        if self.g_prefix {
            self.g_prefix = false;
            return self.parse_g(event);
        }

        match event.key {
            Key::Char(c) => self.parse_char(c, event.modifiers),
            Key::Escape => {
                self.reset();
                Command::Invalid
            }
            Key::Enter => self.motion_command(Motion::Down),
            Key::Backspace => self.motion_command(Motion::Left),
            _ => Command::Invalid,
        }
    }

    fn parse_char(&mut self, c: char, mods: KeyModifiers) -> Command {
        use super::normal_chars::*;

        if mods.ctrl {
            return self.parse_ctrl(c);
        }

        // Count handling
        if c.is_ascii_digit() && (self.count.is_some() || c != '0') {
            let digit = c as usize - '0' as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return Command::Incomplete;
        }

        // Register prefix
        if c == '"' && self.register.is_none() {
            return Command::Incomplete;
        }

        // Try motion
        if let Some(motion) = parse_motion_char(c) {
            return self.motion_command(motion);
        }

        // Try search motion
        if let Some(motion) = parse_search_motion(c) {
            return self.motion_command(motion);
        }

        // Try insert mode
        if let Some(variant) = parse_insert_char(c) {
            return Command::InsertMode(variant);
        }

        // Try visual mode
        if let Some(variant) = parse_visual_char(c) {
            return Command::VisualMode(variant);
        }

        // Try operator
        if let Some(op) = parse_operator_char(c) {
            return self.operator_or_line(op);
        }

        if let Some(cmd) = parse_other_char(
            c,
            &mut self.count,
            &mut self.g_prefix,
            &mut self.z_prefix,
            &mut self.char_pending,
            &mut self.mark_pending,
            self.register,
        ) {
            return cmd;
        }

        Command::Invalid
    }

    fn parse_ctrl(&mut self, c: char) -> Command {
        super::normal_prefix::parse_ctrl_command(c)
    }

    fn parse_g(&mut self, event: KeyEvent) -> Command {
        let (cmd, _) = super::normal_prefix::parse_g_command(event, self.count.take());
        cmd
    }

    fn parse_z(&mut self, event: KeyEvent) -> Command {
        super::normal_prefix::parse_z_command(event)
    }

    fn operator_or_line(&mut self, op: Operator) -> Command {
        if self.operator.is_some() && self.operator.as_ref() == Some(&op) {
            // Doubled operator (dd, yy, cc, etc.)
            self.operator = None;
            Command::OperatorLine {
                count: self.count.take(),
                operator: op,
                register: self.reg(),
            }
        } else if self.operator.is_none() {
            self.operator = Some(op);
            Command::Incomplete
        } else {
            Command::Invalid
        }
    }

    fn motion_command(&mut self, motion: Motion) -> Command {
        if let Some(op) = self.operator.take() {
            Command::OperatorMotion {
                count: self.count.take(),
                operator: op,
                motion,
                register: self.reg(),
            }
        } else {
            Command::Motion {
                count: self.count.take(),
                motion,
            }
        }
    }

    fn reg(&mut self) -> RegisterId {
        self.register.take().unwrap_or(RegisterId::Unnamed)
    }
}
