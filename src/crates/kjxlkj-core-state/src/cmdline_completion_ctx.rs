//! File-path and context-sensitive argument completion.
//!
//! Extracted from cmdline_completion.rs to respect the
//! 200-line file size cap.
use crate::editor::EditorState;

impl EditorState {
    /// Dispatch argument completion by command context.
    pub(crate) fn build_arg_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let cmd = content.split_whitespace().next().unwrap_or("");
        match cmd {
            "set" => self.build_option_candidates(),
            "b" | "buffer" | "bdelete" => self.build_buffer_candidates(),
            "mark" | "delmarks" => self.build_mark_candidates(),
            "reg" | "registers" => self.build_register_candidates(),
            "help" => self.build_help_candidates(),
            _ => self.build_file_candidates(),
        }
    }

    /// Build mark name candidates for :mark/:delmarks completion.
    fn build_mark_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        let mut matches: Vec<String> = Vec::new();
        for c in 'a'..='z' { let s = c.to_string(); if s.starts_with(partial) || partial.is_empty() { matches.push(s); } }
        for c in 'A'..='Z' { let s = c.to_string(); if s.starts_with(partial) || partial.is_empty() { matches.push(s); } }
        if matches.is_empty() { return; }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
    }

    /// Build register name candidates for :reg completion.
    fn build_register_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let mut matches: Vec<String> = Vec::new();
        matches.push("\"".to_string());
        for c in 'a'..='z' { matches.push(c.to_string()); }
        for i in 0..=9u8 { matches.push(i.to_string()); }
        for s in &[".", "%", "#", ":", "/"] { matches.push(s.to_string()); }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
    }

    /// Build help topic candidates.
    fn build_help_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        let topics = ["insert", "normal", "visual", "ex", "pattern", "map", "set", "autocmd", "function", "syntax", "options", "registers", "marks", "motion", "change", "undo"];
        let matches: Vec<String> = topics.iter().filter(|t| t.starts_with(partial) || partial.is_empty()).map(|t| t.to_string()).collect();
        if matches.is_empty() { return; }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
    }

    /// Build option-name candidates for :set completion.
    fn build_option_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        // Filetype completion: "set filetype=" or "set ft="
        if partial.starts_with("filetype=") || partial.starts_with("ft=") {
            let eq = partial.find('=').unwrap_or(0) + 1;
            let prefix = &partial[eq..];
            let fts = ["c", "cpp", "css", "go", "html", "java", "javascript", "json", "lua", "markdown", "perl", "php", "python", "ruby", "rust", "sh", "sql", "toml", "typescript", "vim", "xml", "yaml", "zig"];
            let matches: Vec<String> = fts.iter().filter(|f| f.starts_with(prefix) || prefix.is_empty()).map(|f| format!("{}={}", &partial[..eq], f)).collect();
            if matches.is_empty() { return; }
            let cs = &mut self.cmdline.completion;
            cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
            return;
        }
        let names = self.options.list();
        let matches: Vec<String> = names.iter().map(|(k, _)| k.clone()).filter(|k| k.starts_with(partial)).collect();
        if matches.is_empty() { return; }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
    }

    /// Build buffer-name candidates for :b completion.
    fn build_buffer_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        let mut matches: Vec<String> = Vec::new();
        for buf in self.buffers.iter() {
            let name = buf
                .path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| format!("[{}]", buf.id.0));
            if name.contains(partial) || partial.is_empty() {
                matches.push(name);
            }
        }
        if matches.is_empty() {
            return;
        }
        matches.sort();
        let cs = &mut self.cmdline.completion;
        cs.prefix = content;
        cs.candidates = matches;
        cs.index = Some(0);
    }

    /// Build file-path candidates from the path prefix.
    pub(crate) fn build_file_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        let (dir, prefix) = if let Some(slash) = partial.rfind('/') {
            (&partial[..=slash], &partial[slash + 1..])
        } else {
            ("./", partial)
        };
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        let mut matches: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(prefix) {
                let path = if dir == "./" {
                    name.clone()
                } else {
                    format!("{}{}", dir, name)
                };
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    matches.push(format!("{}/", path));
                } else {
                    matches.push(path);
                }
            }
        }
        matches.sort();
        if matches.is_empty() {
            return;
        }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content;
        cs.candidates = matches;
        cs.index = Some(0);
    }
}
