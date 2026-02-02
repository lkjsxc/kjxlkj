//! Global command implementation.
//!
//! Handles :g/pattern/cmd and :v/pattern/cmd commands.

/// Global command mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlobalMode {
    /// :g - execute on matching lines.
    Matching,
    /// :v - execute on non-matching lines.
    NonMatching,
}

/// A parsed global command.
#[derive(Debug, Clone)]
pub struct GlobalCommand {
    /// The search pattern.
    pub pattern: String,
    /// The command to execute.
    pub command: String,
    /// The mode (matching or non-matching).
    pub mode: GlobalMode,
}

impl GlobalCommand {
    /// Creates a new global command.
    pub fn new(pattern: &str, command: &str, mode: GlobalMode) -> Self {
        Self {
            pattern: pattern.to_string(),
            command: command.to_string(),
            mode,
        }
    }

    /// Parses a global command from a string.
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();

        let (mode, rest) = if input.starts_with("g/") || input.starts_with("global/") {
            (GlobalMode::Matching, input.split_at(input.find('/')?).1)
        } else if input.starts_with("v/") || input.starts_with("vglobal/") {
            (GlobalMode::NonMatching, input.split_at(input.find('/')?).1)
        } else {
            return None;
        };

        // rest starts with '/'
        let rest = &rest[1..];
        
        // Find the closing delimiter
        let delim_pos = rest.find('/')?;
        let pattern = &rest[..delim_pos];
        let command = rest.get(delim_pos + 1..)?.trim();

        Some(Self {
            pattern: pattern.to_string(),
            command: command.to_string(),
            mode,
        })
    }

    /// Returns matching line numbers for the given lines.
    pub fn matching_lines<'a, I>(&self, lines: I) -> Vec<usize>
    where
        I: Iterator<Item = (usize, &'a str)>,
    {
        lines
            .filter(|(_, line)| {
                let matches = line.contains(&self.pattern);
                match self.mode {
                    GlobalMode::Matching => matches,
                    GlobalMode::NonMatching => !matches,
                }
            })
            .map(|(idx, _)| idx)
            .collect()
    }
}

/// Result of a global command execution.
#[derive(Debug, Clone, Default)]
pub struct GlobalResult {
    /// Lines that were processed.
    pub lines_processed: Vec<usize>,
    /// Total count.
    pub count: usize,
}

impl GlobalResult {
    /// Returns whether any lines were processed.
    pub fn has_results(&self) -> bool {
        self.count > 0
    }

    /// Returns a summary message.
    pub fn message(&self) -> String {
        if self.count == 0 {
            "Pattern not found".to_string()
        } else {
            format!("{} lines processed", self.count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_command_parse_matching() {
        let cmd = GlobalCommand::parse("g/pattern/d").unwrap();
        assert_eq!(cmd.pattern, "pattern");
        assert_eq!(cmd.command, "d");
        assert_eq!(cmd.mode, GlobalMode::Matching);
    }

    #[test]
    fn test_global_command_parse_non_matching() {
        let cmd = GlobalCommand::parse("v/pattern/d").unwrap();
        assert_eq!(cmd.pattern, "pattern");
        assert_eq!(cmd.command, "d");
        assert_eq!(cmd.mode, GlobalMode::NonMatching);
    }

    #[test]
    fn test_matching_lines_g() {
        let cmd = GlobalCommand::new("foo", "d", GlobalMode::Matching);
        let lines = vec!["foo bar", "baz", "foo qux", "other"];
        let matches = cmd.matching_lines(lines.iter().enumerate().map(|(i, l)| (i, *l)));
        assert_eq!(matches, vec![0, 2]);
    }

    #[test]
    fn test_matching_lines_v() {
        let cmd = GlobalCommand::new("foo", "d", GlobalMode::NonMatching);
        let lines = vec!["foo bar", "baz", "foo qux", "other"];
        let matches = cmd.matching_lines(lines.iter().enumerate().map(|(i, l)| (i, *l)));
        assert_eq!(matches, vec![1, 3]);
    }

    #[test]
    fn test_global_result_message() {
        let result = GlobalResult::default();
        assert_eq!(result.message(), "Pattern not found");

        let result = GlobalResult {
            lines_processed: vec![0, 1, 2],
            count: 3,
        };
        assert!(result.message().contains("3 lines"));
    }
}
