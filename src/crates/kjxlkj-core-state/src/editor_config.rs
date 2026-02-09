//! Editor configuration system.

use crate::editor::EditorState;
use crate::mappings::{
    MappingMode, parse_mapping_cmd,
};

impl EditorState {
    /// Handle a `:map`/`:nmap`/`:nnoremap`/etc command.
    pub fn do_map_command(
        &mut self,
        cmd: &str,
        args: &str,
    ) {
        if let Some(mapping) =
            parse_mapping_cmd(cmd, args)
        {
            self.mappings.add(mapping);
        }
    }

    /// Handle an unmap command.
    pub fn do_unmap_command(
        &mut self,
        cmd: &str,
        lhs: &str,
    ) {
        let mode = match cmd {
            "nunmap" => MappingMode::Normal,
            "iunmap" => MappingMode::Insert,
            "vunmap" => MappingMode::Visual,
            "cunmap" => MappingMode::Command,
            "tunmap" => MappingMode::Terminal,
            "ounmap" => MappingMode::OperatorPending,
            _ => return,
        };
        self.mappings.remove(lhs, mode);
    }

    /// Handle `:source {file}` command.
    pub fn do_source_file(&mut self, path: &str) {
        let content =
            match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => return,
            };
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with('"')
            {
                continue;
            }
            if let Some(a) =
                crate::dispatch_command(trimmed)
            {
                self.dispatch(a);
            }
        }
    }

    /// Handle `:set` command.
    pub fn do_set_option(&mut self, args: &str) {
        let args = args.trim();
        if args.is_empty() {
            return;
        }
        if let Some((key, val)) =
            args.split_once('=')
        {
            match key.trim() {
                "number" | "nu" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.number =
                            val.trim() != "0"
                                && val.trim() != "false";
                    }
                }
                "relativenumber" | "rnu" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.relative_number =
                            val.trim() != "0"
                                && val.trim() != "false";
                    }
                }
                "wrap" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.wrap =
                            val.trim() != "0"
                                && val.trim() != "false";
                    }
                }
                "scrolloff" | "so" => {
                    if let Ok(n) =
                        val.trim().parse::<u16>()
                    {
                        if let Some(w) =
                            self.focused_window_mut()
                        {
                            w.options.scroll_off = n;
                        }
                    }
                }
                _ => {}
            }
        } else {
            // Boolean options: `:set number`
            let (negated, name) = if let Some(n) =
                args.strip_prefix("no")
            {
                (true, n)
            } else {
                (false, args)
            };
            match name {
                "number" | "nu" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.number = !negated;
                    }
                }
                "relativenumber" | "rnu" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.relative_number =
                            !negated;
                    }
                }
                "wrap" => {
                    if let Some(w) =
                        self.focused_window_mut()
                    {
                        w.options.wrap = !negated;
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_number() {
        let mut ed = EditorState::new(80, 24);
        ed.do_set_option("number");
        let w = ed.focused_window().unwrap();
        assert!(w.options.number);
        ed.do_set_option("nonumber");
        let w = ed.focused_window().unwrap();
        assert!(!w.options.number);
    }

    #[test]
    fn set_scrolloff_value() {
        let mut ed = EditorState::new(80, 24);
        ed.do_set_option("scrolloff=5");
        let w = ed.focused_window().unwrap();
        assert_eq!(w.options.scroll_off, 5);
    }

    #[test]
    fn map_command() {
        let mut ed = EditorState::new(80, 24);
        ed.do_map_command("nnoremap", "jk <Esc>");
        let found = ed.mappings.find(
            "jk",
            MappingMode::Normal,
        );
        assert!(found.is_some());
    }
}
