//! Comment toggling: toggle line comments for the
//! current line or selection.

use crate::EditorState;

/// Maps file type to its line comment prefix.
fn comment_prefix(file_type: &str) -> Option<&'static str> {
    match file_type {
        "rust" | "rs" => Some("// "),
        "javascript" | "js" | "typescript" | "ts"
        | "c" | "cpp" | "java" | "go" | "swift"
        | "kotlin" | "scala" | "dart" => Some("// "),
        "python" | "py" | "ruby" | "rb" | "bash"
        | "sh" | "zsh" | "fish" | "toml" | "yaml"
        | "yml" | "conf" | "perl" | "elixir" => {
            Some("# ")
        }
        "lua" | "haskell" | "hs" | "sql" => {
            Some("-- ")
        }
        "vim" | "vimscript" => Some("\" "),
        "lisp" | "clojure" | "scheme" => Some(";; "),
        "html" | "xml" => None,
        _ => Some("// "),
    }
}

impl EditorState {
    /// Toggle line comment on the current line.
    pub(crate) fn toggle_comment_line(
        &mut self,
    ) {
        let (line, _) = self.cursor_pos();
        let ft = self
            .active_buffer()
            .map(|b| b.file_type.clone())
            .unwrap_or_default();
        let prefix = match comment_prefix(&ft) {
            Some(p) => p,
            None => return,
        };
        let line_text = match self
            .active_buffer()
            .map(|b| b.content.line_content(line))
        {
            Some(t) => t,
            None => return,
        };
        let trimmed = line_text.trim_start();
        if trimmed.starts_with(prefix.trim_end()) {
            // Remove comment prefix.
            self.uncomment_line(line, prefix);
        } else {
            // Add comment prefix.
            self.comment_line(line, prefix);
        }
    }

    fn comment_line(
        &mut self,
        line: usize,
        prefix: &str,
    ) {
        if let Some(buf) = self.active_buffer_mut()
        {
            let line_text =
                buf.content.line_content(line);
            let indent_len = line_text.len()
                - line_text.trim_start().len();
            let off = buf
                .content
                .line_start_offset(line)
                + indent_len;
            for (i, ch) in
                prefix.chars().enumerate()
            {
                buf.content.insert_char(off + i, ch);
            }
            buf.modified = true;
        }
    }

    fn uncomment_line(
        &mut self,
        line: usize,
        prefix: &str,
    ) {
        if let Some(buf) = self.active_buffer_mut()
        {
            let line_text =
                buf.content.line_content(line);
            let indent_len = line_text.len()
                - line_text.trim_start().len();
            let trimmed_prefix =
                prefix.trim_end();
            // Check if it has exact prefix with space.
            let remove_len = if line_text
                [indent_len..]
                .starts_with(prefix)
            {
                prefix.len()
            } else if line_text[indent_len..]
                .starts_with(trimmed_prefix)
            {
                trimmed_prefix.len()
            } else {
                return;
            };
            let off = buf
                .content
                .line_start_offset(line)
                + indent_len;
            buf.content
                .delete_range(off, off + remove_len);
            buf.modified = true;
        }
    }

    /// Toggle comments on a range of lines.
    pub(crate) fn toggle_comment_range(
        &mut self,
        start: usize,
        end: usize,
    ) {
        for line in start..=end {
            // Simplified: toggle each line individually.
            let ft = self
                .active_buffer()
                .map(|b| b.file_type.clone())
                .unwrap_or_default();
            let prefix = match comment_prefix(&ft) {
                Some(p) => p,
                None => return,
            };
            let line_text = match self
                .active_buffer()
                .map(|b| b.content.line_content(line))
            {
                Some(t) => t,
                None => continue,
            };
            let trimmed = line_text.trim_start();
            if trimmed.starts_with(
                prefix.trim_end(),
            ) {
                self.uncomment_line(line, prefix);
            } else {
                self.comment_line(line, prefix);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_prefix_rust() {
        assert_eq!(comment_prefix("rust"), Some("// "));
    }

    #[test]
    fn comment_prefix_python() {
        assert_eq!(
            comment_prefix("python"),
            Some("# ")
        );
    }

    #[test]
    fn toggle_comment() {
        let mut state =
            crate::EditorState::new(80, 24);
        // Insert some text first.
        state.insert_char('h');
        state.insert_char('i');
        state.mode = kjxlkj_core_types::Mode::Normal;
        state.toggle_comment_line();
        let buf = state.active_buffer().unwrap();
        let line = buf.content.line_content(0);
        assert!(line.contains("// "));
    }
}
