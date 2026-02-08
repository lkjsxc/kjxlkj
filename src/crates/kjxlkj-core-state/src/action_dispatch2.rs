//! Extended action dispatch (overflow from action_dispatch).

use kjxlkj_core_types::{Action, Mode};
use crate::EditorState;

impl EditorState {
    /// Dispatch actions not handled by the primary match.
    pub(crate) fn dispatch_extended(
        &mut self,
        action: Action,
    ) {
        match action {
            Action::SetMark(ch) => self.do_set_mark(ch),
            Action::JumpToMark(ch) => self.do_jump_to_mark(ch),
            Action::JumpToMarkLine(ch) => self.do_jump_to_mark_line(ch),
            Action::AlternateFile => self.do_alternate_file(),
            Action::OpenFile(path) => self.do_open_file(&path),
            Action::NextBuffer => self.do_next_buffer(),
            Action::PrevBuffer => self.do_prev_buffer(),
            Action::DeleteBuffer => self.do_delete_buffer(),
            Action::SplitHorizontal => self.do_split_horizontal(),
            Action::SplitVertical => self.do_split_vertical(),
            Action::FocusWindow(dir) => self.do_focus_window(dir),
            Action::CycleWindow => self.do_cycle_window(),
            Action::CloseWindow => self.do_close_window(),
            Action::ExecuteCommand(cmd) => {
                if let Some(a) = crate::dispatch_command(&cmd) {
                    self.dispatch(a);
                }
            }
            Action::CmdlineChar(ch) => self.do_cmdline_char(ch),
            Action::CmdlineBackspace => self.do_cmdline_backspace(),
            Action::CmdlineComplete => self.do_cmdline_complete(),
            Action::CmdlineHistory(dir) => self.do_cmdline_history(dir),
            Action::SearchForward(pat) => self.do_search_forward(pat),
            Action::SearchBackward(pat) => self.do_search_backward(pat),
            Action::NextMatch => self.do_next_match(),
            Action::PrevMatch => self.do_prev_match(),
            Action::RecordMacro(reg) => self.do_record_macro(reg),
            Action::StopRecordMacro => self.do_stop_record_macro(),
            Action::PlayMacro(reg, count) => self.do_play_macro(reg, count),
            Action::SetRegister(_name) => {} // Register prefix handled in key dispatch
            Action::ResizeWindow(dir, amount) => self.do_resize_window(dir, amount),
            Action::EqualizeWindows => self.do_equalize_windows(),
            Action::ZoomWindow => self.do_zoom_window(),
            Action::RotateWindows(forward) => self.do_rotate_windows(forward),
            Action::MoveWindow(dir) => self.do_move_window(dir),
            Action::InsertNormal => { self.mode = Mode::InsertNormal; }
            Action::OperatorTextObject(op, tobj, count) => {
                self.do_operator_text_object(op, tobj, count);
            }
            Action::SwitchBuffer(name) => self.do_switch_buffer(&name),
            Action::Paste(text) => self.do_paste_text(&text),
            Action::SessionSave => {} // Handled at main loop level
            Action::SessionLoad => {} // Handled at main loop level
            Action::SpawnTerminal => {} // Handled at main loop level
            Action::Substitute(args) => self.do_substitute(&args),
            Action::InsertRegister(reg) => self.do_insert_register(reg),
            Action::GlobalCommand(args) => self.do_global_command(&args),
            Action::VglobalCommand(args) => self.do_vglobal_command(&args),
            Action::SortLines(args) => self.do_sort_lines(&args),
            Action::RangeDelete(args) => self.dispatch_range_delete(&args),
            Action::RangeYank(args) => self.dispatch_range_yank(&args),
            Action::RangeCopy(args) => self.dispatch_range_copy(&args),
            Action::RangeMove(args) => self.dispatch_range_move(&args),
            Action::RangeNormal(args) => self.dispatch_range_normal(&args),
            Action::ReadFile(path) => self.do_read_file(&path),
            Action::FocusGained | Action::FocusLost => {}
            Action::EnterVisual(_) => {} // Already handled in primary
            _ => {}
        }
    }
}
