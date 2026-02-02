//! Auto-indentation support.

use serde::{Deserialize, Serialize};

/// Indentation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndentStyle {
    /// Use tabs for indentation.
    Tabs,
    /// Use spaces for indentation.
    Spaces(u8),
}

impl Default for IndentStyle {
    fn default() -> Self {
        Self::Spaces(4)
    }
}

impl IndentStyle {
    /// Returns the string for one level of indentation.
    pub fn indent_str(&self) -> String {
        match self {
            Self::Tabs => "\t".to_string(),
            Self::Spaces(n) => " ".repeat(*n as usize),
        }
    }

    /// Returns the display width of one indent level.
    pub fn width(&self) -> usize {
        match self {
            Self::Tabs => 8, // Default tab width
            Self::Spaces(n) => *n as usize,
        }
    }
}

/// Auto-indent configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndentConfig {
    /// Indent style.
    pub style: IndentStyle,
    /// Enable smart indent.
    pub smart_indent: bool,
    /// Increase indent chars (e.g., '{').
    pub increase_indent_on: Vec<char>,
    /// Decrease indent chars (e.g., '}').
    pub decrease_indent_on: Vec<char>,
}

impl Default for IndentConfig {
    fn default() -> Self {
        Self {
            style: IndentStyle::default(),
            smart_indent: true,
            increase_indent_on: vec!['{', '(', '['],
            decrease_indent_on: vec!['}', ')', ']'],
        }
    }
}

impl IndentConfig {
    /// Creates a new default indent config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a config with tabs.
    pub fn with_tabs() -> Self {
        Self {
            style: IndentStyle::Tabs,
            ..Self::default()
        }
    }

    /// Creates a config with spaces.
    pub fn with_spaces(n: u8) -> Self {
        Self {
            style: IndentStyle::Spaces(n),
            ..Self::default()
        }
    }
}

/// Detects the indentation level of a line.
pub fn detect_indent(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

/// Detects the indent style from text.
pub fn detect_indent_style(text: &str) -> IndentStyle {
    let mut tabs = 0;
    let mut spaces = 0;
    let mut space_widths: Vec<usize> = Vec::new();
    
    for line in text.lines() {
        let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
        if indent.is_empty() {
            continue;
        }
        if indent.starts_with('\t') {
            tabs += 1;
        } else if indent.starts_with(' ') {
            spaces += 1;
            let width = indent.len();
            if width > 0 {
                space_widths.push(width);
            }
        }
    }
    
    if tabs > spaces {
        IndentStyle::Tabs
    } else if !space_widths.is_empty() {
        // Find most common space width
        let gcd = find_gcd(&space_widths);
        IndentStyle::Spaces(gcd.clamp(1, 8) as u8)
    } else {
        IndentStyle::default()
    }
}

fn find_gcd(nums: &[usize]) -> usize {
    if nums.is_empty() {
        return 4;
    }
    let mut result = nums[0];
    for &n in nums.iter().skip(1) {
        result = gcd(result, n);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Computes the indent for a new line based on the previous line.
pub fn compute_indent(
    prev_line: &str,
    config: &IndentConfig,
) -> String {
    let base_indent = detect_indent(prev_line);
    let trimmed = prev_line.trim_end();
    
    if config.smart_indent {
        // Check if should increase indent
        if let Some(last) = trimmed.chars().last() {
            if config.increase_indent_on.contains(&last) {
                let current_indent: String = prev_line.chars()
                    .take_while(|c| c.is_whitespace())
                    .collect();
                return format!("{}{}", current_indent, config.style.indent_str());
            }
        }
    }
    
    // Return same indentation as previous line
    prev_line.chars().take(base_indent).collect()
}

/// Adjusts indent for closing characters.
pub fn adjust_indent_for_closing(
    line: &str,
    config: &IndentConfig,
) -> Option<String> {
    let trimmed = line.trim();
    if let Some(first) = trimmed.chars().next() {
        if config.decrease_indent_on.contains(&first) {
            let current = detect_indent(line);
            let indent_width = config.style.width();
            if current >= indent_width {
                let new_indent = current - indent_width;
                return Some(" ".repeat(new_indent) + trimmed);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_style_default() {
        assert_eq!(IndentStyle::default(), IndentStyle::Spaces(4));
    }

    #[test]
    fn test_indent_str() {
        assert_eq!(IndentStyle::Tabs.indent_str(), "\t");
        assert_eq!(IndentStyle::Spaces(2).indent_str(), "  ");
    }

    #[test]
    fn test_detect_indent() {
        assert_eq!(detect_indent("    foo"), 4);
        assert_eq!(detect_indent("\tfoo"), 1);
        assert_eq!(detect_indent("foo"), 0);
    }

    #[test]
    fn test_detect_indent_style_tabs() {
        let text = "\tfoo\n\tbar\n\tbaz";
        assert_eq!(detect_indent_style(text), IndentStyle::Tabs);
    }

    #[test]
    fn test_detect_indent_style_spaces() {
        let text = "  foo\n  bar\n    baz";
        assert_eq!(detect_indent_style(text), IndentStyle::Spaces(2));
    }

    #[test]
    fn test_compute_indent_basic() {
        let config = IndentConfig::with_spaces(4);
        let indent = compute_indent("    foo", &config);
        assert_eq!(indent, "    ");
    }

    #[test]
    fn test_compute_indent_increase() {
        let config = IndentConfig::with_spaces(4);
        let indent = compute_indent("    if (true) {", &config);
        assert_eq!(indent, "        ");
    }

    #[test]
    fn test_adjust_closing_brace() {
        let config = IndentConfig::with_spaces(4);
        let result = adjust_indent_for_closing("        }", &config);
        assert_eq!(result, Some("    }".to_string()));
    }
}
