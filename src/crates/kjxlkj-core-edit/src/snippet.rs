//! Snippet system for code templates.
//!
//! Provides snippet parsing, placeholder navigation, and expansion.

use std::collections::HashMap;

/// Placeholder in a snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placeholder {
    /// Placeholder index (1, 2, 3... or 0 for final).
    pub index: usize,
    /// Default text.
    pub default: String,
    /// Linked placeholder indices (same value).
    pub linked: Vec<usize>,
    /// Byte range in expanded text.
    pub range: std::ops::Range<usize>,
}

impl Placeholder {
    /// Create a simple placeholder.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            default: String::new(),
            linked: Vec::new(),
            range: 0..0,
        }
    }

    /// Create with default value.
    pub fn with_default(index: usize, default: impl Into<String>) -> Self {
        Self {
            index,
            default: default.into(),
            linked: Vec::new(),
            range: 0..0,
        }
    }

    /// Whether this is the final placeholder ($0).
    pub fn is_final(&self) -> bool {
        self.index == 0
    }
}

/// Snippet transformation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transform {
    /// Uppercase: ${1:/upcase}
    Upcase,
    /// Lowercase: ${1:/downcase}
    Downcase,
    /// Capitalize: ${1:/capitalize}
    Capitalize,
    /// Camelcase: ${1:/camelcase}
    Camelcase,
    /// Pascalcase: ${1:/pascalcase}
    Pascalcase,
    /// Snakecase: ${1:/snakecase}
    Snakecase,
    /// Regex replace: ${1/pattern/replacement/flags}
    Regex {
        pattern: String,
        replacement: String,
        flags: String,
    },
}

impl Transform {
    /// Apply transformation to text.
    pub fn apply(&self, text: &str) -> String {
        match self {
            Transform::Upcase => text.to_uppercase(),
            Transform::Downcase => text.to_lowercase(),
            Transform::Capitalize => {
                let mut chars = text.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            }
            Transform::Camelcase => {
                let parts: Vec<&str> = text.split(['_', '-', ' ']).collect();
                let mut result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    if i == 0 {
                        result.push_str(&part.to_lowercase());
                    } else {
                        let mut chars = part.chars();
                        if let Some(c) = chars.next() {
                            result.push_str(&c.to_uppercase().to_string());
                            result.push_str(&chars.collect::<String>().to_lowercase());
                        }
                    }
                }
                result
            }
            Transform::Pascalcase => {
                let parts: Vec<&str> = text.split(['_', '-', ' ']).collect();
                let mut result = String::new();
                for part in parts {
                    let mut chars = part.chars();
                    if let Some(c) = chars.next() {
                        result.push_str(&c.to_uppercase().to_string());
                        result.push_str(&chars.collect::<String>().to_lowercase());
                    }
                }
                result
            }
            Transform::Snakecase => {
                let mut result = String::new();
                for (i, c) in text.chars().enumerate() {
                    if c.is_uppercase() && i > 0 {
                        result.push('_');
                    }
                    result.push(c.to_lowercase().next().unwrap_or(c));
                }
                result
            }
            Transform::Regex { pattern, replacement, .. } => {
                // Simple replacement, no actual regex
                text.replace(pattern, replacement)
            }
        }
    }
}

/// Built-in snippet variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SnippetVariable {
    /// Current filename without path.
    TmFilename,
    /// Current filename without extension.
    TmFilenameBase,
    /// Full file path.
    TmFilepath,
    /// Directory of current file.
    TmDirectory,
    /// Current line number.
    TmLineNumber,
    /// Current column number.
    TmLineIndex,
    /// Currently selected text.
    TmSelectedText,
    /// Contents of current line.
    TmCurrentLine,
    /// Current word under cursor.
    TmCurrentWord,
    /// Current year (4 digits).
    CurrentYear,
    /// Current year (2 digits).
    CurrentYearShort,
    /// Current month (2 digits).
    CurrentMonth,
    /// Current month name.
    CurrentMonthName,
    /// Current month short name.
    CurrentMonthNameShort,
    /// Current date (2 digits).
    CurrentDate,
    /// Current day name.
    CurrentDayName,
    /// Current day short name.
    CurrentDayNameShort,
    /// Current hour (24h).
    CurrentHour,
    /// Current minute.
    CurrentMinute,
    /// Current second.
    CurrentSecond,
    /// Clipboard contents.
    Clipboard,
    /// UUID.
    Uuid,
    /// Random hex (6 chars).
    RandomHex,
    /// Workspace name.
    WorkspaceName,
    /// Workspace folder.
    WorkspaceFolder,
}

impl SnippetVariable {
    /// Parse variable name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "TM_FILENAME" => Some(Self::TmFilename),
            "TM_FILENAME_BASE" => Some(Self::TmFilenameBase),
            "TM_FILEPATH" => Some(Self::TmFilepath),
            "TM_DIRECTORY" => Some(Self::TmDirectory),
            "TM_LINE_NUMBER" => Some(Self::TmLineNumber),
            "TM_LINE_INDEX" => Some(Self::TmLineIndex),
            "TM_SELECTED_TEXT" => Some(Self::TmSelectedText),
            "TM_CURRENT_LINE" => Some(Self::TmCurrentLine),
            "TM_CURRENT_WORD" => Some(Self::TmCurrentWord),
            "CURRENT_YEAR" => Some(Self::CurrentYear),
            "CURRENT_YEAR_SHORT" => Some(Self::CurrentYearShort),
            "CURRENT_MONTH" => Some(Self::CurrentMonth),
            "CURRENT_MONTH_NAME" => Some(Self::CurrentMonthName),
            "CURRENT_MONTH_NAME_SHORT" => Some(Self::CurrentMonthNameShort),
            "CURRENT_DATE" => Some(Self::CurrentDate),
            "CURRENT_DAY_NAME" => Some(Self::CurrentDayName),
            "CURRENT_DAY_NAME_SHORT" => Some(Self::CurrentDayNameShort),
            "CURRENT_HOUR" => Some(Self::CurrentHour),
            "CURRENT_MINUTE" => Some(Self::CurrentMinute),
            "CURRENT_SECOND" => Some(Self::CurrentSecond),
            "CLIPBOARD" => Some(Self::Clipboard),
            "UUID" => Some(Self::Uuid),
            "RANDOM_HEX" => Some(Self::RandomHex),
            "WORKSPACE_NAME" => Some(Self::WorkspaceName),
            "WORKSPACE_FOLDER" => Some(Self::WorkspaceFolder),
            _ => None,
        }
    }
}

/// Variable context for snippet expansion.
#[derive(Debug, Clone, Default)]
pub struct SnippetContext {
    /// Variable values.
    pub variables: HashMap<SnippetVariable, String>,
    /// Custom variables.
    pub custom: HashMap<String, String>,
}

impl SnippetContext {
    /// Create new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a variable.
    pub fn set(&mut self, var: SnippetVariable, value: impl Into<String>) {
        self.variables.insert(var, value.into());
    }

    /// Get a variable.
    pub fn get(&self, var: SnippetVariable) -> Option<&str> {
        self.variables.get(&var).map(|s| s.as_str())
    }

    /// Set custom variable.
    pub fn set_custom(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.custom.insert(name.into(), value.into());
    }
}

/// Snippet definition.
#[derive(Debug, Clone)]
pub struct Snippet {
    /// Snippet prefix (trigger).
    pub prefix: String,
    /// Snippet body (template).
    pub body: String,
    /// Description.
    pub description: String,
    /// Filetypes (empty = all).
    pub filetypes: Vec<String>,
}

impl Snippet {
    /// Create a new snippet.
    pub fn new(prefix: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            body: body.into(),
            description: String::new(),
            filetypes: Vec::new(),
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set filetypes.
    pub fn for_filetypes(mut self, filetypes: Vec<String>) -> Self {
        self.filetypes = filetypes;
        self
    }

    /// Check if snippet applies to filetype.
    pub fn applies_to(&self, filetype: &str) -> bool {
        self.filetypes.is_empty() || self.filetypes.iter().any(|ft| ft == filetype)
    }
}

/// Expanded snippet state (active editing session).
#[derive(Debug, Clone)]
pub struct SnippetSession {
    /// Expanded text.
    pub text: String,
    /// All placeholders.
    pub placeholders: Vec<Placeholder>,
    /// Current placeholder index (in placeholders vec).
    pub current: usize,
    /// Whether session is active.
    pub active: bool,
    /// Start offset in buffer.
    pub start_offset: usize,
}

impl SnippetSession {
    /// Create new session.
    pub fn new(text: String, placeholders: Vec<Placeholder>, start_offset: usize) -> Self {
        Self {
            text,
            placeholders,
            current: 0,
            active: true,
            start_offset,
        }
    }

    /// Get current placeholder.
    pub fn current_placeholder(&self) -> Option<&Placeholder> {
        self.placeholders.get(self.current)
    }

    /// Move to next placeholder.
    pub fn next_placeholder(&mut self) -> Option<&Placeholder> {
        if self.current + 1 < self.placeholders.len() {
            self.current += 1;
            self.placeholders.get(self.current)
        } else {
            // Move to final placeholder if exists
            let final_idx = self.placeholders.iter().position(|p| p.is_final());
            if let Some(idx) = final_idx {
                self.current = idx;
                self.active = false;
                self.placeholders.get(self.current)
            } else {
                self.active = false;
                None
            }
        }
    }

    /// Move to previous placeholder.
    pub fn prev_placeholder(&mut self) -> Option<&Placeholder> {
        if self.current > 0 {
            self.current -= 1;
            self.placeholders.get(self.current)
        } else {
            None
        }
    }

    /// Update placeholder text.
    pub fn update_current(&mut self, new_text: &str) {
        if let Some(ph) = self.placeholders.get_mut(self.current) {
            let old_len = ph.range.len();
            let new_len = new_text.len();
            let diff = new_len as isize - old_len as isize;

            // Update text
            let range = ph.range.clone();
            self.text.replace_range(range.clone(), new_text);

            // Update this placeholder's range
            ph.range.end = ph.range.start + new_len;

            // Update linked placeholders
            let linked = ph.linked.clone();
            let idx = ph.index;
            for linked_idx in linked {
                if let Some(linked_ph) = self.placeholders.iter_mut().find(|p| p.index == linked_idx && p.index != idx) {
                    linked_ph.range.start = (linked_ph.range.start as isize + diff) as usize;
                    linked_ph.range.end = (linked_ph.range.end as isize + diff) as usize;
                    let lr = linked_ph.range.clone();
                    self.text.replace_range(lr, new_text);
                }
            }

            // Adjust subsequent placeholder ranges
            for p in &mut self.placeholders {
                if p.range.start > range.start {
                    p.range.start = (p.range.start as isize + diff) as usize;
                    p.range.end = (p.range.end as isize + diff) as usize;
                }
            }
        }
    }

    /// Exit snippet session.
    pub fn exit(&mut self) {
        self.active = false;
    }

    /// Check if on final placeholder.
    pub fn is_at_final(&self) -> bool {
        self.current_placeholder().map(|p| p.is_final()).unwrap_or(false)
    }
}

/// Snippet engine.
#[derive(Debug, Default)]
pub struct SnippetEngine {
    /// Registered snippets by prefix.
    snippets: HashMap<String, Vec<Snippet>>,
    /// Current active session.
    session: Option<SnippetSession>,
}

impl SnippetEngine {
    /// Create new engine.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a snippet.
    pub fn register(&mut self, snippet: Snippet) {
        self.snippets
            .entry(snippet.prefix.clone())
            .or_default()
            .push(snippet);
    }

    /// Get snippets for a prefix.
    pub fn get(&self, prefix: &str, filetype: &str) -> Vec<&Snippet> {
        self.snippets
            .get(prefix)
            .map(|snips| snips.iter().filter(|s| s.applies_to(filetype)).collect())
            .unwrap_or_default()
    }

    /// Get all snippets for a filetype.
    pub fn all_for_filetype(&self, filetype: &str) -> Vec<&Snippet> {
        self.snippets
            .values()
            .flat_map(|snips| snips.iter().filter(|s| s.applies_to(filetype)))
            .collect()
    }

    /// Expand a snippet.
    pub fn expand(
        &mut self,
        snippet: &Snippet,
        context: &SnippetContext,
        start_offset: usize,
    ) -> Option<&SnippetSession> {
        let (text, placeholders) = Self::parse_and_expand(&snippet.body, context);
        let session = SnippetSession::new(text, placeholders, start_offset);
        self.session = Some(session);
        self.session.as_ref()
    }

    /// Get current session.
    pub fn session(&self) -> Option<&SnippetSession> {
        self.session.as_ref()
    }

    /// Get mutable session.
    pub fn session_mut(&mut self) -> Option<&mut SnippetSession> {
        self.session.as_mut()
    }

    /// End current session.
    pub fn end_session(&mut self) {
        self.session = None;
    }

    /// Parse and expand snippet body.
    #[allow(clippy::while_let_on_iterator)]
    fn parse_and_expand(body: &str, context: &SnippetContext) -> (String, Vec<Placeholder>) {
        let mut result = String::new();
        let mut placeholders = Vec::new();
        let mut chars = body.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' {
                if let Some(&next) = chars.peek() {
                    if next == '{' {
                        // ${n:default} or ${VAR}
                        chars.next();
                        let mut content = String::new();
                        let mut depth = 1;
                        while let Some(c) = chars.next() {
                            if c == '{' {
                                depth += 1;
                                content.push(c);
                            } else if c == '}' {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                                content.push(c);
                            } else {
                                content.push(c);
                            }
                        }
                        Self::process_braced(&content, context, &mut result, &mut placeholders);
                    } else if next.is_ascii_digit() {
                        // $n
                        chars.next();
                        let index = next.to_digit(10).unwrap() as usize;
                        let start = result.len();
                        let ph = Placeholder {
                            index,
                            default: String::new(),
                            linked: Vec::new(),
                            range: start..start,
                        };
                        placeholders.push(ph);
                    } else if next.is_alphabetic() || next == '_' {
                        // $VAR
                        let mut name = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_alphanumeric() || c == '_' {
                                name.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if let Some(var) = SnippetVariable::from_name(&name) {
                            if let Some(val) = context.get(var) {
                                result.push_str(val);
                            }
                        } else if let Some(val) = context.custom.get(&name) {
                            result.push_str(val);
                        }
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            } else if c == '\\' {
                // Escape
                if let Some(&next) = chars.peek() {
                    if matches!(next, '$' | '{' | '}' | '\\') {
                        result.push(next);
                        chars.next();
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }

        // Sort placeholders by index, with 0 (final) last
        placeholders.sort_by_key(|p| if p.index == 0 { usize::MAX } else { p.index });

        (result, placeholders)
    }

    fn process_braced(
        content: &str,
        context: &SnippetContext,
        result: &mut String,
        placeholders: &mut Vec<Placeholder>,
    ) {
        if let Some((idx_str, rest)) = content.split_once(':') {
            // ${n:default}
            if let Ok(index) = idx_str.parse::<usize>() {
                let default = rest.to_string();
                let start = result.len();
                result.push_str(&default);
                let end = result.len();
                let ph = Placeholder {
                    index,
                    default,
                    linked: Vec::new(),
                    range: start..end,
                };
                placeholders.push(ph);
            }
        } else if let Ok(index) = content.parse::<usize>() {
            // ${n}
            let start = result.len();
            let ph = Placeholder {
                index,
                default: String::new(),
                linked: Vec::new(),
                range: start..start,
            };
            placeholders.push(ph);
        } else if let Some(var) = SnippetVariable::from_name(content) {
            // ${VAR}
            if let Some(val) = context.get(var) {
                result.push_str(val);
            }
        } else if let Some(val) = context.custom.get(content) {
            result.push_str(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_new() {
        let ph = Placeholder::new(1);
        assert_eq!(ph.index, 1);
        assert!(!ph.is_final());
    }

    #[test]
    fn test_placeholder_final() {
        let ph = Placeholder::new(0);
        assert!(ph.is_final());
    }

    #[test]
    fn test_transform_upcase() {
        let t = Transform::Upcase;
        assert_eq!(t.apply("hello"), "HELLO");
    }

    #[test]
    fn test_transform_downcase() {
        let t = Transform::Downcase;
        assert_eq!(t.apply("HELLO"), "hello");
    }

    #[test]
    fn test_transform_capitalize() {
        let t = Transform::Capitalize;
        assert_eq!(t.apply("hello"), "Hello");
    }

    #[test]
    fn test_transform_camelcase() {
        let t = Transform::Camelcase;
        assert_eq!(t.apply("hello_world"), "helloWorld");
    }

    #[test]
    fn test_transform_pascalcase() {
        let t = Transform::Pascalcase;
        assert_eq!(t.apply("hello_world"), "HelloWorld");
    }

    #[test]
    fn test_transform_snakecase() {
        let t = Transform::Snakecase;
        assert_eq!(t.apply("HelloWorld"), "hello_world");
    }

    #[test]
    fn test_snippet_variable_parse() {
        assert_eq!(
            SnippetVariable::from_name("TM_FILENAME"),
            Some(SnippetVariable::TmFilename)
        );
        assert_eq!(SnippetVariable::from_name("UNKNOWN"), None);
    }

    #[test]
    fn test_snippet_new() {
        let s = Snippet::new("fn", "fn ${1:name}() {\n    $0\n}");
        assert_eq!(s.prefix, "fn");
        assert!(s.applies_to("rust"));
    }

    #[test]
    fn test_snippet_filetypes() {
        let s = Snippet::new("fn", "fn").for_filetypes(vec!["rust".to_string()]);
        assert!(s.applies_to("rust"));
        assert!(!s.applies_to("python"));
    }

    #[test]
    fn test_snippet_engine_register() {
        let mut engine = SnippetEngine::new();
        engine.register(Snippet::new("fn", "fn() {}"));
        let snips = engine.get("fn", "rust");
        assert_eq!(snips.len(), 1);
    }

    #[test]
    fn test_snippet_expand_simple() {
        let mut engine = SnippetEngine::new();
        let snippet = Snippet::new("test", "hello ${1:world}!");
        let ctx = SnippetContext::new();
        engine.expand(&snippet, &ctx, 0);

        let session = engine.session().unwrap();
        assert_eq!(session.text, "hello world!");
        assert_eq!(session.placeholders.len(), 1);
        assert_eq!(session.placeholders[0].default, "world");
    }

    #[test]
    fn test_snippet_expand_with_variable() {
        let mut engine = SnippetEngine::new();
        let snippet = Snippet::new("file", "File: $TM_FILENAME");
        let mut ctx = SnippetContext::new();
        ctx.set(SnippetVariable::TmFilename, "test.rs");
        engine.expand(&snippet, &ctx, 0);

        let session = engine.session().unwrap();
        assert_eq!(session.text, "File: test.rs");
    }

    #[test]
    fn test_session_next_prev() {
        let placeholders = vec![
            Placeholder::with_default(1, "first"),
            Placeholder::with_default(2, "second"),
            Placeholder::new(0),
        ];
        let mut session = SnippetSession::new("first second ".to_string(), placeholders, 0);

        assert_eq!(session.current, 0);
        session.next_placeholder();
        assert_eq!(session.current, 1);
        session.prev_placeholder();
        assert_eq!(session.current, 0);
    }

    #[test]
    fn test_session_update_current() {
        let mut session = SnippetSession::new(
            "hello world!".to_string(),
            vec![Placeholder {
                index: 1,
                default: "world".to_string(),
                linked: Vec::new(),
                range: 6..11,
            }],
            0,
        );

        session.update_current("universe");
        assert_eq!(session.text, "hello universe!");
    }
}
