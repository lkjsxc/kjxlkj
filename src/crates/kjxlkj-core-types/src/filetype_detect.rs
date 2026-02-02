//! File type detection functions.

use super::filetype::FileType;

/// Detects file type from extension.
pub fn detect_from_extension(ext: &str) -> Option<FileType> {
    let ft = match ext.to_lowercase().as_str() {
        "rs" => "rust",
        "c" | "h" => "c",
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => "cpp",
        "py" | "pyi" | "pyw" => "python",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "mts" | "cts" => "typescript",
        "jsx" => "javascriptreact",
        "tsx" => "typescriptreact",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" => "scss",
        "less" => "less",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "xml" => "xml",
        "sh" | "bash" => "sh",
        "zsh" => "zsh",
        "fish" => "fish",
        "md" | "markdown" => "markdown",
        "lua" => "lua",
        "go" => "go",
        "java" => "java",
        "kt" | "kts" => "kotlin",
        "rb" | "rake" | "gemspec" => "ruby",
        "php" => "php",
        "swift" => "swift",
        "vim" => "vim",
        "sql" => "sql",
        "mk" => "make",
        "gitignore" | "gitattributes" => "gitconfig",
        "diff" | "patch" => "diff",
        _ => return None,
    };
    Some(FileType::new(ft))
}

/// Detects file type from filename.
pub fn detect_from_filename(name: &str) -> Option<FileType> {
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
pub fn detect_from_shebang(content: &str) -> Option<FileType> {
    let first_line = content.lines().next()?;
    if !first_line.starts_with("#!") {
        return None;
    }
    let shebang = first_line.trim_start_matches("#!");
    let interpreter = shebang.split_whitespace().next()?.rsplit('/').next()?;

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
    fn test_detect_extension_rust() {
        assert_eq!(detect_from_extension("rs").unwrap().name(), "rust");
    }

    #[test]
    fn test_detect_extension_python() {
        assert_eq!(detect_from_extension("py").unwrap().name(), "python");
    }

    #[test]
    fn test_detect_filename_makefile() {
        assert_eq!(detect_from_filename("Makefile").unwrap().name(), "make");
    }

    #[test]
    fn test_detect_shebang_python() {
        let ft = detect_from_shebang("#!/usr/bin/python3\ncode");
        assert_eq!(ft.unwrap().name(), "python");
    }

    #[test]
    fn test_detect_shebang_env() {
        let ft = detect_from_shebang("#!/usr/bin/env python\ncode");
        assert_eq!(ft.unwrap().name(), "python");
    }
}
