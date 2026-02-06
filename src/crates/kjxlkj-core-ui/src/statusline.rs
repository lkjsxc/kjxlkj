//! Status line model — configurable sections and formatting for the status bar.

/// A status line section position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAlign { Left, Center, Right }

/// A single status line section.
#[derive(Debug, Clone)]
pub struct StatusSection {
    pub content: String,
    pub align: SectionAlign,
    pub min_width: u16,
    pub priority: u8,
}

impl StatusSection {
    pub fn left(content: &str) -> Self { Self { content: content.into(), align: SectionAlign::Left, min_width: 0, priority: 50 } }
    pub fn right(content: &str) -> Self { Self { content: content.into(), align: SectionAlign::Right, min_width: 0, priority: 50 } }
    pub fn center(content: &str) -> Self { Self { content: content.into(), align: SectionAlign::Center, min_width: 0, priority: 50 } }
    pub fn with_priority(mut self, p: u8) -> Self { self.priority = p; self }
}

/// Status line layout computer.
#[derive(Debug, Clone)]
pub struct StatusLineLayout { pub sections: Vec<StatusSection> }

impl StatusLineLayout {
    pub fn new() -> Self { Self { sections: Vec::new() } }
    pub fn add(&mut self, section: StatusSection) { self.sections.push(section); }

    /// Render the status line into a fixed-width string.
    pub fn render(&self, width: usize) -> String {
        let lefts: Vec<_> = self.sections.iter().filter(|s| s.align == SectionAlign::Left).collect();
        let rights: Vec<_> = self.sections.iter().filter(|s| s.align == SectionAlign::Right).collect();
        let centers: Vec<_> = self.sections.iter().filter(|s| s.align == SectionAlign::Center).collect();
        let left_str: String = lefts.iter().map(|s| s.content.as_str()).collect::<Vec<_>>().join(" ");
        let right_str: String = rights.iter().map(|s| s.content.as_str()).collect::<Vec<_>>().join(" ");
        let center_str: String = centers.iter().map(|s| s.content.as_str()).collect::<Vec<_>>().join(" ");
        if center_str.is_empty() {
            let gap = width.saturating_sub(left_str.len() + right_str.len());
            format!("{}{:gap$}{}", left_str, "", right_str, gap = gap)
        } else {
            let center_start = width.saturating_sub(center_str.len()) / 2;
            let left_end = center_start.min(left_str.len());
            let right_start = (center_start + center_str.len()).max(width.saturating_sub(right_str.len()));
            let mut line = vec![' '; width];
            for (i, c) in left_str.chars().enumerate().take(left_end) { line[i] = c; }
            for (i, c) in center_str.chars().enumerate() {
                if center_start + i < width { line[center_start + i] = c; }
            }
            for (i, c) in right_str.chars().enumerate() {
                if right_start + i < width { line[right_start + i] = c; }
            }
            line.into_iter().collect()
        }
    }

    /// Build a default Vim-like status line.
    pub fn vim_default(filename: &str, mode: &str, line: usize, col: usize, total: usize, modified: bool) -> Self {
        let mut layout = Self::new();
        let mod_flag = if modified { "[+]" } else { "" };
        layout.add(StatusSection::left(&format!(" {} {}{}", mode, filename, mod_flag)));
        let pct = if total == 0 { 0 } else { (line + 1) * 100 / total };
        layout.add(StatusSection::right(&format!("{}:{} {}% ", line + 1, col + 1, pct)));
        layout
    }
}

impl Default for StatusLineLayout { fn default() -> Self { Self::new() } }

/// Truncate a section to fit within available width.
pub fn truncate_section(content: &str, max: usize) -> String {
    if content.len() <= max { return content.into(); }
    if max <= 3 { return content.chars().take(max).collect(); }
    format!("{}...", &content[..max - 3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_left_right() {
        let mut layout = StatusLineLayout::new();
        layout.add(StatusSection::left("LEFT"));
        layout.add(StatusSection::right("RIGHT"));
        let line = layout.render(40);
        assert!(line.starts_with("LEFT"));
        assert!(line.ends_with("RIGHT"));
        assert_eq!(line.len(), 40);
    }

    #[test]
    fn vim_default_format() {
        let layout = StatusLineLayout::vim_default("main.rs", "NORMAL", 9, 4, 100, true);
        let line = layout.render(60);
        assert!(line.contains("NORMAL"));
        assert!(line.contains("main.rs"));
        assert!(line.contains("[+]"));
        assert!(line.contains("10:5")); // 1-indexed
    }

    #[test]
    fn render_center() {
        let mut layout = StatusLineLayout::new();
        layout.add(StatusSection::center("CENTERED"));
        let line = layout.render(40);
        let pos = line.find("CENTERED").unwrap();
        assert!((pos as i32 - 16).abs() <= 1); // roughly centered
    }

    #[test]
    fn truncate() {
        assert_eq!(truncate_section("hello", 10), "hello");
        assert_eq!(truncate_section("hello world long", 10), "hello w...");
    }

    #[test]
    fn empty_layout() {
        let layout = StatusLineLayout::new();
        let line = layout.render(40);
        assert_eq!(line.len(), 40);
        assert!(line.trim().is_empty());
    }

    #[test]
    fn section_priority() {
        let s = StatusSection::left("test").with_priority(90);
        assert_eq!(s.priority, 90);
    }

    #[test]
    fn vim_default_no_modification() {
        let layout = StatusLineLayout::vim_default("lib.rs", "INSERT", 0, 0, 50, false);
        let line = layout.render(50);
        assert!(!line.contains("[+]"));
        assert!(line.contains("INSERT"));
    }
}
