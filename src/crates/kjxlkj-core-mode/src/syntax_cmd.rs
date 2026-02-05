//! Syntax-related Ex commands.
//!
//! Provides :syntax, :highlight, and :colorscheme commands.

use std::collections::HashMap;

/// Syntax command type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxCommand {
    /// :syntax on - enable syntax highlighting.
    On,
    /// :syntax off - disable syntax highlighting.
    Off,
    /// :syntax enable - enable with default colors.
    Enable,
    /// :syntax reset - reset to default syntax.
    Reset,
    /// :syntax clear - clear all syntax items.
    Clear,
    /// :syntax list - list syntax items.
    List(Option<String>),
}

/// Highlight attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighlightAttr {
    /// Foreground color.
    Fg(String),
    /// Background color.
    Bg(String),
    /// Special color (underline, etc.).
    Sp(String),
    /// Bold attribute.
    Bold(bool),
    /// Italic attribute.
    Italic(bool),
    /// Underline attribute.
    Underline(bool),
    /// Underline curl.
    Undercurl(bool),
    /// Strikethrough.
    Strikethrough(bool),
    /// Reverse video.
    Reverse(bool),
    /// Standout.
    Standout(bool),
    /// Blend level (0-100).
    Blend(u8),
    /// No combine.
    NoCombine(bool),
    /// Link to another group.
    Link(String),
    /// Clear the group.
    Clear,
}

/// Highlight command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightCommand {
    /// Whether to use ! (force override).
    pub bang: bool,
    /// Group name to define/modify.
    pub group: Option<String>,
    /// Attributes to set.
    pub attrs: Vec<HighlightAttr>,
    /// List mode (no group or attrs).
    pub list: bool,
    /// Default flag (use as default only).
    pub default: bool,
}

impl HighlightCommand {
    /// Create a new highlight command.
    pub fn new() -> Self {
        Self {
            bang: false,
            group: None,
            attrs: Vec::new(),
            list: false,
            default: false,
        }
    }

    /// Parse a highlight command from arguments.
    pub fn parse(args: &str) -> Result<Self, SyntaxError> {
        let mut cmd = Self::new();
        let mut args = args.trim();

        // Check for bang.
        if let Some(rest) = args.strip_prefix('!') {
            cmd.bang = true;
            args = rest.trim_start();
        }

        // Empty args = list mode.
        if args.is_empty() {
            cmd.list = true;
            return Ok(cmd);
        }

        // Check for clear.
        if args.eq_ignore_ascii_case("clear") {
            return Ok(cmd);
        }

        // Check for default keyword.
        if let Some(rest) = args.strip_prefix("default").or_else(|| args.strip_prefix("def")) {
            cmd.default = true;
            args = rest.trim_start();
        }

        // Check for link.
        if let Some(rest) = args.strip_prefix("link") {
            let rest = rest.trim_start();
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                cmd.group = Some(parts[0].to_string());
                cmd.attrs.push(HighlightAttr::Link(parts[1].to_string()));
            } else {
                return Err(SyntaxError::InvalidHighlight("link requires two group names".into()));
            }
            return Ok(cmd);
        }

        // Parse group name.
        let mut parts = args.split_whitespace();
        if let Some(group) = parts.next() {
            cmd.group = Some(group.to_string());
        }

        // Parse attributes.
        for part in parts {
            if let Some(attr) = Self::parse_attr(part)? {
                cmd.attrs.push(attr);
            }
        }

        if cmd.group.is_some() && cmd.attrs.is_empty() {
            cmd.list = true;
        }

        Ok(cmd)
    }

    fn parse_attr(attr: &str) -> Result<Option<HighlightAttr>, SyntaxError> {
        let attr_lower = attr.to_lowercase();

        // Handle key=value pairs.
        if let Some(eq_pos) = attr.find('=') {
            let key = &attr[..eq_pos];
            let value = &attr[eq_pos + 1..];

            return match key.to_lowercase().as_str() {
                "guifg" | "ctermfg" | "fg" => Ok(Some(HighlightAttr::Fg(value.to_string()))),
                "guibg" | "ctermbg" | "bg" => Ok(Some(HighlightAttr::Bg(value.to_string()))),
                "guisp" | "sp" => Ok(Some(HighlightAttr::Sp(value.to_string()))),
                "gui" | "cterm" => Self::parse_gui_value(value),
                "blend" => {
                    let blend = value.parse::<u8>().map_err(|_| {
                        SyntaxError::InvalidHighlight(format!("invalid blend value: {}", value))
                    })?;
                    Ok(Some(HighlightAttr::Blend(blend.min(100))))
                }
                _ => Ok(None),
            };
        }

        // Handle NONE.
        if attr_lower == "none" {
            return Ok(Some(HighlightAttr::Clear));
        }

        Ok(None)
    }

    fn parse_gui_value(value: &str) -> Result<Option<HighlightAttr>, SyntaxError> {
        let attrs: Vec<&str> = value.split(',').collect();
        for attr in attrs {
            match attr.to_lowercase().as_str() {
                "bold" => return Ok(Some(HighlightAttr::Bold(true))),
                "italic" => return Ok(Some(HighlightAttr::Italic(true))),
                "underline" => return Ok(Some(HighlightAttr::Underline(true))),
                "undercurl" => return Ok(Some(HighlightAttr::Undercurl(true))),
                "strikethrough" => return Ok(Some(HighlightAttr::Strikethrough(true))),
                "reverse" => return Ok(Some(HighlightAttr::Reverse(true))),
                "standout" => return Ok(Some(HighlightAttr::Standout(true))),
                "nocombine" => return Ok(Some(HighlightAttr::NoCombine(true))),
                "none" => return Ok(Some(HighlightAttr::Clear)),
                _ => {}
            }
        }
        Ok(None)
    }
}

impl Default for HighlightCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// Colorscheme command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorschemeCommand {
    /// Colorscheme name to load.
    pub name: Option<String>,
}

impl ColorschemeCommand {
    /// Parse colorscheme command.
    pub fn parse(args: &str) -> Self {
        let name = args.trim();
        Self {
            name: if name.is_empty() { None } else { Some(name.to_string()) },
        }
    }
}

/// Syntax command error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    /// Invalid syntax command.
    InvalidSyntax(String),
    /// Invalid highlight command.
    InvalidHighlight(String),
    /// Unknown colorscheme.
    UnknownColorscheme(String),
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSyntax(msg) => write!(f, "Invalid syntax command: {}", msg),
            Self::InvalidHighlight(msg) => write!(f, "Invalid highlight command: {}", msg),
            Self::UnknownColorscheme(name) => write!(f, "Unknown colorscheme: {}", name),
        }
    }
}

impl std::error::Error for SyntaxError {}

/// Syntax state for the editor.
#[derive(Debug, Clone)]
pub struct SyntaxSettings {
    /// Whether syntax highlighting is enabled.
    pub enabled: bool,
    /// Current colorscheme name.
    pub colorscheme: String,
    /// Highlight group definitions.
    pub highlights: HashMap<String, HighlightDef>,
    /// Available colorschemes.
    pub available_colorschemes: Vec<String>,
}

/// A highlight group definition.
#[derive(Debug, Clone, Default)]
pub struct HighlightDef {
    /// Foreground color.
    pub fg: Option<String>,
    /// Background color.
    pub bg: Option<String>,
    /// Special color.
    pub sp: Option<String>,
    /// Bold.
    pub bold: bool,
    /// Italic.
    pub italic: bool,
    /// Underline.
    pub underline: bool,
    /// Undercurl.
    pub undercurl: bool,
    /// Strikethrough.
    pub strikethrough: bool,
    /// Reverse.
    pub reverse: bool,
    /// Standout.
    pub standout: bool,
    /// Blend.
    pub blend: Option<u8>,
    /// Linked group.
    pub link: Option<String>,
}

impl SyntaxSettings {
    /// Create default syntax settings.
    pub fn new() -> Self {
        Self {
            enabled: true,
            colorscheme: "default".to_string(),
            highlights: HashMap::new(),
            available_colorschemes: vec!["default".to_string()],
        }
    }

    /// Execute a syntax command.
    pub fn execute_syntax(&mut self, cmd: &SyntaxCommand) -> Result<String, SyntaxError> {
        match cmd {
            SyntaxCommand::On | SyntaxCommand::Enable => {
                self.enabled = true;
                Ok("Syntax highlighting enabled".to_string())
            }
            SyntaxCommand::Off => {
                self.enabled = false;
                Ok("Syntax highlighting disabled".to_string())
            }
            SyntaxCommand::Reset => {
                self.highlights.clear();
                Ok("Syntax reset".to_string())
            }
            SyntaxCommand::Clear => {
                self.highlights.clear();
                Ok("Syntax cleared".to_string())
            }
            SyntaxCommand::List(group) => {
                let result = if let Some(g) = group {
                    if let Some(def) = self.highlights.get(g) {
                        self.format_highlight(g, def)
                    } else {
                        format!("{} not defined", g)
                    }
                } else {
                    let mut lines = Vec::new();
                    for (name, def) in &self.highlights {
                        lines.push(self.format_highlight(name, def));
                    }
                    lines.join("\n")
                };
                Ok(result)
            }
        }
    }

    /// Execute a highlight command.
    pub fn execute_highlight(&mut self, cmd: &HighlightCommand) -> Result<String, SyntaxError> {
        if cmd.list {
            return if let Some(group) = &cmd.group {
                if let Some(def) = self.highlights.get(group) {
                    Ok(self.format_highlight(group, def))
                } else {
                    Ok(format!("{} not defined", group))
                }
            } else {
                let mut lines = Vec::new();
                for (name, def) in &self.highlights {
                    lines.push(self.format_highlight(name, def));
                }
                Ok(lines.join("\n"))
            };
        }

        if let Some(group) = &cmd.group {
            let def = self.highlights.entry(group.clone()).or_default();

            for attr in &cmd.attrs {
                match attr {
                    HighlightAttr::Fg(c) => def.fg = Some(c.clone()),
                    HighlightAttr::Bg(c) => def.bg = Some(c.clone()),
                    HighlightAttr::Sp(c) => def.sp = Some(c.clone()),
                    HighlightAttr::Bold(v) => def.bold = *v,
                    HighlightAttr::Italic(v) => def.italic = *v,
                    HighlightAttr::Underline(v) => def.underline = *v,
                    HighlightAttr::Undercurl(v) => def.undercurl = *v,
                    HighlightAttr::Strikethrough(v) => def.strikethrough = *v,
                    HighlightAttr::Reverse(v) => def.reverse = *v,
                    HighlightAttr::Standout(v) => def.standout = *v,
                    HighlightAttr::Blend(v) => def.blend = Some(*v),
                    HighlightAttr::NoCombine(_) => {}
                    HighlightAttr::Link(target) => def.link = Some(target.clone()),
                    HighlightAttr::Clear => *def = HighlightDef::default(),
                }
            }
        }

        Ok(String::new())
    }

    /// Execute a colorscheme command.
    pub fn execute_colorscheme(&mut self, cmd: &ColorschemeCommand) -> Result<String, SyntaxError> {
        if let Some(name) = &cmd.name {
            if self.available_colorschemes.contains(name) {
                self.colorscheme = name.clone();
                Ok(format!("Colorscheme: {}", name))
            } else {
                Err(SyntaxError::UnknownColorscheme(name.clone()))
            }
        } else {
            Ok(format!("Colorscheme: {}", self.colorscheme))
        }
    }

    fn format_highlight(&self, name: &str, def: &HighlightDef) -> String {
        let mut parts = vec![name.to_string()];

        if let Some(link) = &def.link {
            parts.push(format!("links to {}", link));
        } else {
            if let Some(fg) = &def.fg {
                parts.push(format!("guifg={}", fg));
            }
            if let Some(bg) = &def.bg {
                parts.push(format!("guibg={}", bg));
            }
            if def.bold {
                parts.push("gui=bold".to_string());
            }
            if def.italic {
                parts.push("gui=italic".to_string());
            }
        }

        parts.join(" ")
    }
}

impl Default for SyntaxSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_parse_empty() {
        let cmd = HighlightCommand::parse("").unwrap();
        assert!(cmd.list);
        assert!(cmd.group.is_none());
    }

    #[test]
    fn test_highlight_parse_group_only() {
        let cmd = HighlightCommand::parse("Comment").unwrap();
        assert!(cmd.list);
        assert_eq!(cmd.group, Some("Comment".to_string()));
    }

    #[test]
    fn test_highlight_parse_with_attrs() {
        let cmd = HighlightCommand::parse("Comment guifg=#ff0000 guibg=#000000").unwrap();
        assert_eq!(cmd.group, Some("Comment".to_string()));
        assert!(cmd.attrs.iter().any(|a| matches!(a, HighlightAttr::Fg(c) if c == "#ff0000")));
        assert!(cmd.attrs.iter().any(|a| matches!(a, HighlightAttr::Bg(c) if c == "#000000")));
    }

    #[test]
    fn test_highlight_parse_link() {
        let cmd = HighlightCommand::parse("link Constant Number").unwrap();
        assert_eq!(cmd.group, Some("Constant".to_string()));
        assert!(cmd.attrs.iter().any(|a| matches!(a, HighlightAttr::Link(t) if t == "Number")));
    }

    #[test]
    fn test_highlight_parse_default() {
        let cmd = HighlightCommand::parse("default Comment guifg=red").unwrap();
        assert!(cmd.default);
        assert_eq!(cmd.group, Some("Comment".to_string()));
    }

    #[test]
    fn test_highlight_parse_bang() {
        let cmd = HighlightCommand::parse("! Comment guifg=red").unwrap();
        assert!(cmd.bang);
        assert_eq!(cmd.group, Some("Comment".to_string()));
    }

    #[test]
    fn test_colorscheme_parse() {
        let cmd = ColorschemeCommand::parse("gruvbox");
        assert_eq!(cmd.name, Some("gruvbox".to_string()));

        let cmd = ColorschemeCommand::parse("");
        assert!(cmd.name.is_none());
    }

    #[test]
    fn test_syntax_settings_enable_disable() {
        let mut settings = SyntaxSettings::new();
        assert!(settings.enabled);

        settings.execute_syntax(&SyntaxCommand::Off).unwrap();
        assert!(!settings.enabled);

        settings.execute_syntax(&SyntaxCommand::On).unwrap();
        assert!(settings.enabled);
    }

    #[test]
    fn test_syntax_settings_highlight() {
        let mut settings = SyntaxSettings::new();
        let cmd = HighlightCommand::parse("Comment guifg=#00ff00").unwrap();
        settings.execute_highlight(&cmd).unwrap();

        let def = settings.highlights.get("Comment").unwrap();
        assert_eq!(def.fg, Some("#00ff00".to_string()));
    }

    #[test]
    fn test_syntax_settings_colorscheme() {
        let mut settings = SyntaxSettings::new();
        let result = settings.execute_colorscheme(&ColorschemeCommand::parse("")).unwrap();
        assert!(result.contains("default"));

        // Unknown colorscheme should error.
        let err = settings.execute_colorscheme(&ColorschemeCommand::parse("nonexistent"));
        assert!(err.is_err());
    }
}
