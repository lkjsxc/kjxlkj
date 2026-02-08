//! Regex engine abstraction and Vim magic modes.
//!
//! Provides a unified regex interface supporting Vim's
//! magic/nomagic/very-magic/very-nomagic escape modes.

/// Vim regex magic mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicMode {
    /// `\m` — default: `.`, `*`, `^`, `$` are special.
    Magic,
    /// `\M` — only `^` and `$` are special.
    NoMagic,
    /// `\v` — very magic: all ASCII symbols are special.
    VeryMagic,
    /// `\V` — very nomagic: only `\` is special.
    VeryNoMagic,
}

impl Default for MagicMode {
    fn default() -> Self {
        MagicMode::Magic
    }
}

/// A compiled regex pattern with magic mode awareness.
#[derive(Debug, Clone)]
pub struct VimRegex {
    /// Original pattern as typed.
    pub original: String,
    /// Translated pattern (Rust regex syntax).
    pub translated: String,
    /// Which magic mode was used.
    pub mode: MagicMode,
    /// Whether the pattern is case sensitive.
    pub case_sensitive: bool,
}

impl VimRegex {
    /// Compile a Vim-style regex pattern.
    pub fn compile(pattern: &str) -> Self {
        let (mode, pat) = Self::detect_mode(pattern);
        let translated = Self::translate(&pat, mode);
        let case_sensitive =
            !pat.contains("\\c") && !pattern.contains("\\c");
        Self {
            original: pattern.to_string(),
            translated,
            mode,
            case_sensitive,
        }
    }

    /// Detect magic mode prefix.
    fn detect_mode(pattern: &str) -> (MagicMode, String) {
        if let Some(rest) = pattern.strip_prefix("\\v") {
            (MagicMode::VeryMagic, rest.to_string())
        } else if let Some(rest) = pattern.strip_prefix("\\V") {
            (MagicMode::VeryNoMagic, rest.to_string())
        } else if let Some(rest) = pattern.strip_prefix("\\m") {
            (MagicMode::Magic, rest.to_string())
        } else if let Some(rest) = pattern.strip_prefix("\\M") {
            (MagicMode::NoMagic, rest.to_string())
        } else {
            (MagicMode::Magic, pattern.to_string())
        }
    }

    /// Translate Vim regex to Rust regex syntax.
    fn translate(pattern: &str, mode: MagicMode) -> String {
        match mode {
            MagicMode::VeryMagic => {
                // In very magic, most chars are special already
                // Just need to handle Vim-specific atoms
                pattern
                    .replace("\\<", "\\b")
                    .replace("\\>", "\\b")
                    .replace("\\c", "")
                    .replace("\\C", "")
            }
            MagicMode::VeryNoMagic => {
                // Almost everything is literal
                let mut result = String::new();
                let mut chars = pattern.chars().peekable();
                while let Some(ch) = chars.next() {
                    if ch == '\\' {
                        if let Some(&next) = chars.peek() {
                            match next {
                                'n' => { result.push('\n'); chars.next(); }
                                't' => { result.push('\t'); chars.next(); }
                                _ => {
                                    result.push(next);
                                    chars.next();
                                }
                            }
                        }
                    } else {
                        // Escape regex-special chars
                        if ".+*?^${}()|[]".contains(ch) {
                            result.push('\\');
                        }
                        result.push(ch);
                    }
                }
                result
            }
            MagicMode::Magic | MagicMode::NoMagic => {
                pattern
                    .replace("\\<", "\\b")
                    .replace("\\>", "\\b")
                    .replace("\\c", "")
                    .replace("\\C", "")
                    .replace("\\(", "(")
                    .replace("\\)", ")")
                    .replace("\\|", "|")
                    .replace("\\+", "+")
                    .replace("\\?", "?")
                    .replace("\\{", "{")
                    .replace("\\}", "}")
            }
        }
    }

    /// Test if this pattern matches a string.
    pub fn is_match(&self, text: &str) -> bool {
        if self.case_sensitive {
            text.contains(&self.translated)
        } else {
            text.to_lowercase()
                .contains(&self.translated.to_lowercase())
        }
    }
}

/// Regex engine configuration.
#[derive(Debug, Clone)]
pub struct RegexConfig {
    /// Default magic mode.
    pub default_mode: MagicMode,
    /// Whether to use smartcase.
    pub smartcase: bool,
    /// Whether to enable incremental search.
    pub incsearch: bool,
}

impl Default for RegexConfig {
    fn default() -> Self {
        Self {
            default_mode: MagicMode::Magic,
            smartcase: true,
            incsearch: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_magic() {
        let re = VimRegex::compile("\\mfoo.*bar");
        assert_eq!(re.mode, MagicMode::Magic);
        assert!(re.translated.contains("foo"));
    }

    #[test]
    fn compile_very_magic() {
        let re = VimRegex::compile("\\vfoo(bar|baz)");
        assert_eq!(re.mode, MagicMode::VeryMagic);
        assert!(re.translated.contains("(bar|baz)"));
    }

    #[test]
    fn compile_very_nomagic() {
        let re = VimRegex::compile("\\Vfoo.bar");
        assert_eq!(re.mode, MagicMode::VeryNoMagic);
        assert!(re.translated.contains("foo\\.bar"));
    }

    #[test]
    fn word_boundary_translation() {
        let re = VimRegex::compile("\\<word\\>");
        assert!(re.translated.contains("\\b"));
    }
}
