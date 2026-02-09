//! Range command dispatch helpers: parse command strings
//! with range prefixes and delegate to range commands.

use crate::editor_range_parse::parse_range;
use crate::EditorState;

impl EditorState {
    /// Dispatch `:1,5d` or `:%d` etc.
    pub(crate) fn dispatch_range_delete(
        &mut self,
        full_cmd: &str,
    ) {
        let (cl, ll) = self.current_last_line();
        let cleaned = strip_cmd_name(full_cmd, "d");
        let (s, e, _) = parse_range(&cleaned, cl, ll);
        self.do_range_delete(s, e);
    }

    /// Dispatch `:1,5y` etc.
    pub(crate) fn dispatch_range_yank(
        &mut self,
        full_cmd: &str,
    ) {
        let (cl, ll) = self.current_last_line();
        let cleaned = strip_cmd_name(full_cmd, "y");
        let (s, e, _) = parse_range(&cleaned, cl, ll);
        self.do_range_yank(s, e);
    }

    /// Dispatch `:1,5t10` etc.
    pub(crate) fn dispatch_range_copy(
        &mut self,
        full_cmd: &str,
    ) {
        let (cl, ll) = self.current_last_line();
        let cleaned = strip_cmd_name(full_cmd, "t");
        let (s, e, rest) =
            parse_range(&cleaned, cl, ll);
        let dest = rest
            .trim()
            .parse::<usize>()
            .map(|n| n.saturating_sub(1))
            .unwrap_or(cl);
        self.do_range_copy(s, e, dest);
    }

    /// Dispatch `:1,5m10` etc.
    pub(crate) fn dispatch_range_move(
        &mut self,
        full_cmd: &str,
    ) {
        let (cl, ll) = self.current_last_line();
        let cleaned = strip_cmd_name(full_cmd, "m");
        let (s, e, rest) =
            parse_range(&cleaned, cl, ll);
        let dest = rest
            .trim()
            .parse::<usize>()
            .map(|n| n.saturating_sub(1))
            .unwrap_or(cl);
        self.do_range_move(s, e, dest);
    }

    /// Dispatch `:1,5normal @a` etc.
    pub(crate) fn dispatch_range_normal(
        &mut self,
        full_cmd: &str,
    ) {
        let (cl, ll) = self.current_last_line();
        let cleaned =
            strip_cmd_name(full_cmd, "normal");
        let (s, e, rest) =
            parse_range(&cleaned, cl, ll);
        self.do_range_normal(s, e, &rest);
    }

    /// Read file contents into buffer after cursor line.
    pub(crate) fn do_read_file(
        &mut self,
        path: &str,
    ) {
        let path = path.trim();
        if path.is_empty() {
            return;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let (line, _) = self.cursor_pos();
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let insert_line = line + 1;
            let off = if insert_line >= buf.line_count() {
                buf.content.len_chars()
            } else {
                buf.content.line_start_offset(insert_line)
            };
            // Ensure content ends with newline
            let text = if content.ends_with('\n') {
                content
            } else {
                format!("{}\n", content)
            };
            for (i, ch) in text.chars().enumerate() {
                buf.content.insert_char(off + i, ch);
            }
            buf.modified = true;
        }
    }

    fn current_last_line(&self) -> (usize, usize) {
        let cl = self.cursor_pos().0;
        let ll = self
            .active_buffer()
            .map(|b| b.line_count().saturating_sub(1))
            .unwrap_or(0);
        (cl, ll)
    }
}

/// Strip the command name from a full command string,
/// leaving only the range prefix and arguments.
fn strip_cmd_name(
    cmd: &str,
    name: &str,
) -> String {
    let cmd = cmd.trim();
    // Skip the range prefix: digits, commas, %, $, .
    let mut pos = 0;
    let bytes = cmd.as_bytes();
    while pos < bytes.len() {
        let b = bytes[pos];
        if b.is_ascii_digit()
            || b == b','
            || b == b'%'
            || b == b'$'
            || b == b'.'
            || b == b'\''
            || b == b' '
        {
            pos += 1;
        } else {
            break;
        }
    }
    // Now at pos we expect the command name
    if cmd[pos..].starts_with(name) {
        let range_part = &cmd[..pos];
        let after = &cmd[pos + name.len()..];
        format!(
            "{}{}",
            range_part.trim_end(),
            if after.is_empty() {
                String::new()
            } else {
                format!(" {}", after.trim_start())
            },
        )
    } else {
        cmd.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_cmd_name_works() {
        assert_eq!(strip_cmd_name("1,5d", "d"), "1,5");
        assert_eq!(
            strip_cmd_name("%d", "d"),
            "%"
        );
        assert_eq!(
            strip_cmd_name("3,7t 10", "t"),
            "3,7 10"
        );
    }
}
