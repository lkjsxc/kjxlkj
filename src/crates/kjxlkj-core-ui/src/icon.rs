//! Icon types for the editor UI.
//!
//! Implements icons as specified in `/docs/spec/features/ui/icons.md`.

/// Icon kind/category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconKind {
    /// File type icon.
    FileType,
    /// Directory icon.
    Directory,
    /// Git status icon.
    Git,
    /// Diagnostic icon.
    Diagnostic,
    /// UI navigation icon.
    Navigation,
    /// Action icon.
    Action,
    /// Arrow/direction icon.
    Arrow,
}

/// File type for icon lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    /// Rust source file.
    Rust,
    /// JavaScript file.
    JavaScript,
    /// TypeScript file.
    TypeScript,
    /// Python file.
    Python,
    /// Go file.
    Go,
    /// Lua file.
    Lua,
    /// C file.
    C,
    /// C++ file.
    Cpp,
    /// Java file.
    Java,
    /// Ruby file.
    Ruby,
    /// JSON file.
    Json,
    /// YAML file.
    Yaml,
    /// TOML file.
    Toml,
    /// Markdown file.
    Markdown,
    /// HTML file.
    Html,
    /// CSS file.
    Css,
    /// Shell script.
    Shell,
    /// Git file.
    Git,
    /// Docker file.
    Docker,
    /// Config file.
    Config,
    /// Lock file.
    Lock,
    /// Generic/unknown file.
    Generic,
}

impl FileType {
    /// Detect file type from extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "js" | "mjs" | "cjs" => Self::JavaScript,
            "ts" | "tsx" => Self::TypeScript,
            "py" | "pyw" => Self::Python,
            "go" => Self::Go,
            "lua" => Self::Lua,
            "c" | "h" => Self::C,
            "cpp" | "cxx" | "cc" | "hpp" => Self::Cpp,
            "java" => Self::Java,
            "rb" => Self::Ruby,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "md" | "markdown" => Self::Markdown,
            "html" | "htm" => Self::Html,
            "css" | "scss" | "sass" | "less" => Self::Css,
            "sh" | "bash" | "zsh" | "fish" => Self::Shell,
            "gitignore" | "gitattributes" => Self::Git,
            "dockerfile" => Self::Docker,
            "lock" => Self::Lock,
            "ini" | "cfg" | "conf" => Self::Config,
            _ => Self::Generic,
        }
    }

    /// Get icon for this file type.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Rust => "",
            Self::JavaScript => "",
            Self::TypeScript => "",
            Self::Python => "",
            Self::Go => "",
            Self::Lua => "",
            Self::C => "",
            Self::Cpp => "",
            Self::Java => "",
            Self::Ruby => "",
            Self::Json => "",
            Self::Yaml => "",
            Self::Toml => "",
            Self::Markdown => "",
            Self::Html => "",
            Self::Css => "",
            Self::Shell => "",
            Self::Git => "",
            Self::Docker => "",
            Self::Config => "",
            Self::Lock => "",
            Self::Generic => "",
        }
    }

    /// Get ASCII fallback icon.
    pub fn icon_ascii(&self) -> &'static str {
        "[f]"
    }
}

/// Git status for icon lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    /// Clean/unchanged.
    Clean,
    /// Modified.
    Modified,
    /// Staged.
    Staged,
    /// Deleted.
    Deleted,
    /// Untracked.
    Untracked,
    /// Renamed.
    Renamed,
    /// Conflict.
    Conflict,
    /// Ignored.
    Ignored,
}

impl GitStatus {
    /// Get icon for this status.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Clean => "✓",
            Self::Modified => "●",
            Self::Staged => "✚",
            Self::Deleted => "✖",
            Self::Untracked => "?",
            Self::Renamed => "➜",
            Self::Conflict => "═",
            Self::Ignored => "◌",
        }
    }

    /// Get single character for gutter.
    pub fn gutter_char(&self) -> char {
        match self {
            Self::Clean => ' ',
            Self::Modified => 'M',
            Self::Staged => 'A',
            Self::Deleted => 'D',
            Self::Untracked => '?',
            Self::Renamed => 'R',
            Self::Conflict => 'C',
            Self::Ignored => '!',
        }
    }
}

/// Diagnostic severity for icon lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    /// Error.
    Error,
    /// Warning.
    Warning,
    /// Information.
    Information,
    /// Hint.
    Hint,
}

impl DiagnosticLevel {
    /// Get icon for this level.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Error => "",
            Self::Warning => "",
            Self::Information => "",
            Self::Hint => "",
        }
    }

    /// Get ASCII fallback icon.
    pub fn icon_ascii(&self) -> &'static str {
        match self {
            Self::Error => "[E]",
            Self::Warning => "[W]",
            Self::Information => "[I]",
            Self::Hint => "[H]",
        }
    }
}

/// Directory icon type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectoryIcon {
    /// Closed folder.
    Closed,
    /// Open folder.
    Open,
    /// Git folder.
    Git,
    /// Node modules.
    NodeModules,
    /// Source directory.
    Source,
    /// Tests directory.
    Tests,
}

impl DirectoryIcon {
    /// Get icon for this directory type.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Closed => "",
            Self::Open => "",
            Self::Git => "",
            Self::NodeModules => "",
            Self::Source => "",
            Self::Tests => "",
        }
    }

    /// Get ASCII fallback.
    pub fn icon_ascii(&self) -> &'static str {
        match self {
            Self::Closed => "[+]",
            Self::Open => "[-]",
            _ => "[d]",
        }
    }
}

/// UI action icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionIcon {
    /// Close.
    Close,
    /// Add/new.
    Add,
    /// Edit.
    Edit,
    /// Save.
    Save,
    /// Refresh.
    Refresh,
    /// Play/run.
    Play,
    /// Search.
    Search,
    /// Settings.
    Settings,
}

impl ActionIcon {
    /// Get icon.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Close => "",
            Self::Add => "",
            Self::Edit => "",
            Self::Save => "",
            Self::Refresh => "",
            Self::Play => "",
            Self::Search => "",
            Self::Settings => "",
        }
    }

    /// Get ASCII fallback.
    pub fn icon_ascii(&self) -> &'static str {
        match self {
            Self::Close => "[x]",
            Self::Add => "[+]",
            Self::Edit => "[e]",
            Self::Save => "[s]",
            Self::Refresh => "[r]",
            Self::Play => "[>]",
            Self::Search => "[?]",
            Self::Settings => "[*]",
        }
    }
}

/// Arrow/direction icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowIcon {
    /// Expand/right.
    Expand,
    /// Collapse/down.
    Collapse,
    /// Right arrow.
    Right,
    /// Left arrow.
    Left,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
}

impl ArrowIcon {
    /// Get icon.
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Expand => "",
            Self::Collapse => "",
            Self::Right => "",
            Self::Left => "",
            Self::Up => "",
            Self::Down => "",
        }
    }

    /// Get ASCII fallback.
    pub fn icon_ascii(&self) -> &'static str {
        match self {
            Self::Expand => "[>]",
            Self::Collapse => "[v]",
            Self::Right => "->",
            Self::Left => "<-",
            Self::Up => "^",
            Self::Down => "v",
        }
    }
}

/// Icon configuration.
#[derive(Debug, Clone, Default)]
pub struct IconConfig {
    /// Use nerd font icons.
    pub nerd_fonts: bool,
    /// Use colored icons.
    pub colored: bool,
}

impl IconConfig {
    /// Create config with nerd fonts enabled.
    pub fn with_nerd_fonts() -> Self {
        Self {
            nerd_fonts: true,
            colored: true,
        }
    }

    /// Create ASCII-only config.
    pub fn ascii_only() -> Self {
        Self {
            nerd_fonts: false,
            colored: false,
        }
    }

    /// Get file icon based on config.
    pub fn file_icon(&self, file_type: &FileType) -> &'static str {
        if self.nerd_fonts {
            file_type.icon()
        } else {
            file_type.icon_ascii()
        }
    }

    /// Get diagnostic icon based on config.
    pub fn diagnostic_icon(&self, level: DiagnosticLevel) -> &'static str {
        if self.nerd_fonts {
            level.icon()
        } else {
            level.icon_ascii()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension("rs"), FileType::Rust);
        assert_eq!(FileType::from_extension("py"), FileType::Python);
        assert_eq!(FileType::from_extension("ts"), FileType::TypeScript);
        assert_eq!(FileType::from_extension("unknown"), FileType::Generic);
    }

    #[test]
    fn test_file_type_icon() {
        // Icon may be nerd font character - just check it returns something
        let _ = FileType::Rust.icon();
        let _ = FileType::Python.icon();
    }

    #[test]
    fn test_git_status_icon() {
        // Git status icons should exist
        let icon = GitStatus::Modified.icon();
        let _ = icon; // May be Unicode character
        assert_eq!(GitStatus::Modified.gutter_char(), 'M');
    }

    #[test]
    fn test_diagnostic_level_icon() {
        // Diagnostic icons exist
        let _ = DiagnosticLevel::Error.icon();
        assert!(!DiagnosticLevel::Error.icon_ascii().is_empty());
    }

    #[test]
    fn test_directory_icon() {
        // Directory icons exist
        let _ = DirectoryIcon::Closed.icon();
        let _ = DirectoryIcon::Open.icon();
    }

    #[test]
    fn test_action_icon() {
        // Action icons exist
        let _ = ActionIcon::Close.icon();
        let _ = ActionIcon::Search.icon();
    }

    #[test]
    fn test_arrow_icon() {
        // Arrow icons exist
        let _ = ArrowIcon::Expand.icon();
        let _ = ArrowIcon::Collapse.icon();
    }

    #[test]
    fn test_icon_config_default() {
        let config = IconConfig::default();
        assert!(!config.nerd_fonts);
    }

    #[test]
    fn test_icon_config_with_nerd_fonts() {
        let config = IconConfig::with_nerd_fonts();
        assert!(config.nerd_fonts);
        assert!(config.colored);
    }

    #[test]
    fn test_icon_config_ascii_only() {
        let config = IconConfig::ascii_only();
        assert!(!config.nerd_fonts);
    }

    #[test]
    fn test_icon_config_file_icon() {
        let config = IconConfig::with_nerd_fonts();
        let _ = config.file_icon(&FileType::Rust);
    }

    #[test]
    fn test_icon_config_diagnostic_icon() {
        let config = IconConfig::ascii_only();
        let icon = config.diagnostic_icon(DiagnosticLevel::Error);
        assert!(icon.contains("[E]"));
    }

    #[test]
    fn test_file_type_case_insensitive() {
        assert_eq!(FileType::from_extension("RS"), FileType::Rust);
        assert_eq!(FileType::from_extension("Py"), FileType::Python);
    }
}
