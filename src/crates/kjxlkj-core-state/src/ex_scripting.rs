/// Scripting-related ex commands: user commands, autocmd, marks, registers.
use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn handle_command_command(&mut self, prefix: &str, args: &str) {
        use crate::user_commands_parse::parse_command_def;
        if prefix == "comclear" {
            self.user_commands.clear();
            return self.notify_info("All user commands cleared");
        }
        if args.is_empty() {
            let n = self.user_commands.list().len();
            return self.notify_info(&format_user_cmd_count(n));
        }
        let overwrite = prefix == "command!";
        let input = if overwrite {
            format!("! {args}")
        } else {
            args.to_string()
        };
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
        match self.user_commands.remove(name) {
            Ok(()) => self.notify_info(&format!("Command removed: {name}")),
            Err(e) => self.notify_error(&e),
        }
    }

    pub(crate) fn handle_autocmd(&mut self, args: &str) {
        use crate::events_types::EventKind;
        if args.is_empty() {
            return self.notify_info(&format!("{} autocmd(s) registered", self.events.len()));
        }
        let parts: Vec<&str> = args.splitn(3, ' ').collect();
        if parts.len() < 2 {
            return self.notify_error("E471: Usage: autocmd {event} {pattern} {cmd}");
        }
        let event_name = parts[0];
        let event = match event_name {
            "BufNew" | "BufferNew" => EventKind::BufferNew,
            "BufRead" | "BufferRead" => EventKind::BufferRead,
            "BufWritePre" | "BufferWritePre" => EventKind::BufferWritePre,
            "BufWrite" | "BufferWrite" => EventKind::BufferWrite,
            "BufWritePost" | "BufferWritePost" => EventKind::BufferWritePost,
            "BufEnter" | "BufferEnter" => EventKind::BufferEnter,
            "BufLeave" | "BufferLeave" => EventKind::BufferLeave,
            "BufDelete" | "BufferDelete" => EventKind::BufferDelete,
            "WinNew" | "WindowNew" => EventKind::WindowNew,
            "WinClosed" | "WindowClosed" => EventKind::WindowClosed,
            "WinEnter" | "WindowEnter" => EventKind::WindowEnter,
            "WinLeave" | "WindowLeave" => EventKind::WindowLeave,
            "ModeChanged" => EventKind::ModeChanged,
            "InsertEnter" => EventKind::InsertEnter,
            "InsertLeave" => EventKind::InsertLeave,
            "CursorMoved" => EventKind::CursorMoved,
            "CursorHold" => EventKind::CursorHold,
            "FileType" => EventKind::FileType,
            "ExitPre" => EventKind::ExitPre,
            _ => return self.notify_error(&format!("E216: Unknown event: {event_name}")),
        };
        let (pattern, command) = if parts.len() == 3 {
            (Some(parts[1].to_string()), parts[2].to_string())
        } else {
            (None, parts[1].to_string())
        };
        self.events.register(event, command, pattern, None);
        self.notify_info(&format!("Autocmd registered for {event_name}"));
    }

    pub(crate) fn handle_set_mark(&mut self, name: char) {
        let c = self.windows.focused().cursor;
        let bid = self.current_buffer_id().0 as usize;
        self.marks.set(
            name,
            crate::marks::MarkPosition {
                buffer_id: bid,
                line: c.line,
                col: c.grapheme,
            },
        );
        self.notify_info(&format!("Mark '{name}' set"));
    }

    pub(crate) fn handle_delmarks(&mut self, names: &str) {
        let bid = self.current_buffer_id().0 as usize;
        for ch in names.chars() {
            if ch.is_alphanumeric() {
                self.marks.delete(ch, bid);
            }
        }
        self.notify_info(&format!("Marks deleted: {names}"));
    }

    pub(crate) fn handle_list_marks(&mut self) {
        let bid = self.current_buffer_id().0 as usize;
        let marks = self.marks.list_for_buffer(bid);
        let msg = if marks.is_empty() {
            "No marks set".into()
        } else {
            format!("{} mark(s) set", marks.len())
        };
        self.notify_info(&msg);
    }

    pub(crate) fn handle_list_registers(&mut self) {
        use kjxlkj_core_edit::RegisterName;
        let mut lines = Vec::new();
        lines.push("--- Registers ---".to_string());
        // Show unnamed.
        if let Some(r) = self.registers.get_unnamed() {
            let s = r.content.replace('\n', "^J");
            lines.push(format!("\"\"   {}", truncate(&s, 40)));
        }
        // Named a-z.
        for c in 'a'..='z' {
            let name = RegisterName::Named(c);
            if let Some(r) = self.registers.get(name) {
                let s = r.content.replace('\n', "^J");
                lines.push(format!("\"{c}   {}", truncate(&s, 40)));
            } else if let Some(keys) = self.macro_store.get(&c) {
                let s: String = keys.iter().map(macro_key_char).collect();
                lines.push(format!("\"{c}   {}", truncate(&s, 40)));
            }
        }
        // Numbered 0-9.
        for i in 0..=9u8 {
            let name = RegisterName::Numbered(i);
            if let Some(r) = self.registers.get(name) {
                let s = r.content.replace('\n', "^J");
                lines.push(format!("\"{i}   {}", truncate(&s, 40)));
            }
        }
        // Last-inserted text (. register).
        if let Some(r) = self.registers.get(RegisterName::LastInserted) {
            let s = r.content.replace('\n', "^J");
            lines.push(format!("\".   {}", truncate(&s, 40)));
        }
        // Read-only registers.
        self.append_readonly_regs(&mut lines);
        self.notify_info(&lines.join("\n"));
    }

    pub(crate) fn handle_set_command(&mut self, args: &str) {
        match crate::options::parse_set_command(&mut self.options, args) {
            Ok(Some(msg)) => self.notify_info(&msg),
            Ok(None) => {}
            Err(e) => self.notify_error(&e),
        }
    }

    fn append_readonly_regs(&self, lines: &mut Vec<String>) {
        let pct = self
            .buffers
            .get(self.current_buffer_id())
            .and_then(|b| b.path.as_ref())
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        if !pct.is_empty() {
            lines.push(format!("\"%   {}", truncate(&pct, 40)));
        }
        if !self.last_ex_command.is_empty() {
            lines.push(format!("\":   {}", truncate(&self.last_ex_command, 40)));
        }
        if let Some(ref pat) = self.search.pattern {
            if self.search.active {
                lines.push(format!("\"/   {}", truncate(pat, 40)));
            }
        }
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
