//! Search engine: find_next, count_matches, build_all_matches.

use crate::regex_translate::compile_vim_pattern;
use crate::search_types::{SearchDirection, SearchMatch, SearchState};

/// Match pattern against a line using regex, falling back to plain text.
fn find_in_line(line: &str, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
    if let Some(re) = compile_vim_pattern(pattern, case_sensitive) {
        re.find_iter(line).map(|m| (m.start(), m.end())).collect()
    } else if case_sensitive {
        line.match_indices(pattern)
            .map(|(i, m)| (i, i + m.len()))
            .collect()
    } else {
        let line_lower = line.to_lowercase();
        let pat_lower = pattern.to_lowercase();
        line_lower
            .match_indices(&pat_lower)
            .map(|(i, m)| (i, i + m.len()))
            .collect()
    }
}

impl SearchState {
    /// Search for pattern in lines, starting from a position.
    pub fn find_next(
        &self,
        lines: &[&str],
        start_line: usize,
        start_col: usize,
    ) -> Option<SearchMatch> {
        let pattern = self.pattern.as_ref()?;
        if pattern.is_empty() || lines.is_empty() {
            return None;
        }

        let case_sensitive = self.is_case_sensitive();
        let total = lines.len();

        match self.direction {
            SearchDirection::Forward => {
                let hits = find_in_line(lines[start_line], pattern, case_sensitive);
                for (s, e) in &hits {
                    if *s > start_col {
                        return Some(SearchMatch {
                            line: start_line,
                            col_start: *s,
                            col_end: *e,
                        });
                    }
                }
                for offset in 1..total {
                    let idx = (start_line + offset) % total;
                    if !self.wrap_scan && idx < start_line {
                        break;
                    }
                    let hits = find_in_line(lines[idx], pattern, case_sensitive);
                    if let Some((s, e)) = hits.first() {
                        return Some(SearchMatch {
                            line: idx,
                            col_start: *s,
                            col_end: *e,
                        });
                    }
                }
            }
            SearchDirection::Backward => {
                let hits = find_in_line(lines[start_line], pattern, case_sensitive);
                for (s, e) in hits.iter().rev() {
                    if *s < start_col {
                        return Some(SearchMatch {
                            line: start_line,
                            col_start: *s,
                            col_end: *e,
                        });
                    }
                }
                for offset in 1..total {
                    let idx = (start_line + total - offset) % total;
                    if !self.wrap_scan && idx > start_line {
                        break;
                    }
                    let hits = find_in_line(lines[idx], pattern, case_sensitive);
                    if let Some((s, e)) = hits.last() {
                        return Some(SearchMatch {
                            line: idx,
                            col_start: *s,
                            col_end: *e,
                        });
                    }
                }
            }
        }

        None
    }

    /// Count all matches of the current pattern.
    pub fn count_matches(&self, lines: &[&str]) -> usize {
        let Some(pattern) = self.pattern.as_ref() else {
            return 0;
        };
        if pattern.is_empty() {
            return 0;
        }
        let cs = self.is_case_sensitive();
        lines
            .iter()
            .map(|line| find_in_line(line, pattern, cs).len())
            .sum()
    }

    /// Build all matches for highlighting.
    pub fn build_all_matches(&mut self, lines: &[&str]) {
        self.matches.clear();
        let Some(pattern) = self.pattern.as_ref() else {
            return;
        };
        if pattern.is_empty() {
            return;
        }
        let cs = self.is_case_sensitive();

        // Multi-line pattern: search joined text.
        if pattern.contains("\\n") || pattern.contains('\n') {
            let text = lines.join("\n");
            let re = compile_vim_pattern(pattern, cs);
            if let Some(re) = re {
                for m in re.find_iter(&text) {
                    let (line, col) = byte_to_line_col_in(&text, m.start());
                    self.matches.push(SearchMatch {
                        line,
                        col_start: col,
                        col_end: col + m.len(),
                    });
                }
            }
            return;
        }

        for (line_idx, line) in lines.iter().enumerate() {
            for (s, e) in find_in_line(line, pattern, cs) {
                self.matches.push(SearchMatch {
                    line: line_idx,
                    col_start: s,
                    col_end: e,
                });
            }
        }
    }
}

/// Convert byte offset in full text to (line, col).
fn byte_to_line_col_in(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut line_start = 0;
    for (i, ch) in text.char_indices() {
        if i == offset {
            return (line, i - line_start);
        }
        if ch == '\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    (line, offset.saturating_sub(line_start))
}
