/// Action dispatch for EditorState.
use kjxlkj_core_types::{Action, CommandKind, Mode};

use crate::editor::EditorState;

impl EditorState {
    /// Process an action.
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => {
                if self.has_unsaved_buffers() {
                    self.notify_error("Unsaved buffers. Use :q! to force quit.");
                } else {
                    self.quit_requested = true;
                }
            }
            Action::ForceQuit => {
                self.quit_requested = true;
            }
            Action::WriteQuit => {
                self.write_current_buffer();
                self.quit_requested = true;
            }
            Action::Resize(cols, rows) => {
                self.terminal_size = (cols, rows);
            }
            Action::Paste(text) => self.insert_text(&text),
            Action::MoveUp(n) => self.move_cursor_up(n),
            Action::MoveDown(n) => self.move_cursor_down(n),
            Action::MoveLeft(n) => self.move_cursor_left(n),
            Action::MoveRight(n) => self.move_cursor_right(n),
            Action::MoveToLineStart => self.move_to_line_start(),
            Action::MoveToFirstNonBlank => self.move_to_first_non_blank(),
            Action::MoveToLineEnd => self.move_to_line_end(),
            Action::MoveWordForward(n) => self.move_word_forward(n),
            Action::MoveWordBackward(n) => self.move_word_backward(n),
            Action::MoveWordEndForward(n) => self.move_word_end_forward(n),
            Action::MoveToTop => self.move_to_top(),
            Action::MoveToBottom => self.move_to_bottom(),
            Action::MoveToLine(n) => self.move_to_line(n),
            Action::PageUp => self.page_up(),
            Action::PageDown => self.page_down(),
            Action::HalfPageUp => self.half_page_up(),
            Action::HalfPageDown => self.half_page_down(),
            Action::InsertBefore => self.enter_insert(),
            Action::InsertAfter => self.enter_insert_after(),
            Action::InsertAtLineStart => self.enter_insert_line_start(),
            Action::InsertAtLineEnd => self.enter_insert_line_end(),
            Action::OpenBelow => self.open_below(),
            Action::OpenAbove => self.open_above(),
            Action::InsertChar(c) => self.insert_char(c),
            Action::InsertNewline => self.insert_newline(),
            Action::DeleteCharBackward => self.delete_char_backward(),
            Action::DeleteCharForward => self.delete_char_forward(),
            Action::DeleteChar(n) => {
                for _ in 0..n {
                    self.delete_char_forward();
                }
            }
            Action::DeleteCharBack(n) => {
                for _ in 0..n {
                    self.delete_char_backward();
                }
            }
            Action::Undo => self.undo(),
            Action::Redo => self.redo(),
            Action::JoinLines => self.join_lines(true),
            Action::JoinLinesNoSpace => self.join_lines(false),
            Action::DeleteLine(n) => self.delete_lines(n),
            Action::YankLine(n) => self.yank_lines(n),
            Action::EnterNormal => {
                self.mode = Mode::Normal;
            }
            Action::EnterCommandEx => {
                self.mode = Mode::Command(CommandKind::Ex);
                self.cmdline.open(':');
            }
            Action::EnterSearchForward => {
                self.mode = Mode::Command(CommandKind::Search);
                self.cmdline.open('/');
                self.search.forward = true;
            }
            Action::EnterSearchBackward => {
                self.mode = Mode::Command(CommandKind::Search);
                self.cmdline.open('?');
                self.search.forward = false;
            }
            Action::CmdlineInsertChar(c) => self.cmdline.insert_char(c),
            Action::CmdlineBackspace => self.cmdline.backspace(),
            Action::CmdlineExecute => self.execute_cmdline(),
            Action::CmdlineCancel => {
                self.cmdline.close();
                self.mode = Mode::Normal;
            }
            Action::WriteBuffer => self.write_current_buffer(),
            Action::NextBuffer => self.next_buffer(),
            Action::PrevBuffer => self.prev_buffer(),
            Action::SplitHorizontal => self.split_horizontal(),
            Action::SplitVertical => self.split_vertical(),
            Action::CloseWindow => self.close_window(),
            Action::FocusNextWindow => self.windows.focus_next(),
            Action::FocusPrevWindow => self.windows.focus_prev(),
            Action::ScrollCenterCursor => self.scroll_center_cursor(),
            Action::ScrollCursorTop => self.scroll_cursor_top(),
            Action::ScrollCursorBottom => self.scroll_cursor_bottom(),
            Action::PutAfter => self.put_after(),
            Action::PutBefore => self.put_before(),
            Action::SearchNext => self.search_next(),
            Action::SearchPrev => self.search_prev(),
            Action::SetMark(c) => self.set_mark_at_cursor(c),
            Action::JumpToMark(c) => self.jump_to_mark(c),
            Action::JumpToMarkLine(c) => self.jump_to_mark_line(c),
            Action::SelectRegister(c) => {
                self.pending_register = Some(c);
            }
            Action::StartRecording(c) => self.start_recording(c),
            Action::StopRecording => self.stop_recording(),
            Action::PlayMacro(c) => self.play_macro(c, 1),
            Action::EnterOperatorPending(op) => {
                self.op_count = self.dispatch.take_count();
                self.motion_count = None;
                self.g_prefix = false;
                self.mode = Mode::OperatorPending(op);
            }
            Action::MoveToMatchingBracket => {}
            Action::FindCharForward(c) => self.find_char_forward(c),
            Action::FindCharBackward(c) => self.find_char_backward(c),
            Action::TillCharForward(c) => self.till_char_forward(c),
            Action::TillCharBackward(c) => self.till_char_backward(c),
            Action::RepeatFindChar => self.repeat_find_char(),
            Action::RepeatFindCharReverse => self.repeat_find_char_reverse(),
            Action::ToggleCase => self.toggle_case(),
            Action::ChangelistOlder => self.changelist_older(),
            Action::ChangelistNewer => self.changelist_newer(),
            Action::DotRepeat => {}
            _ => {}
        }
    }
}
