//! Snippet session state machine.
//! See /docs/spec/modes/insert/completion/insert-snippets.md.

/// A tabstop in a snippet template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tabstop {
    /// Tabstop index (0 = final position).
    pub index: usize,
    /// Byte offset in the expanded text where this tabstop sits.
    pub offset: usize,
    /// Default/placeholder text length (0 if none).
    pub placeholder_len: usize,
}

/// Active snippet editing session.
#[derive(Debug, Clone)]
pub struct SnippetSession {
    /// Expanded body text (with placeholders filled).
    body: String,
    /// Ordered tabstops (sorted by index, $0 last).
    tabstops: Vec<Tabstop>,
    /// Current tabstop position in the tabstops vec.
    current: usize,
    /// Whether the session is active.
    active: bool,
}

impl SnippetSession {
    /// Parse a snippet body and create a session.
    /// Supported syntax: `$1`, `$0`, `${1:placeholder}`.
    pub fn parse(body: &str) -> Self {
        let mut expanded = String::new();
        let mut tabstops = Vec::new();
        let mut chars = body.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$' {
                if let Some(&'{') = chars.peek() {
                    chars.next(); // consume '{'
                    let idx = Self::read_number(&mut chars);
                    let mut placeholder = String::new();
                    if chars.peek() == Some(&':') {
                        chars.next(); // consume ':'
                        let mut depth = 1u32;
                        while let Some(&pc) = chars.peek() {
                            if pc == '}' { depth -= 1; if depth == 0 { break; } }
                            if pc == '{' { depth += 1; }
                            placeholder.push(pc);
                            chars.next();
                        }
                    }
                    if chars.peek() == Some(&'}') { chars.next(); }
                    let offset = expanded.len();
                    expanded.push_str(&placeholder);
                    tabstops.push(Tabstop { index: idx, offset, placeholder_len: placeholder.len() });
                } else if chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    let idx = Self::read_number(&mut chars);
                    tabstops.push(Tabstop { index: idx, offset: expanded.len(), placeholder_len: 0 });
                } else {
                    expanded.push('$');
                }
            } else {
                expanded.push(c);
            }
        }
        // Sort: $1, $2, ... then $0 last.
        tabstops.sort_by_key(|t| if t.index == 0 { usize::MAX } else { t.index });
        let active = !tabstops.is_empty();
        Self { body: expanded, tabstops, current: 0, active }
    }

    fn read_number(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> usize {
        let mut n = 0usize;
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() { n = n * 10 + (c as usize - '0' as usize); chars.next(); }
            else { break; }
        }
        n
    }

    pub fn is_active(&self) -> bool { self.active }
    pub fn body(&self) -> &str { &self.body }
    pub fn tabstops(&self) -> &[Tabstop] { &self.tabstops }
    pub fn current_tabstop(&self) -> Option<&Tabstop> { self.tabstops.get(self.current) }

    /// Jump to next tabstop. Returns true if moved, false if at end.
    pub fn next(&mut self) -> bool {
        if !self.active || self.current + 1 >= self.tabstops.len() {
            self.active = false;
            return false;
        }
        self.current += 1;
        true
    }

    /// Jump to previous tabstop. Returns true if moved.
    pub fn prev(&mut self) -> bool {
        if !self.active || self.current == 0 { return false; }
        self.current -= 1;
        true
    }

    /// Cancel snippet session.
    pub fn cancel(&mut self) { self.active = false; }

    /// Get current tabstop index value.
    pub fn current_index(&self) -> Option<usize> {
        self.current_tabstop().map(|t| t.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_tabstops() {
        let s = SnippetSession::parse("hello $1 world $2 end $0");
        assert_eq!(s.body(), "hello  world  end ");
        assert_eq!(s.tabstops().len(), 3);
        assert_eq!(s.tabstops()[0].index, 1);
        assert_eq!(s.tabstops()[1].index, 2);
        assert_eq!(s.tabstops()[2].index, 0); // $0 last
    }

    #[test]
    fn parse_placeholder() {
        let s = SnippetSession::parse("fn ${1:name}() { $0 }");
        assert_eq!(s.body(), "fn name() {  }");
        assert_eq!(s.tabstops()[0].index, 1);
        assert_eq!(s.tabstops()[0].placeholder_len, 4); // "name"
        assert_eq!(s.tabstops()[1].index, 0);
    }

    #[test]
    fn navigation_forward() {
        let mut s = SnippetSession::parse("$1 $2 $0");
        assert!(s.is_active());
        assert_eq!(s.current_index(), Some(1));
        assert!(s.next());
        assert_eq!(s.current_index(), Some(2));
        assert!(s.next());
        assert_eq!(s.current_index(), Some(0));
        assert!(!s.next()); // at end
        assert!(!s.is_active());
    }

    #[test]
    fn navigation_backward() {
        let mut s = SnippetSession::parse("$1 $2 $0");
        s.next(); // at $2
        assert!(s.prev());
        assert_eq!(s.current_index(), Some(1));
        assert!(!s.prev()); // at start
    }

    #[test]
    fn cancel_deactivates() {
        let mut s = SnippetSession::parse("$1 $0");
        assert!(s.is_active());
        s.cancel();
        assert!(!s.is_active());
    }

    #[test]
    fn no_tabstops_inactive() {
        let s = SnippetSession::parse("plain text");
        assert!(!s.is_active());
        assert_eq!(s.body(), "plain text");
    }

    #[test]
    fn dollar_without_digit() {
        let s = SnippetSession::parse("cost is $, ok");
        assert_eq!(s.body(), "cost is $, ok");
        assert!(!s.is_active());
    }

    #[test]
    fn nested_placeholder() {
        let s = SnippetSession::parse("${1:a{b}c} $0");
        assert_eq!(s.body(), "a{b}c ");
        assert_eq!(s.tabstops()[0].placeholder_len, 5);
    }
}
