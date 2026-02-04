//! Normal mode handler.

use crate::handler::{ModeHandler, ModeResult};
use crate::parser::{ActionKind, OperatorKind, ParseResult, ParsedCommand, Parser, ScrollAction};
use kjxlkj_core_types::{Intent, KeyEvent, Mode, MotionIntent, ScrollIntent};

/// Normal mode handler.
pub struct NormalMode {
    parser: Parser,
}

impl NormalMode {
    /// Create a new normal mode handler.
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    fn handle_command(&self, cmd: ParsedCommand) -> ModeResult {
        let mut intents = Vec::new();

        match cmd.action {
            ActionKind::Motion(motion) => {
                if let Some(op) = cmd.operator {
                    // Operator + motion
                    intents.push(self.operator_motion_intent(op, motion, cmd.count));
                } else {
                    // Just motion
                    for _ in 0..cmd.count {
                        intents.push(Intent::Motion(motion.clone()));
                    }
                }
            }
            ActionKind::Line => {
                if let Some(op) = cmd.operator {
                    intents.push(self.operator_line_intent(op, cmd.count));
                }
            }
            ActionKind::InsertMode => intents.push(Intent::SwitchMode(Mode::Insert)),
            ActionKind::AppendMode => {
                intents.push(Intent::Motion(MotionIntent::Right));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            ActionKind::InsertLineStart => {
                intents.push(Intent::Motion(MotionIntent::FirstNonBlank));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            ActionKind::AppendLineEnd => {
                intents.push(Intent::Motion(MotionIntent::LineEnd));
                intents.push(Intent::Motion(MotionIntent::Right));
                intents.push(Intent::SwitchMode(Mode::Insert));
            }
            ActionKind::OpenBelow => intents.push(Intent::OpenLine { below: true }),
            ActionKind::OpenAbove => intents.push(Intent::OpenLine { below: false }),
            ActionKind::VisualMode => intents.push(Intent::SwitchMode(Mode::Visual)),
            ActionKind::VisualLineMode => intents.push(Intent::SwitchMode(Mode::VisualLine)),
            ActionKind::VisualBlockMode => intents.push(Intent::SwitchMode(Mode::VisualBlock)),
            ActionKind::CommandMode => intents.push(Intent::SwitchMode(Mode::Command)),
            ActionKind::ReplaceMode => intents.push(Intent::SwitchMode(Mode::Replace)),
            ActionKind::ReplaceChar(c) => intents.push(Intent::ReplaceChar(c)),
            ActionKind::Undo => intents.push(Intent::Undo),
            ActionKind::Redo => intents.push(Intent::Redo),
            ActionKind::Paste { before } => intents.push(Intent::Paste {
                before,
                cursor_at_end: false,
            }),
            ActionKind::Repeat => intents.push(Intent::Repeat),
            ActionKind::JoinLines { add_space } => intents.push(Intent::JoinLines { add_space }),
            ActionKind::DeleteChar => {
                intents.push(Intent::Delete { linewise: false });
            }
            ActionKind::DeleteCharBefore => {
                intents.push(Intent::Motion(MotionIntent::Left));
                intents.push(Intent::Delete { linewise: false });
            }
            ActionKind::Substitute => {
                intents.push(Intent::Substitute);
            }
            ActionKind::SubstituteLine => {
                intents.push(Intent::Change { linewise: true });
            }
            ActionKind::DeleteToEnd => {
                intents.push(Intent::Delete { linewise: false });
            }
            ActionKind::ChangeToEnd => {
                intents.push(Intent::Change { linewise: false });
            }
            ActionKind::YankLine => {
                intents.push(Intent::Yank { linewise: true });
            }
            ActionKind::Search { forward } => {
                if forward {
                    intents.push(Intent::SearchForward(String::new()));
                } else {
                    intents.push(Intent::SearchBackward(String::new()));
                }
            }
            ActionKind::NextMatch => intents.push(Intent::NextMatch),
            ActionKind::PrevMatch => intents.push(Intent::PrevMatch),
            ActionKind::SearchWord { forward } => {
                // Will search for word under cursor
                if forward {
                    intents.push(Intent::SearchForward(String::new()));
                } else {
                    intents.push(Intent::SearchBackward(String::new()));
                }
            }
            ActionKind::FindChar { c, forward, .. } => {
                if forward {
                    intents.push(Intent::Motion(MotionIntent::FindChar { c, inclusive: true }));
                } else {
                    intents.push(Intent::Motion(MotionIntent::FindCharBack { c, inclusive: true }));
                }
            }
            ActionKind::RepeatFind => intents.push(Intent::Motion(MotionIntent::RepeatFind)),
            ActionKind::RepeatFindBack => {
                intents.push(Intent::Motion(MotionIntent::RepeatFindBack))
            }
            ActionKind::SetMark(c) => intents.push(Intent::SetMark(c)),
            ActionKind::JumpMark { mark, first_non_blank } => {
                intents.push(Intent::JumpToMark { mark, first_non_blank })
            }
            ActionKind::MacroToggle(c) => intents.push(Intent::MacroToggle(c)),
            ActionKind::MacroPlay(c) => intents.push(Intent::MacroPlay(c)),
            ActionKind::RepeatMacro => intents.push(Intent::MacroPlay('@')),
            ActionKind::SelectRegister(c) => intents.push(Intent::SelectRegister(c)),
            ActionKind::Scroll(scroll) => {
                let intent = match scroll {
                    ScrollAction::HalfPageDown => ScrollIntent::HalfPageDown,
                    ScrollAction::HalfPageUp => ScrollIntent::HalfPageUp,
                    ScrollAction::PageDown => ScrollIntent::PageDown,
                    ScrollAction::PageUp => ScrollIntent::PageUp,
                    ScrollAction::LineDown => ScrollIntent::LineDown,
                    ScrollAction::LineUp => ScrollIntent::LineUp,
                };
                intents.push(Intent::Scroll(intent));
            }
            ActionKind::ZCommand(z) => {
                use crate::parser::ZAction;
                let intent = match z {
                    ZAction::CenterCursor | ZAction::CenterFirstNonBlank => {
                        ScrollIntent::CenterCursor
                    }
                    ZAction::CursorToTop | ZAction::TopFirstNonBlank => ScrollIntent::CursorToTop,
                    ZAction::CursorToBottom | ZAction::BottomFirstNonBlank => {
                        ScrollIntent::CursorToBottom
                    }
                };
                intents.push(Intent::Scroll(intent));
            }
            ActionKind::GCommand(g) => {
                use crate::parser::GAction;
                let motion = match g {
                    GAction::FileStart => MotionIntent::FileStart,
                    GAction::LastNonBlank => MotionIntent::LastNonBlank,
                    GAction::MiddleLine => MotionIntent::LineMiddle,
                    GAction::PrevWordEnd => MotionIntent::WordEndBack,
                    _ => MotionIntent::FileStart,
                };
                intents.push(Intent::Motion(motion));
            }
            ActionKind::Increment(n) => {
                if n == 0 {
                    intents.push(Intent::ToggleCase);
                } else {
                    intents.push(Intent::Increment(n));
                }
            }
            ActionKind::WriteQuit => {
                intents.push(Intent::ExecuteCommand(":wq".to_string()));
            }
            ActionKind::QuitNoSave => {
                intents.push(Intent::ExecuteCommand(":q!".to_string()));
            }
        }

        if intents.is_empty() {
            ModeResult::nop()
        } else {
            ModeResult::intents(intents)
        }
    }

    fn operator_motion_intent(
        &self,
        op: OperatorKind,
        motion: MotionIntent,
        _count: usize,
    ) -> Intent {
        match op {
            OperatorKind::Delete => Intent::Delete { linewise: false },
            OperatorKind::Yank => Intent::Yank { linewise: false },
            OperatorKind::Change => Intent::Change { linewise: false },
            OperatorKind::Indent => Intent::Indent,
            OperatorKind::Outdent => Intent::Outdent,
            OperatorKind::ToggleCase => Intent::ToggleCase,
            OperatorKind::Uppercase => Intent::Uppercase,
            OperatorKind::Lowercase => Intent::Lowercase,
        }
    }

    fn operator_line_intent(&self, op: OperatorKind, _count: usize) -> Intent {
        match op {
            OperatorKind::Delete => Intent::Delete { linewise: true },
            OperatorKind::Yank => Intent::Yank { linewise: true },
            OperatorKind::Change => Intent::Change { linewise: true },
            OperatorKind::Indent => Intent::Indent,
            OperatorKind::Outdent => Intent::Outdent,
            OperatorKind::ToggleCase => Intent::ToggleCase,
            OperatorKind::Uppercase => Intent::Uppercase,
            OperatorKind::Lowercase => Intent::Lowercase,
        }
    }
}

impl Default for NormalMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ModeHandler for NormalMode {
    fn handle_key(&mut self, key: &KeyEvent) -> ModeResult {
        match self.parser.parse(key) {
            ParseResult::Complete(cmd) => self.handle_command(cmd),
            ParseResult::Pending => ModeResult::Pending,
            ParseResult::Invalid => ModeResult::Ignored,
        }
    }

    fn mode(&self) -> Mode {
        Mode::Normal
    }

    fn reset(&mut self) {
        self.parser.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mode_motion() {
        let mut mode = NormalMode::new();
        let result = mode.handle_key(&KeyEvent::char('j'));
        assert!(matches!(result, ModeResult::Consumed(_)));
    }

    #[test]
    fn test_normal_mode_insert() {
        let mut mode = NormalMode::new();
        let result = mode.handle_key(&KeyEvent::char('i'));
        if let ModeResult::Consumed(intents) = result {
            assert!(intents.iter().any(|i| matches!(i, Intent::SwitchMode(Mode::Insert))));
        } else {
            panic!("Expected consumed");
        }
    }
}
