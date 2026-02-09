/// Ex command dispatch — main entry and simple handlers.
use kjxlkj_core_types::Action;

use crate::editor::EditorState;
use crate::ex_parse::ExRange;
use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};

impl EditorState {
    pub(crate) fn execute_cmdline(&mut self) {
        let prefix = self.cmdline.prefix;
        let content = self.cmdline.take_content();
        self.mode = kjxlkj_core_types::Mode::Normal;

        match prefix {
            Some(':') => self.execute_ex_command(&content),
            Some('=') => {
                let from_insert = self.expr_from_insert;
                self.expr_from_insert = false;
                match crate::expr_eval::eval_expression(&content) {
                    Ok(result) => {
                        self.registers.set(kjxlkj_core_edit::RegisterName::Expression, kjxlkj_core_edit::Register::new(result.clone(), false));
                        if from_insert { self.mode = kjxlkj_core_types::Mode::Insert; self.insert_text(&result); }
                        else { self.pending_register = Some('='); self.mode = kjxlkj_core_types::Mode::Normal; }
                    }
                    Err(e) => { self.notify_error(&format!("E15: {e}")); self.mode = if from_insert { kjxlkj_core_types::Mode::Insert } else { kjxlkj_core_types::Mode::Normal }; }
                }
            }
            _ => {
                let (pat, off) = crate::search_types::parse_search_with_offset(&content);
                self.search.pattern = Some(pat);
                self.search.offset = off;
                self.search.active = true;
            }
        }
    }

    pub(crate) fn execute_ex_command(&mut self, cmd: &str) {
        let cmd = cmd.trim();
        // Accumulate function body lines.
        if let Some(ref mut acc) = self.function_body_acc {
            if cmd == "endfunction" || cmd == "endf" {
                let a = self.function_body_acc.take().unwrap();
                self.functions.define(crate::user_functions::UserFunction { name: a.name.clone(), params: a.params, body: a.body });
                self.notify_info(&format!("Function defined: {}", a.name));
            } else { acc.body.push(cmd.to_string()); }
            return;
        }
        // Accumulate :for/:endfor loop body.
        if let Some(ref mut acc) = self.for_loop_acc {
            if cmd == "endfor" || cmd == "endfo" {
                let a = self.for_loop_acc.take().unwrap();
                self.execute_for_loop(&a.var, &a.list_expr, &a.body);
            } else { acc.body.push(cmd.to_string()); }
            return;
        }
        self.last_ex_command = cmd.to_string();
        let current_line = self.windows.focused().cursor.line;
        let buf_id = self.current_buffer_id();
        let total_lines = self.buffers.get(buf_id).map(|b| b.line_count()).unwrap_or(1);
        let text: String = self.buffers.get(buf_id).map(|b| b.content.to_string()).unwrap_or_default();
        let text_lines: Vec<&str> = text.lines().collect();
        let marks = &self.marks;
        let funcs = &self.functions;
        let bid = buf_id.0 as usize;
        let mark_fn = |ch: char| -> Option<usize> { marks.get(ch, bid).map(|p| p.line) };
        let mut reg_vars = std::collections::HashMap::new(); // @a..@z register vars for expression addresses
        for c in b'a'..=b'z' {
            let ch = c as char;
            if let Some(r) = self.registers.get(kjxlkj_core_edit::RegisterName::Named(ch)) { reg_vars.insert(format!("@{ch}"), r.content.clone()); }
        }
        #[rustfmt::skip]
        let call_fn = |expr: &str| -> Option<String> {
            let p = expr.find('(')?; let c = expr.rfind(')')?;
            let (name, args_s) = (expr[..p].trim(), expr[p+1..c].trim());
            let f = funcs.get(name)?;
            let args: Vec<String> = if args_s.is_empty() { vec![] } else { args_s.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect() };
            let vars: std::collections::HashMap<String,String> = f.params.iter().enumerate().map(|(i,p)| (format!("a:{p}"), args.get(i).cloned().unwrap_or_default())).collect();
            f.body.iter().find_map(|l| l.trim().strip_prefix("return ").map(|e| crate::expr_eval::eval_expression_with_vars(e.trim(), &vars).unwrap_or_default()))
        };
        let ctx = RangeContext { current_line, total_lines, lines: &text_lines, mark_line: Some(&mark_fn), last_search: self.search.pattern.as_deref(), vars: Some(&reg_vars), call_fn: Some(&call_fn) };
        let (range, rest) = parse_range_ctx(cmd, &ctx);
        // Mark-not-set detection.
        if range.is_none() && cmd.contains('\'') {
            let after = cmd.split('\'').nth(1).and_then(|s| s.chars().next());
            if let Some(ch) = after {
                if ch.is_alphabetic() && marks.get(ch, bid).is_none() {
                    self.notify_error(&format!("E20: Mark not set: {ch}"));
                    return;
                }
            }
        }
        let range = range.map(|mut r| { if r.start > r.end { std::mem::swap(&mut r.start, &mut r.end); self.notify_info("Backwards range corrected"); } r });
        let rest = rest.trim();

        match rest {
            "q" | "quit" => self.handle_action(Action::Quit),
            "q!" | "quit!" => self.handle_action(Action::ForceQuit),
            "w" | "write" => self.write_current_buffer(),
            "wq" | "x" => { self.write_current_buffer(); self.handle_action(Action::Quit); }
            "bn" | "bnext" => self.next_buffer(),
            "bp" | "bprev" | "bprevious" => self.prev_buffer(),
            "sp" | "split" => self.split_horizontal(),
            "vs" | "vsplit" => self.split_vertical(),
            "d" | "delete" => { self.delete_range(range.unwrap_or(ExRange::single(current_line))); }
            "y" | "yank" => { self.yank_range(range.unwrap_or(ExRange::single(current_line))); }
            _ if rest.starts_with("s/") || rest.starts_with("s#") || rest.starts_with("substitute") => {
                let sub_input = if let Some(stripped) = rest.strip_prefix("substitute") { stripped } else { &rest[1..] };
                self.execute_substitute(sub_input, range.unwrap_or(ExRange::single(current_line)));
            }
            _ if rest.starts_with('!') => { self.handle_filter_shell(rest[1..].trim(), range.unwrap_or(ExRange::single(current_line))); }
            _ if rest == "sort" || rest.starts_with("sort ") || rest.starts_with("sort!") => {
                let flags = rest.strip_prefix("sort").unwrap_or("").trim();
                let r = range.unwrap_or(ExRange { start: 0, end: total_lines.saturating_sub(1) });
                self.handle_sort(flags, r);
            }
            _ if rest.starts_with("call cursor(") => { self.handle_call_cursor(rest); }
            _ if rest.starts_with("call ") => { self.handle_call_function(rest); }
            _ if rest.starts_with("let ") => { self.handle_let_command(rest.strip_prefix("let ").unwrap()); }
            _ if rest.starts_with("e ") || rest.starts_with("edit ") => { let path = rest.split_once(' ').map(|x| x.1).unwrap_or("").trim(); if !path.is_empty() { self.notify_info(&format!("Opening: {path}")); } }
            _ if rest.starts_with("b ") => { if let Ok(n) = rest[2..].trim().parse::<u64>() { self.buffers.switch_to(kjxlkj_core_types::BufferId(n)); } }
            _ if rest.starts_with("echo") => { self.dispatch_echo(rest); }
            _ if rest.starts_with("throw ") => { let msg = rest[6..].trim().trim_matches('"'); self.last_error = Some(msg.to_string()); self.notify_error(&format!("E605: Exception: {msg}")); }
            _ if rest.starts_with("execute ") || rest.starts_with("exe ") => {
                let arg = rest.split_once(' ').map(|x| x.1).unwrap_or("").trim();
                match crate::expr_eval::eval_expression(arg) { Ok(cmd) => { let cmd = cmd.trim().to_string(); if !cmd.is_empty() { self.execute_ex_command(&cmd); } } Err(e) => self.notify_error(&format!("E15: {e}")), }
            }
            _ if rest == "retab" || rest.starts_with("retab ") || rest.starts_with("retab!") => { self.handle_retab(rest.strip_prefix("retab").unwrap_or("")); }
            _ if rest.starts_with("normal ") || rest.starts_with("normal! ") || rest.starts_with("norm ") || rest.starts_with("norm! ") => { self.handle_normal_command(rest); }
            _ if rest.starts_with("for ") => { self.handle_for_start(rest); }
            _ if rest.starts_with("move ") || rest.starts_with("m ") => { let dest: usize = rest.split_once(' ').and_then(|(_,d)| d.trim().parse::<usize>().ok()).unwrap_or(1).saturating_sub(1); self.handle_move_range(range.unwrap_or(ExRange::single(current_line)), dest); }
            _ if rest.starts_with("copy ") || rest.starts_with("co ") || rest.starts_with("t ") => { let dest: usize = rest.split_once(' ').and_then(|(_,d)| d.trim().parse::<usize>().ok()).unwrap_or(1).saturating_sub(1); self.handle_copy_range(range.unwrap_or(ExRange::single(current_line)), dest); }
            _ if rest == "center" || rest.starts_with("center ") || rest == "left" || rest.starts_with("left ") || rest == "right" || rest.starts_with("right ") => {
                self.handle_alignment(rest, range.unwrap_or(crate::ex_parse::ExRange::single(current_line)));
            }
            _ if super::ex_map::is_map_command(rest) => self.handle_map_command(rest),
            _ if rest == "command" || rest == "comclear" => { self.handle_command_command(rest, ""); }
            _ if rest.starts_with("command ") || rest.starts_with("command! ") => {
                let (prefix, args) = rest.split_once(' ').unwrap();
                self.handle_command_command(prefix, args);
            }
            _ if rest.starts_with("delcommand ") => { self.handle_delcommand(rest.strip_prefix("delcommand ").unwrap().trim()); }
            _ if rest.starts_with("autocmd ") || rest == "autocmd" => { self.handle_autocmd(rest.strip_prefix("autocmd").unwrap().trim()); }
            _ if rest.starts_with("mark ") || rest.starts_with("k ") => { self.handle_mark_command(rest); }
            _ if rest.starts_with("delmarks ") => { self.handle_delmarks(rest.strip_prefix("delmarks ").unwrap().trim()); }
            "delmarks!" => {
                let bid = self.current_buffer_id().0 as usize;
                self.marks.clear_buffer(bid);
                self.notify_info("All local marks cleared");
            }
            "marks" => self.handle_list_marks(),
            _ if rest == "registers" || rest == "reg" || rest.starts_with("registers ") || rest.starts_with("reg ") => {
                let filter = rest.split_once(' ').map(|x| x.1.trim()).unwrap_or("");
                self.handle_list_registers_filtered(filter);
            }
            "jumps" => self.handle_list_jumps(),
            "noh" | "nohlsearch" => self.handle_nohlsearch(),
            _ if rest.starts_with("debug @") => {
                let reg = rest.strip_prefix("debug @").unwrap().chars().next().unwrap_or('a');
                self.handle_debug_macro(reg);
            }
            "mksession" => self.handle_mksession(None),
            _ if rest.starts_with("mksession ") => { self.handle_mksession(Some(rest.strip_prefix("mksession ").unwrap().trim())); }
            "source" => self.notify_error("E471: Argument required"),
            _ if rest.starts_with("source ") => { self.handle_source(rest.strip_prefix("source ").unwrap().trim()); }
            "trust" => self.handle_trust_directory(),
            _ if rest == "set" || rest.starts_with("set ") || rest.starts_with("set\t") => {
                let args = rest.strip_prefix("set").unwrap_or("").trim();
                self.handle_set_command(args);
            }
            _ if rest.is_empty() => {
                if let Some(r) = range {
                    self.windows.focused_mut().cursor.line = r.end;
                    self.windows.focused_mut().cursor.grapheme = 0;
                    self.ensure_cursor_visible();
                }
            }
            _ if rest.starts_with("function!") || rest.starts_with("function ") => {
                match crate::user_functions::parse_function_header(rest) {
                    Ok((name, params)) => {
                        self.function_body_acc = Some(crate::editor::FunctionBodyAcc { name, params, body: Vec::new() });
                    }
                    Err(e) => self.notify_error(&e),
                }
            }
            _ => {
                let cmd_word = rest.split_whitespace().next().unwrap_or("");
                if cmd_word.starts_with(|c: char| c.is_ascii_uppercase()) {
                    let args = rest.strip_prefix(cmd_word).unwrap_or("").trim();
                    if let Some(expanded) = self.user_commands.expand(cmd_word, args, false) {
                        self.execute_ex_command(&expanded);
                        return;
                    }
                }
                self.notify_error(&format!("E492: Unknown command: {rest}"));
            }
        }
    }
}
