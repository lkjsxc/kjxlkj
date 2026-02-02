//! File type detection.

use std::path::Path;

/// Detected file type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileType {
    /// Name of the file type.
    name: String,
}

impl FileType {
    /// Creates a new file type.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Plain text file type.
    pub fn text() -> Self {
        Self::new("text")
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::text()
    }
}

/// Detects file type from path and content.
pub fn detect(path: Option<&Path>, content: Option<&str>) -> FileType {
    // Try extension first
    if let Some(path) = path {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(ft) = detect_from_extension(ext) {
                return ft;
            }
        }
        // Try filename
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(ft) = detect_from_filename(name) {
                return ft;
            }
        }
    }
    // Try shebang
    if let Some(content) = content {
        if let Some(ft) = detect_from_shebang(content) {
            return ft;
        }
    }
    FileType::text()
}

/// Detects file type from extension.
fn detect_from_extension(ext: &str) -> Option<FileType> {
    let ft = match ext.to_lowercase().as_str() {
        // Rust
        "rs" => "rust",
        // C/C++
        "c" | "h" => "c",
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => "cpp",
        // Python
        "py" | "pyi" | "pyw" => "python",
        // JavaScript/TypeScript
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "mts" | "cts" => "typescript",
        "jsx" => "javascriptreact",
        "tsx" => "typescriptreact",
        // Web
        "html" | "htm" => "html",
        "css" => "css",
        "scss" => "scss",
        "less" => "less",
        // Data
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "xml" => "xml",
        // Shell
        "sh" | "bash" => "sh",
        "zsh" => "zsh",
        "fish" => "fish",
        // Markdown
        "md" | "markdown" => "markdown",
        // Lua
        "lua" => "lua",
        // Go
        "go" => "go",
        // Java/Kotlin
        "java" => "java",
        "kt" | "kts" => "kotlin",
        // Ruby
        "rb" | "rake" | "gemspec" => "ruby",
        // PHP
        "php" => "php",
        // Swift
        "swift" => "swift",
        // Vim
        "vim" => "vim",
        // SQL
        "sql" => "sql",
        // Makefile
        "mk" => "make",
        // Git
        "gitignore" | "gitattributes" => "gitconfig",
        // Diff
        "diff" | "patch" => "diff",
        _ => return None,
    };
    Some(FileType::new(ft))
}

/// Detects file type from filename.
fn detect_from_filename(name: &str) -> Option<FileType> {
    let ft = match name {
        "Makefile" | "makefile" | "GNUmakefile" => "make",
        "Dockerfile" => "dockerfile",
        "CMakeLists.txt" => "cmake",
        "Cargo.toml" | "Cargo.lock" => "toml",
        ".gitignore" | ".gitattributes" | ".gitmodules" => "gitconfig",
        ".bashrc" | ".bash_profile" | ".profile" => "sh",
        ".zshrc" | ".zprofile" => "zsh",
        "Gemfile" | "Rakefile" => "ruby",
        "Vagrantfile" => "ruby",
        "package.json" | "tsconfig.json" => "json",
        ".eslintrc" | ".prettierrc" => "json",
        _ => return None,
    };
    Some(FileType::new(ft))
}

/// Detects file type from shebang.
fn detect_from_shebang(content: &str) -> Option<FileType> {
    let first_line = content.lines().next()?;
    if !first_line.starts_with("#!") {
        return None;
    }
    let shebang = first_line.trim_start_matches("#!");
    let interpreter = shebang
        .split_whitespace()
        .next()?
        .rsplit('/')
        .next()?;

    let ft = match interpreter {
        "sh" | "bash" | "dash" => "sh",
        "zsh" => "zsh",
        "fish" => "fish",
        "python" | "python3" | "python2" => "python",
        "ruby" => "ruby",
        "perl" => "perl",
        "node" | "nodejs" => "javascript",
        "lua" => "lua",
        "env" => {
            // Handle /usr/bin/env python etc
            let prog = shebang.split_whitespace().nth(1)?;
            return detect_from_shebang(&format!("#!{}", prog));
        }
        _ => return None,
    };
    Some(FileType::new(ft))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust() {
        let ft = detect(Some(Path::new("main.rs")), None);
        assert_eq!(ft.name(), "rust");
    }

    #[test]
    fn test_detect_python() {
        let ft = detect(Some(Path::new("script.py")), None);
        assert_eq!(ft.name(), "python");
    }

    #[test]
    fn test_detect_makefile() {
        let ft = detect(Some(Path::new("Makefile")), None);
        assert_eq!(ft.name(), "make");
    }

    #[test]
    fn test_detect_dockerfile() {
        let ft = detect(Some(Path::new("Dockerfile")), None);
        assert_eq!(ft.name(), "dockerfile");
    }

    #[test]
    fn test_detect_shebang_python() {
        let ft = detect(None, Some("#!/usr/bin/python3\nprint('hi')"));
        assert_eq!(ft.name(), "python");
    }

    #[test]
    fn test_detect_shebang_env() {
        let ft = detect(None, Some("#!/usr/bin/env python\nprint('hi')"));
        assert_eq!(ft.name(), "python");
    }

    #[test]
    fn test_detect_shebang_bash() {
        let ft = detect(None, Some("#!/bin/bash\necho hello"));
        assert_eq!(ft.name(), "sh");
    }

    #[test]
    fn test_detect_fallback() {
        let ft = detect(None, None);
        assert_eq!(ft.name(), "text");
    }
}
