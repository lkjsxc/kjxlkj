//! Command-line completion — deterministic completion sources for :command, path, buffer names.

/// A completion candidate with score and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionCandidate {
    pub text: String,
    pub kind: CandidateKind,
    pub score: u32,
}

/// Kind of completion candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateKind { Command, Option, Buffer, Path, User }

impl CompletionCandidate {
    pub fn cmd(text: &str, score: u32) -> Self {
        Self { text: text.into(), kind: CandidateKind::Command, score }
    }
    pub fn opt(text: &str, score: u32) -> Self {
        Self { text: text.into(), kind: CandidateKind::Option, score }
    }
}

/// Built-in command names for command completion.
const BUILTIN_COMMANDS: &[&str] = &[
    "autocmd", "bdelete", "bnext", "bprev", "buffer", "buffers", "cd", "changes",
    "cnext", "copen", "cprev", "delete", "digraphs", "edit", "enew", "execute",
    "exit", "file", "filetype", "global", "highlight", "imap", "jumps", "ls",
    "map", "mapclear", "marks", "messages", "mksession", "move", "new", "nmap",
    "noh", "normal", "oldfiles", "only", "put", "pwd", "quit", "quitall", "read",
    "reg", "registers", "saveas", "scratch", "set", "sort", "source", "split",
    "substitute", "syntax", "unmap", "vglobal", "vnew", "vsplit", "wall", "wq",
    "write", "yank",
];

/// Options for :set completion.
const OPTION_NAMES: &[&str] = &[
    "autoindent", "autopairs", "cursorcolumn", "cursorline", "expandtab",
    "hidden", "hlsearch", "ignorecase", "incsearch", "list", "number",
    "relativenumber", "scrolloff", "shiftwidth", "showcmd", "showmode",
    "sidescrolloff", "smartcase", "smartindent", "tabstop", "wrap",
];

/// Generate command-name completions for a prefix.
pub fn complete_command(prefix: &str) -> Vec<CompletionCandidate> {
    let lower = prefix.to_lowercase();
    let mut results: Vec<_> = BUILTIN_COMMANDS.iter()
        .filter(|c| c.starts_with(&lower))
        .map(|c| {
            let score = if *c == lower { 100 } else if c.starts_with(&lower) { 50 } else { 10 };
            CompletionCandidate::cmd(c, score)
        })
        .collect();
    results.sort_by(|a, b| b.score.cmp(&a.score).then(a.text.cmp(&b.text)));
    results
}

/// Generate :set option completions for a prefix.
pub fn complete_option(prefix: &str) -> Vec<CompletionCandidate> {
    let lower = prefix.to_lowercase();
    OPTION_NAMES.iter()
        .filter(|o| o.starts_with(&lower))
        .map(|o| CompletionCandidate::opt(o, 50))
        .collect()
}

/// Complete buffer names from a list.
pub fn complete_buffer(prefix: &str, buffers: &[&str]) -> Vec<CompletionCandidate> {
    buffers.iter()
        .filter(|b| b.starts_with(prefix) || b.contains(prefix))
        .map(|b| CompletionCandidate { text: b.to_string(), kind: CandidateKind::Buffer, score: 50 })
        .collect()
}

/// Determine completion kind from command-line text.
pub fn detect_completion_kind(cmdline: &str) -> CandidateKind {
    let trimmed = cmdline.trim_start();
    if trimmed.starts_with("set ") || trimmed.starts_with("se ") { return CandidateKind::Option; }
    if trimmed.starts_with("buffer ") || trimmed.starts_with("b ") { return CandidateKind::Buffer; }
    if trimmed.starts_with("edit ") || trimmed.starts_with("e ") { return CandidateKind::Path; }
    CandidateKind::Command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_command_prefix() {
        let r = complete_command("qu");
        assert!(r.iter().any(|c| c.text == "quit"));
        assert!(r.iter().any(|c| c.text == "quitall"));
    }

    #[test]
    fn complete_command_exact() {
        let r = complete_command("quit");
        let quit = r.iter().find(|c| c.text == "quit").unwrap();
        assert_eq!(quit.score, 100);
    }

    #[test]
    fn complete_command_empty() {
        let r = complete_command("zzz");
        assert!(r.is_empty());
    }

    #[test]
    fn complete_option_prefix() {
        let r = complete_option("tab");
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].text, "tabstop");
    }

    #[test]
    fn complete_buffer_names() {
        let bufs = vec!["main.rs", "lib.rs", "mod.rs"];
        let r = complete_buffer("lib", &bufs);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].text, "lib.rs");
    }

    #[test]
    fn detect_kinds() {
        assert_eq!(detect_completion_kind("set tab"), CandidateKind::Option);
        assert_eq!(detect_completion_kind("buffer m"), CandidateKind::Buffer);
        assert_eq!(detect_completion_kind("edit src"), CandidateKind::Path);
        assert_eq!(detect_completion_kind("qu"), CandidateKind::Command);
    }

    #[test]
    fn all_builtins_sorted() {
        for w in BUILTIN_COMMANDS.windows(2) {
            assert!(w[0] <= w[1], "{} should come before {}", w[0], w[1]);
        }
    }

    #[test]
    fn complete_command_all() {
        let r = complete_command("");
        assert_eq!(r.len(), BUILTIN_COMMANDS.len());
    }
}
