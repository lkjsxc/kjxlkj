/// Scripting-related ex commands: user commands, autocmd, marks, registers.
use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn handle_command_command(&mut self, prefix: &str, args: &str) {
        use crate::user_commands_parse::parse_command_def;

        if prefix == "comclear" {
            self.user_commands.clear();
            self.notify_info("All user commands cleared");
            return;
        }

        if args.is_empty() {
            let cmds = self.user_commands.list();
            if cmds.is_empty() {
                self.notify_info("No user commands defined");
            } else {
                let count = cmds.len();
                self.notify_info(&format!("{count} user command(s) defined"));
            }
            return;
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
            let count = self.events.len();
            self.notify_info(&format!("{count} autocmd(s) registered"));
            return;
        }

        let parts: Vec<&str> = args.splitn(3, ' ').collect();
        if parts.len() < 2 {
            self.notify_error("E471: Usage: autocmd {event} {pattern} {cmd}");
            return;
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
            _ => {
                self.notify_error(&format!("E216: Unknown event: {event_name}"));
                return;
            }
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
        use crate::marks::MarkPosition;

        let cursor = self.windows.focused().cursor;
        let buf_id = self.current_buffer_id();
        self.marks.set(
            name,
            MarkPosition {
                buffer_id: buf_id.0 as usize,
                line: cursor.line,
                col: cursor.grapheme,
            },
        );
        self.notify_info(&format!("Mark '{name}' set"));
    }

    pub(crate) fn handle_delmarks(&mut self, names: &str) {
        let buf_id = self.current_buffer_id().0 as usize;
        for ch in names.chars() {
            if ch.is_alphanumeric() {
                self.marks.delete(ch, buf_id);
            }
        }
        self.notify_info(&format!("Marks deleted: {names}"));
    }

    pub(crate) fn handle_list_marks(&mut self) {
        let buf_id = self.current_buffer_id().0 as usize;
        let marks = self.marks.list_for_buffer(buf_id);
        if marks.is_empty() {
            self.notify_info("No marks set");
        } else {
            let count = marks.len();
            self.notify_info(&format!("{count} mark(s) set"));
        }
    }

    pub(crate) fn handle_list_registers(&mut self) {
        self.notify_info("Registers: (use :reg to view)");
    }
}
