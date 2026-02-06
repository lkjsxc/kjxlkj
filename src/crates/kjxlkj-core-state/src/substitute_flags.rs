//! Substitute command flag parsing and confirmation mode.

/// Parsed substitute flags from `:s/pat/rep/flags`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstituteFlags {
    pub global: bool,
    pub confirm: bool,
    pub ignore_case: bool,
    pub no_ignore_case: bool,
    pub count_only: bool,
    pub repeat: bool,
}

impl SubstituteFlags {
    /// Parse flags string (e.g., "gc", "gI", "n").
    pub fn parse(flags: &str) -> Self {
        let mut f = Self::default();
        for c in flags.chars() {
            match c {
                'g' => f.global = true,
                'c' => f.confirm = true,
                'i' => f.ignore_case = true,
                'I' => f.no_ignore_case = true,
                'n' => f.count_only = true,
                '&' => f.repeat = true,
                _ => {}
            }
        }
        f
    }

    pub fn is_case_sensitive(&self, global_ignorecase: bool) -> bool {
        if self.no_ignore_case { return true; }
        if self.ignore_case { return false; }
        !global_ignorecase
    }
}

impl Default for SubstituteFlags {
    fn default() -> Self {
        Self { global: false, confirm: false, ignore_case: false,
               no_ignore_case: false, count_only: false, repeat: false }
    }
}

/// State for interactive `:s///c` confirmation.
#[derive(Debug, Clone)]
pub struct ConfirmState {
    pub pattern: String,
    pub replacement: String,
    pub remaining_lines: Vec<usize>,
    pub current_col: usize,
    pub replaced: usize,
    pub skipped: usize,
}

impl ConfirmState {
    pub fn new(pattern: &str, replacement: &str, lines: Vec<usize>) -> Self {
        Self { pattern: pattern.into(), replacement: replacement.into(),
               remaining_lines: lines, current_col: 0, replaced: 0, skipped: 0 }
    }

    /// Process user response: y=replace, n=skip, a=all, q=quit.
    pub fn respond(&mut self, response: ConfirmResponse) -> ConfirmAction {
        match response {
            ConfirmResponse::Yes => { self.replaced += 1; self.advance() }
            ConfirmResponse::No => { self.skipped += 1; self.advance() }
            ConfirmResponse::All => { self.replaced += self.remaining_lines.len(); ConfirmAction::ReplaceAll }
            ConfirmResponse::Quit => ConfirmAction::Done,
        }
    }

    fn advance(&mut self) -> ConfirmAction {
        self.remaining_lines.pop();
        if self.remaining_lines.is_empty() { ConfirmAction::Done } else { ConfirmAction::Next }
    }

    pub fn is_done(&self) -> bool { self.remaining_lines.is_empty() }

    pub fn summary(&self) -> String {
        format!("{} substitutions on {} lines ({} skipped)", self.replaced,
                self.replaced + self.skipped, self.skipped)
    }
}

/// User response in confirm mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResponse { Yes, No, All, Quit }

/// Action to take after confirm response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction { Next, ReplaceAll, Done }

/// Parse a full substitute command into (pattern, replacement, flags).
pub fn parse_substitute_cmd(cmd: &str) -> Option<(&str, &str, SubstituteFlags)> {
    let rest = cmd.strip_prefix(":s").or_else(|| cmd.strip_prefix(":substitute"))?;
    if rest.is_empty() { return None; }
    let sep = rest.chars().next()?;
    if sep.is_alphanumeric() { return None; }
    let parts: Vec<&str> = rest[sep.len_utf8()..].splitn(3, sep).collect();
    if parts.len() < 2 { return None; }
    let flags = SubstituteFlags::parse(parts.get(2).unwrap_or(&""));
    Some((parts[0], parts[1], flags))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_flags() {
        let f = SubstituteFlags::parse("gc");
        assert!(f.global && f.confirm);
        assert!(!f.count_only);
    }

    #[test]
    fn case_sensitivity() {
        let f = SubstituteFlags::parse("I");
        assert!(f.is_case_sensitive(true));
        let f2 = SubstituteFlags::parse("i");
        assert!(!f2.is_case_sensitive(false));
        let f3 = SubstituteFlags::default();
        assert!(!f3.is_case_sensitive(true)); // follows global
    }

    #[test]
    fn parse_cmd_basic() {
        let (pat, rep, flags) = parse_substitute_cmd(":s/foo/bar/g").unwrap();
        assert_eq!(pat, "foo");
        assert_eq!(rep, "bar");
        assert!(flags.global);
    }

    #[test]
    fn parse_cmd_pipe_sep() {
        let (pat, rep, _) = parse_substitute_cmd(":s|old|new|").unwrap();
        assert_eq!(pat, "old");
        assert_eq!(rep, "new");
    }

    #[test]
    fn parse_cmd_count_only() {
        let (_, _, flags) = parse_substitute_cmd(":s/x/y/n").unwrap();
        assert!(flags.count_only);
    }

    #[test]
    fn confirm_yes_then_done() {
        let mut cs = ConfirmState::new("a", "b", vec![3, 1]);
        assert_eq!(cs.respond(ConfirmResponse::Yes), ConfirmAction::Next);
        assert_eq!(cs.respond(ConfirmResponse::Yes), ConfirmAction::Done);
        assert_eq!(cs.replaced, 2);
    }

    #[test]
    fn confirm_skip() {
        let mut cs = ConfirmState::new("x", "y", vec![1]);
        assert_eq!(cs.respond(ConfirmResponse::No), ConfirmAction::Done);
        assert_eq!(cs.skipped, 1);
    }

    #[test]
    fn confirm_all() {
        let mut cs = ConfirmState::new("x", "y", vec![5, 3, 1]);
        assert_eq!(cs.respond(ConfirmResponse::All), ConfirmAction::ReplaceAll);
        assert_eq!(cs.replaced, 3);
    }

    #[test]
    fn confirm_quit() {
        let mut cs = ConfirmState::new("x", "y", vec![2, 1]);
        assert_eq!(cs.respond(ConfirmResponse::Quit), ConfirmAction::Done);
    }

    #[test]
    fn summary_format() {
        let mut cs = ConfirmState::new("a", "b", vec![2, 1]);
        cs.respond(ConfirmResponse::Yes);
        cs.respond(ConfirmResponse::No);
        assert!(cs.summary().contains("1 substitutions"));
    }
}
