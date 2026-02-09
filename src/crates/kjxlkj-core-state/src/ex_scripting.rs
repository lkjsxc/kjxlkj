/// Scripting-related ex commands: user commands, autocmd, marks, registers.
use crate::editor::EditorState;

impl EditorState {
    #[rustfmt::skip]
    pub(crate) fn handle_command_command(&mut self, prefix: &str, args: &str) {
        use crate::user_commands_parse::parse_command_def;
        if prefix == "comclear" { self.user_commands.clear(); return self.notify_info("All user commands cleared"); }
        if args.is_empty() { return self.notify_info(&format_user_cmd_count(self.user_commands.list().len())); }
        let overwrite = prefix == "command!";
        let input = if overwrite { format!("! {args}") } else { args.to_string() };
        match parse_command_def(&input) {
            Ok((cmd, ow)) => {
                let name = cmd.name.clone();
                match self.user_commands.define(cmd, ow || overwrite) {
                    Ok(()) => self.notify_info(&format!("Command defined: {name}")),
                    Err(e) => self.notify_error(&e),
                }
            }
            Err(e) => self.notify_error(&e),
        }
    }

    pub(crate) fn handle_delcommand(&mut self, name: &str) {
        match self.user_commands.remove(name) { Ok(()) => self.notify_info(&format!("Command removed: {name}")), Err(e) => self.notify_error(&e) }
    }

    #[rustfmt::skip]
    pub(crate) fn handle_autocmd(&mut self, args: &str) {
        use crate::events_types::EventKind;
        if args.is_empty() { return self.notify_info(&format!("{} autocmd(s) registered", self.events.len())); }
        let parts: Vec<&str> = args.splitn(3, ' ').collect();
        if parts.len() < 2 { return self.notify_error("E471: Usage: autocmd {event} {pattern} {cmd}"); }
        let en = parts[0];
        let event = match en {
            "BufNew" | "BufferNew" => EventKind::BufferNew, "BufRead" | "BufferRead" => EventKind::BufferRead,
            "BufWritePre" | "BufferWritePre" => EventKind::BufferWritePre, "BufWrite" | "BufferWrite" => EventKind::BufferWrite,
            "BufWritePost" | "BufferWritePost" => EventKind::BufferWritePost, "BufEnter" | "BufferEnter" => EventKind::BufferEnter,
            "BufLeave" | "BufferLeave" => EventKind::BufferLeave, "BufDelete" | "BufferDelete" => EventKind::BufferDelete,
            "WinNew" | "WindowNew" => EventKind::WindowNew, "WinClosed" | "WindowClosed" => EventKind::WindowClosed,
            "WinEnter" | "WindowEnter" => EventKind::WindowEnter, "WinLeave" | "WindowLeave" => EventKind::WindowLeave,
            "ModeChanged" => EventKind::ModeChanged, "InsertEnter" => EventKind::InsertEnter,
            "InsertLeave" => EventKind::InsertLeave, "CursorMoved" => EventKind::CursorMoved,
            "CursorHold" => EventKind::CursorHold, "FileType" => EventKind::FileType, "ExitPre" => EventKind::ExitPre,
            _ => return self.notify_error(&format!("E216: Unknown event: {en}")),
        };
        let (pattern, command) = if parts.len() == 3 { (Some(parts[1].to_string()), parts[2].to_string()) } else { (None, parts[1].to_string()) };
        self.events.register(event, command, pattern, None);
        self.notify_info(&format!("Autocmd registered for {en}"));
    }

    pub(crate) fn handle_set_mark(&mut self, name: char) {
        let c = self.windows.focused().cursor;
        let bid = self.current_buffer_id().0 as usize;
        self.marks.set(
            name,
            crate::marks::MarkPosition::new(bid, c.line, c.grapheme),
        );
        self.notify_info(&format!("Mark '{name}' set"));
    }

    #[rustfmt::skip]
    pub(crate) fn handle_delmarks(&mut self, names: &str) {
        let bid = self.current_buffer_id().0 as usize;
        let chars: Vec<char> = names.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if i + 2 < chars.len() && chars[i + 1] == '-' && chars[i].is_alphanumeric() && chars[i + 2].is_alphanumeric() {
                let (a, b) = (chars[i] as u8, chars[i + 2] as u8);
                for c in a.min(b)..=a.max(b) { self.marks.delete(c as char, bid); }
                i += 3;
            } else if chars[i].is_alphanumeric() {
                self.marks.delete(chars[i], bid); i += 1;
            } else { i += 1; }
        }
        self.notify_info(&format!("Marks deleted: {names}"));
    }

    pub(crate) fn handle_list_marks(&mut self) {
        let bid = self.current_buffer_id().0 as usize;
        let marks = self.marks.list_for_buffer(bid);
        if marks.is_empty() { return self.notify_info("No marks set"); }
        let mut lines = vec!["mark line  col".to_string()];
        for (name, pos) in &marks { lines.push(format!(" {name}   {:>4}  {:>3}", pos.line + 1, pos.col + 1)); }
        self.notify_info(&lines.join("\n"));
    }

    pub(crate) fn handle_list_registers_filtered(&mut self, filter: &str) {
        use kjxlkj_core_edit::RegisterName;
        let filter_chars: Vec<char> = filter.chars().collect();
        let show = |c: char| -> bool { filter_chars.is_empty() || filter_chars.contains(&c) };
        let mut lines = Vec::new();
        lines.push("--- Registers ---".to_string());
        if show('"') { if let Some(r) = self.registers.get_unnamed() {
            let s = r.content.replace('\n', "^J"); lines.push(format!("\"\"   {}", truncate(&s, 40)));
        } }
        for c in 'a'..='z' {
            if !show(c) { continue; }
            let name = RegisterName::Named(c);
            if let Some(r) = self.registers.get(name) {
                let s = r.content.replace('\n', "^J");
                lines.push(format!("\"{c}   {}", truncate(&s, 40)));
            } else if let Some(keys) = self.macro_store.get(&c) {
                let s: String = keys.iter().map(macro_key_char).collect();
                lines.push(format!("\"{c}   {}", truncate(&s, 40)));
            }
        }
        for i in 0..=9u8 {
            if !show((b'0' + i) as char) { continue; }
            let name = RegisterName::Numbered(i);
            if let Some(r) = self.registers.get(name) {
                let s = r.content.replace('\n', "^J");
                lines.push(format!("\"{i}   {}", truncate(&s, 40)));
            }
        }
        if show('.') { if let Some(r) = self.registers.get(RegisterName::LastInserted) {
            let s = r.content.replace('\n', "^J"); lines.push(format!("\".   {}", truncate(&s, 40)));
        } }
        self.append_readonly_regs_filtered(&mut lines, &show);
        self.notify_info(&lines.join("\n"));
    }

    pub(crate) fn handle_set_command(&mut self, args: &str) {
        match crate::options::parse_set_command(&mut self.options, args) {
            Ok(Some(msg)) => self.notify_info(&msg),
            Ok(None) => {}
            Err(e) => self.notify_error(&e),
        }
    }

    fn append_readonly_regs_filtered(&self, lines: &mut Vec<String>, show: &dyn Fn(char) -> bool) {
        if show('%') { if let Some(p) = self.buffers.get(self.current_buffer_id()).and_then(|b| b.path.as_ref()) { lines.push(format!("\"%   {}", truncate(&p.display().to_string(), 40))); } }
        if show(':') && !self.last_ex_command.is_empty() { lines.push(format!("\":   {}", truncate(&self.last_ex_command, 40))); }
        if show('/') { if let Some(ref pat) = self.search.pattern { if self.search.active { lines.push(format!("\"/   {}", truncate(pat, 40))); } } }
    }

    /// Handle `:call FuncName(args)`. Returns value if `:return` used.
    #[rustfmt::skip]
    pub(crate) fn handle_call_function(&mut self, rest: &str) -> Option<String> {
        let rest = rest.strip_prefix("call ").unwrap_or(rest).trim();
        let paren = match rest.find('(') { Some(i) => i, None => { self.notify_error("E107: Missing parentheses"); return None; } };
        let name = rest[..paren].trim();
        let close = rest.rfind(')').unwrap_or(rest.len());
        let arg_str = rest[paren + 1..close].trim();
        // Autoload resolution: name#func → look up "func" from autoload/name.vim
        let resolved_name = if name.contains('#') { name.rsplit('#').next().unwrap_or(name) } else { name };
        let (body, params) = if let Some(f) = self.functions.get(resolved_name).or_else(|| self.functions.get(name)) { (f.body.clone(), f.params.clone()) } else { self.notify_error(&format!("E117: Unknown function: {name}")); return None; };
        let args: Vec<String> = if arg_str.is_empty() { Vec::new() } else { arg_str.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect() };
        for (i, p) in params.iter().enumerate() { let val = args.get(i).cloned().unwrap_or_default(); self.options.set(&format!("a:{p}"), crate::options::OptionValue::Str(val)); }
        let mut ret_val = None;
        for line in &body {
            let trimmed = line.trim();
            if let Some(expr) = trimmed.strip_prefix("return ").or_else(|| trimmed.strip_prefix("return")) {
                let expr = expr.trim();
                ret_val = Some(if expr.is_empty() { String::new() } else {
                    let opt_val = self.options.get_str(expr).to_string();
                    if !opt_val.is_empty() { opt_val } else { crate::expr_eval::eval_expression(expr).unwrap_or_default() }
                });
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("let ") { self.handle_let_command(rest); continue; }
            self.execute_ex_command(line);
        }
        ret_val
    }

    /// Handle `:let var = expr`. Supports `:let @a = "text"` for register/macro sync.
    #[rustfmt::skip]
    pub(crate) fn handle_let_command(&mut self, args: &str) {
        let args = args.trim();
        let eq = match args.find('=') { Some(i) => i, None => { self.notify_error("E15: Invalid let"); return; } };
        let (var, expr) = (args[..eq].trim(), args[eq + 1..].trim());
        let opt_val = self.options.get_str(expr).to_string();
        let val = if !opt_val.is_empty() { opt_val } else { crate::expr_eval::eval_expression(expr).unwrap_or_default() };
        if var.starts_with('@') && var.len() == 2 { let c = var.as_bytes()[1] as char; if c.is_ascii_lowercase() {
            use kjxlkj_core_edit::{Register, RegisterName};
            self.registers.set(RegisterName::Named(c), Register::new(val, false)); self.sync_register_to_macro(c); return;
        } }
        self.options.set(var, crate::options::OptionValue::Str(val));
    }
}

#[rustfmt::skip]
fn truncate(s: &str, max: usize) -> &str { if s.len() <= max { s } else { &s[..max] } }
#[rustfmt::skip]
fn format_user_cmd_count(n: usize) -> String { if n == 0 { "No user commands defined".into() } else { format!("{n} user command(s) defined") } }

fn macro_key_char(key: &kjxlkj_core_types::Key) -> char {
    match &key.code {
        kjxlkj_core_types::KeyCode::Char(c) => *c,
        kjxlkj_core_types::KeyCode::Enter => '\n',
        kjxlkj_core_types::KeyCode::Esc => '\x1b',
        _ => '?',
    }
}
