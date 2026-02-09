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
    #[rustfmt::skip]
    fn build_buffer_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let partial = &content[content.rfind(' ').unwrap_or(0) + 1..];
        let mut matches: Vec<String> = self.buffers.iter().filter_map(|buf| {
            let name = buf.path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| format!("[{}]", buf.id.0));
            if name.contains(partial) || partial.is_empty() { Some(name) } else { None }
        }).collect();
        if matches.is_empty() { return; }
        matches.sort();
        let cs = &mut self.cmdline.completion;
        cs.prefix = content; cs.candidates = matches; cs.index = Some(0);
    }

    /// Build file-path candidates from the path prefix.
    pub(crate) fn build_file_candidates(&mut self) {
        let content = self.cmdline.content.clone();
        let space = content.rfind(' ').unwrap_or(0) + 1;
        let partial = &content[space..];
        // Expand ~ to HOME.
        let expanded;
        let partial = if let Some(rest) = partial.strip_prefix('~') {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            expanded = format!("{}{}", home, rest);
            &expanded
        } else {
            partial
        };
        // Glob pattern expansion: if partial contains * or ?, expand as glob.
        if partial.contains('*') || partial.contains('?') {
            self.build_glob_candidates(&content, partial);
            return;
        }
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
                let path = if dir == "./" { name.clone() } else { format!("{}{}", dir, name) };
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) { matches.push(format!("{}/", path)); }
                else { matches.push(path); }
            }
        }
        // Fuzzy fallback: if no prefix matches, score all entries by subsequence match.
        if matches.is_empty() && !prefix.is_empty() {
            let mut scored: Vec<(i32, String)> = Vec::new();
            if let Ok(ents) = std::fs::read_dir(dir) {
                for entry in ents.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if let Some(sc) = fuzzy_score(prefix, &name) {
                        let path = if dir == "./" { name } else { format!("{}{}", dir, name) };
                        scored.push((sc, path));
                    }
                }
            }
            scored.sort_by(|a, b| b.0.cmp(&a.0));
            matches = scored.into_iter().map(|(_, p)| p).collect();
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

    /// Build candidates from glob pattern (e.g., *.rs, src/**/*.ts).
    fn build_glob_candidates(&mut self, content: &str, pattern: &str) {
        let (dir, glob_pat) = if let Some(slash) = pattern.rfind('/') {
            (&pattern[..=slash], &pattern[slash + 1..])
        } else {
            ("./", pattern)
        };
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        let re_pat = glob_pat.replace('.', "\\.").replace('*', ".*").replace('?', ".");
        let re = match regex::Regex::new(&format!("^{re_pat}$")) {
            Ok(r) => r,
            Err(_) => return,
        };
        let mut matches: Vec<String> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if re.is_match(&name) {
                let path = if dir == "./" { name } else { format!("{}{}", dir, name) };
                matches.push(path);
            }
        }
        matches.sort();
        if matches.is_empty() { return; }
        let cs = &mut self.cmdline.completion;
        cs.prefix = content.to_string();
        cs.candidates = matches;
        cs.index = Some(0);
    }
}
/// Fuzzy subsequence score: higher = better. Returns None if no subsequence match.
#[rustfmt::skip]
pub fn fuzzy_score(query: &str, target: &str) -> Option<i32> {
    let (mut qi, qb, tb) = (0usize, query.as_bytes(), target.as_bytes());
    let (mut score, ql) = (0i32, qb.len());
    for (i, &b) in tb.iter().enumerate() {
        if qi < ql && b.eq_ignore_ascii_case(&qb[qi]) { score += if i == 0 || tb[i-1] == b'_' || tb[i-1] == b'/' { 3 } else { 1 }; qi += 1; }
    }
    if qi == ql { Some(score) } else { None }
}
