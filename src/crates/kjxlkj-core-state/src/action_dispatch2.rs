//! Extended action dispatch (overflow from action_dispatch).

use crate::EditorState;
use kjxlkj_core_types::{Action, Mode};

impl EditorState {
    /// Dispatch actions not handled by the primary match.
    pub(crate) fn dispatch_extended(&mut self, action: Action) {
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
            Action::InsertNormal => {
                self.mode = Mode::InsertNormal;
            }
            Action::OperatorTextObject(op, tobj, count) => {
                self.do_operator_text_object(op, tobj, count);
            }
            Action::SwitchBuffer(name) => self.do_switch_buffer(&name),
            Action::Paste(text) => self.do_paste_text(&text),
            Action::SessionSave => {}   // Handled at main loop level
            Action::SessionLoad => {}   // Handled at main loop level
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
            Action::ShellCommand(cmd) => self.do_shell_command(&cmd),
            Action::FilterLines(args) => self.do_filter_lines(&args),
            Action::ExecuteExpr(expr) => self.do_execute_expr(&expr),
            Action::OnlyWindow => self.do_only_window(),
            Action::HideWindow => self.do_close_window(),
            Action::ExchangeWindow => self.do_exchange_window(),
            Action::FocusTopLeft => self.do_focus_top_left(),
            Action::FocusBottomRight => self.do_focus_bottom_right(),
            Action::FocusPrevWindow => self.do_focus_prev_window(),
            Action::MoveWindowToTab => {} // Tab system stub
            Action::NewSplit => self.do_new_split(),
            Action::NewVsplit => self.do_new_vsplit(),
            Action::SplitOpen(path) => self.do_split_open(&path),
            Action::VsplitOpen(path) => self.do_vsplit_open(&path),
            Action::ResizeCmd(args) => self.do_resize_cmd(&args),
            Action::TabNew(path) => self.do_tab_new(path.as_deref()),
            Action::TabClose => self.do_tab_close(),
            Action::TabOnly => self.do_tab_only(),
            Action::TabNext => self.do_tab_next(),
            Action::TabPrev => self.do_tab_prev(),
            Action::TabFirst => self.do_tab_first(),
            Action::TabLast => self.do_tab_last(),
            Action::TabGoto(n) => self.do_tab_goto(n),
            Action::TabMove(args) => self.do_tab_move(&args),
            Action::MapCommand(cmd, args) => {
                self.do_map_command(&cmd, &args);
            }
            Action::UnmapCommand(cmd, args) => {
                self.do_unmap_command(&cmd, &args);
            }
            Action::UserCommand(_) => {} // User command stub
            Action::SourceFile(path) => self.do_source_file(&path),
            Action::SetOption(args) => self.do_set_option(&args),
            Action::LspHover => self.do_lsp_hover(),
            Action::LspCodeAction => self.do_lsp_code_action(),
            Action::LspFormat => self.do_lsp_format(),
            Action::LspRename(name) => self.do_lsp_rename(&name),
            Action::LspSignatureHelp => self.do_lsp_signature_help(),
            Action::LspReferences => self.do_lsp_references(),
            Action::LspDocumentSymbols => self.do_lsp_document_symbols(),
            Action::LspWorkspaceSymbols => self.do_lsp_workspace_symbols(),
            Action::LspCodeLens => self.do_lsp_code_lens(),
            Action::LspInlayHints => self.do_lsp_inlay_hints(),
            Action::LspCallHierarchy => self.do_lsp_call_hierarchy(),
            Action::LspTypeHierarchy => self.do_lsp_type_hierarchy(),
            Action::GitSigns => self.do_git_signs(),
            Action::GitDiff => self.do_git_diff(),
            Action::GitBlame => self.do_git_blame(),
            Action::FlashJump => self.do_flash_jump(),
            Action::IncludeSearch(_) => {} // Include search stub
            Action::MultiCursorAdd => {
                if let Some(win) = self.focused_window() {
                    let cursor = win.cursor;
                    self.multi_cursor.add_cursor(cursor);
                }
            }
            Action::MultiCursorAll => {}  // Select all matches stub
            Action::MultiCursorSkip => {} // Skip match stub
            Action::SnippetExpand => {}   // Snippet expand stub
            Action::SnippetNext => {
                self.snippet_state.next_stop();
            }
            Action::SnippetPrev => {
                self.snippet_state.prev_stop();
            }
            Action::SpellToggle => self.spell_checker.toggle(),
            Action::SpellNext => {
                self.spell_checker.next_error();
            }
            Action::SpellPrev => {
                self.spell_checker.prev_error();
            }
            Action::AutoSaveToggle => {
                self.persistence.auto_save.enabled = !self.persistence.auto_save.enabled;
            }
            Action::UndoTreeToggle => {} // Undo tree viz stub
            Action::NotificationDismiss => {
                self.notifications.dismiss_latest();
            }
            Action::FocusGained | Action::FocusLost => {}
            Action::EnterVisual(_) => {} // Already handled in primary
            _ => {}
        }
    }
}
