//! Language detection from file extensions.

/// Known language identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Plain,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    C,
    Cpp,
    Go,
    Java,
    Ruby,
    Shell,
    Lua,
    Markdown,
    Json,
    Yaml,
    Toml,
    Html,
    Css,
    Xml,
    Sql,
    Zig,
    Haskell,
    Elixir,
    Svelte,
    Vue,
    Dockerfile,
    Makefile,
}

impl LanguageId {
    /// Detect language from a file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "py" | "pyw" | "pyi" => Self::Python,
            "js" | "mjs" | "cjs" | "jsx" => Self::JavaScript,
            "ts" | "mts" | "cts" | "tsx" => Self::TypeScript,
            "c" | "h" => Self::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => {
                Self::Cpp
            }
            "go" => Self::Go,
            "java" => Self::Java,
            "rb" | "erb" => Self::Ruby,
            "sh" | "bash" | "zsh" | "fish" => Self::Shell,
            "lua" => Self::Lua,
            "md" | "markdown" => Self::Markdown,
            "json" | "jsonc" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "html" | "htm" => Self::Html,
            "css" | "scss" | "sass" | "less" => Self::Css,
            "xml" | "xsl" | "xslt" => Self::Xml,
            "sql" => Self::Sql,
            "zig" => Self::Zig,
            "hs" | "lhs" => Self::Haskell,
            "ex" | "exs" => Self::Elixir,
            "svelte" => Self::Svelte,
            "vue" => Self::Vue,
            _ => Self::Plain,
        }
    }

    /// Detect language from a filename (handles special names).
    pub fn from_filename(name: &str) -> Self {
        // Check special filenames first
        let lower = name.to_lowercase();
        match lower.as_str() {
            "dockerfile" | "containerfile" => {
                return Self::Dockerfile
            }
            "makefile" | "gnumakefile" => {
                return Self::Makefile
            }
            _ => {}
        }
        // Try extension
        if let Some(ext) = name.rsplit('.').next() {
            if ext != name {
                return Self::from_extension(ext);
            }
        }
        Self::Plain
    }

    /// Detect language from a full file path.
    pub fn detect(path: &str) -> Self {
        let name = std::path::Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path);
        Self::from_filename(name)
    }

    /// LSP language identifier string.
    pub fn lsp_id(&self) -> &'static str {
        match self {
            Self::Plain => "plaintext",
            Self::Rust => "rust",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Go => "go",
            Self::Java => "java",
            Self::Ruby => "ruby",
            Self::Shell => "shellscript",
            Self::Lua => "lua",
            Self::Markdown => "markdown",
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
            Self::Html => "html",
            Self::Css => "css",
            Self::Xml => "xml",
            Self::Sql => "sql",
            Self::Zig => "zig",
            Self::Haskell => "haskell",
            Self::Elixir => "elixir",
            Self::Svelte => "svelte",
            Self::Vue => "vue",
            Self::Dockerfile => "dockerfile",
            Self::Makefile => "makefile",
        }
    }
}

impl std::fmt::Display for LanguageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lsp_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_extension() {
        assert_eq!(
            LanguageId::from_extension("rs"),
            LanguageId::Rust,
        );
    }

    #[test]
    fn c_header() {
        assert_eq!(
            LanguageId::from_extension("h"),
            LanguageId::C,
        );
    }

    #[test]
    fn cpp_extensions() {
        assert_eq!(
            LanguageId::from_extension("cpp"),
            LanguageId::Cpp,
        );
        assert_eq!(
            LanguageId::from_extension("hpp"),
            LanguageId::Cpp,
        );
    }

    #[test]
    fn unknown_fallback() {
        assert_eq!(
            LanguageId::from_extension("xyz"),
            LanguageId::Plain,
        );
    }

    #[test]
    fn dockerfile() {
        assert_eq!(
            LanguageId::from_filename("Dockerfile"),
            LanguageId::Dockerfile,
        );
    }

    #[test]
    fn makefile() {
        assert_eq!(
            LanguageId::from_filename("Makefile"),
            LanguageId::Makefile,
        );
    }

    #[test]
    fn filename_with_ext() {
        assert_eq!(
            LanguageId::from_filename("main.py"),
            LanguageId::Python,
        );
    }

    #[test]
    fn lsp_id() {
        assert_eq!(LanguageId::Rust.lsp_id(), "rust");
        assert_eq!(LanguageId::Plain.lsp_id(), "plaintext");
    }

    #[test]
    fn display_trait() {
        assert_eq!(format!("{}", LanguageId::Go), "go");
    }
}
