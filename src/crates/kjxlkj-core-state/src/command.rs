//! Ex command parsing.

use kjxlkj_core_types::EditorAction;

/// Parses Ex commands into editor actions.
pub struct CommandParser;

impl CommandParser {
    /// Parses a command string into an action.
    pub fn parse(input: &str) -> EditorAction {
        let input = input.trim();
        if input.is_empty() {
            return EditorAction::Nop;
        }

        // Handle substitute command: s/pattern/replacement/flags
        if input.starts_with("s/") || input.starts_with("s#") || input.starts_with("s|") {
            if let Some(action) = Self::parse_substitute(input) {
                return action;
            }
        }

        // Handle commands with arguments
        if let Some(rest) = input.strip_prefix("w ") {
            return EditorAction::Write {
                path: Some(rest.trim().to_string()),
            };
        }
        if let Some(rest) = input.strip_prefix("e! ") {
            return EditorAction::EditFile {
                path: rest.trim().to_string(),
                force: true,
            };
        }
        if let Some(rest) = input.strip_prefix("e ") {
            return EditorAction::EditFile {
                path: rest.trim().to_string(),
                force: false,
            };
        }
        if let Some(rest) = input.strip_prefix("wq ") {
            return EditorAction::Write {
                path: Some(rest.trim().to_string()),
            };
        }
        if let Some(rest) = input.strip_prefix("! ") {
            return EditorAction::RunExternal(rest.to_string());
        }

        // Simple commands
        match input {
            "q" => EditorAction::Quit { force: false },
            "q!" => EditorAction::Quit { force: true },
            "qa" => EditorAction::Quit { force: false },
            "qa!" => EditorAction::Quit { force: true },
            "w" => EditorAction::Write { path: None },
            "wa" => EditorAction::Write { path: None },
            "wq" | "x" => EditorAction::Write { path: None },
            _ => EditorAction::Nop,
        }
    }

    /// Parses a substitute command: s/pattern/replacement/flags
    fn parse_substitute(input: &str) -> Option<EditorAction> {
        // Get the delimiter (first char after 's')
        let chars: Vec<char> = input.chars().collect();
        if chars.len() < 2 {
            return None;
        }
        let delimiter = chars[1];
        
        // Split by delimiter, being careful about escaped delimiters
        let rest = &input[2..];
        let parts: Vec<&str> = rest.splitn(3, delimiter).collect();
        
        if parts.is_empty() {
            return None;
        }
        
        let pattern = parts.get(0).unwrap_or(&"").to_string();
        let replacement = parts.get(1).unwrap_or(&"").to_string();
        let flags = parts.get(2).unwrap_or(&"").to_string();
        
        Some(EditorAction::Substitute {
            pattern,
            replacement,
            flags,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quit() {
        assert!(matches!(
            CommandParser::parse("q"),
            EditorAction::Quit { force: false }
        ));
        assert!(matches!(
            CommandParser::parse("q!"),
            EditorAction::Quit { force: true }
        ));
    }

    #[test]
    fn parse_write() {
        assert!(matches!(
            CommandParser::parse("w"),
            EditorAction::Write { path: None }
        ));
    }

    #[test]
    fn parse_edit() {
        if let EditorAction::EditFile { path, force } = CommandParser::parse("e test.txt") {
            assert_eq!(path, "test.txt");
            assert!(!force);
        } else {
            panic!("Expected EditFile");
        }
    }

    #[test]
    fn parse_external() {
        if let EditorAction::RunExternal(cmd) = CommandParser::parse("! ls -la") {
            assert_eq!(cmd, "ls -la");
        } else {
            panic!("Expected RunExternal");
        }
    }

    #[test]
    fn parse_substitute_basic() {
        if let EditorAction::Substitute { pattern, replacement, flags } = CommandParser::parse("s/foo/bar/") {
            assert_eq!(pattern, "foo");
            assert_eq!(replacement, "bar");
            assert_eq!(flags, "");
        } else {
            panic!("Expected Substitute");
        }
    }

    #[test]
    fn parse_substitute_with_flags() {
        if let EditorAction::Substitute { pattern, replacement, flags } = CommandParser::parse("s/foo/bar/g") {
            assert_eq!(pattern, "foo");
            assert_eq!(replacement, "bar");
            assert_eq!(flags, "g");
        } else {
            panic!("Expected Substitute");
        }
    }

    #[test]
    fn parse_substitute_alternate_delimiter() {
        if let EditorAction::Substitute { pattern, replacement, flags } = CommandParser::parse("s#foo#bar#gi") {
            assert_eq!(pattern, "foo");
            assert_eq!(replacement, "bar");
            assert_eq!(flags, "gi");
        } else {
            panic!("Expected Substitute");
        }
    }
}
