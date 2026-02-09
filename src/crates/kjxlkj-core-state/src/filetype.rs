//! Filetype detection.

pub fn filetype_from_extension(ext: &str) -> &'static str {
    match ext {
        "rs" => "rust",
        "py" | "pyw" => "python",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" | "mts" => "typescript",
        "tsx" => "typescriptreact",
        "jsx" => "javascriptreact",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "go" => "go",
        "md" | "markdown" => "markdown",
        "toml" => "toml",
        "json" | "jsonc" => "json",
        "yaml" | "yml" => "yaml",
        "lua" => "lua",
        "sh" | "bash" | "zsh" => "sh",
        "html" | "htm" => "html",
        "css" => "css",
        "scss" | "sass" => "scss",
        "xml" | "xsl" | "xslt" => "xml",
        "sql" => "sql",
        "rb" => "ruby",
        "java" => "java",
        "kt" | "kts" => "kotlin",
        "swift" => "swift",
        "zig" => "zig",
        "vim" => "vim",
        "el" | "lisp" => "lisp",
        "hs" => "haskell",
        "ex" | "exs" => "elixir",
        "erl" | "hrl" => "erlang",
        "clj" | "cljs" => "clojure",
        "dart" => "dart",
        "r" | "R" => "r",
        "ini" | "cfg" => "ini",
        "tex" | "sty" => "tex",
        "diff" | "patch" => "diff",
        "dockerfile" => "dockerfile",
        "proto" => "protobuf",
        "graphql" | "gql" => "graphql",
        _ => "",
    }
}

pub fn filetype_from_filename(name: &str) -> &'static str {
    match name {
        "Makefile" | "GNUmakefile" => "make",
        "Dockerfile" => "dockerfile",
        "Jenkinsfile" => "groovy",
        "CMakeLists.txt" => "cmake",
        "Cargo.toml" => "toml",
        "Cargo.lock" => "toml",
        ".gitignore" | ".gitattributes" => "gitconfig",
        ".bashrc" | ".bash_profile" | ".profile" => "sh",
        ".zshrc" | ".zshenv" | ".zprofile" => "sh",
        "LICENSE" | "LICENCE" => "text",
        "README" => "text",
        _ => "",
    }
}

pub fn filetype_from_shebang(line: &str) -> &'static str {
    let line = line.trim();
    if !line.starts_with("#!") {
        return "";
    }
    let rest = &line[2..].trim();
    if rest.contains("python") {
        return "python";
    }
    if rest.contains("ruby") {
        return "ruby";
    }
    if rest.contains("node") {
        return "javascript";
    }
    if rest.contains("bash") || rest.ends_with("sh") {
        return "sh";
    }
    if rest.contains("perl") {
        return "perl";
    }
    if rest.contains("lua") {
        return "lua";
    }
    ""
}

pub fn detect_filetype(path: &str, first_line: Option<&str>) -> String {
    if let Some(line) = first_line {
        let ft = filetype_from_shebang(line);
        if !ft.is_empty() {
            return ft.to_string();
        }
    }
    let name = std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ft = filetype_from_filename(&name);
    if !ft.is_empty() {
        return ft.to_string();
    }
    let ext = std::path::Path::new(path)
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    let ft = filetype_from_extension(&ext);
    if !ft.is_empty() {
        return ft.to_string();
    }
    String::new()
}

pub fn comment_string(ft: &str) -> &'static str {
    match ft {
        "rust" | "c" | "cpp" | "java" | "javascript" | "typescript" | "typescriptreact"
        | "javascriptreact" | "go" | "swift" | "kotlin" | "dart" | "zig" | "scss" | "protobuf"
        | "graphql" => "//",
        "python" | "ruby" | "sh" | "yaml" | "toml" | "make" | "cmake" | "r" | "ini"
        | "gitconfig" | "elixir" | "perl" => "#",
        "lua" | "haskell" | "sql" => "--",
        "vim" | "lisp" | "clojure" => "\"",
        "html" | "xml" => "",
        "tex" => "%",
        "erlang" => "%",
        _ => "//",
    }
}

pub fn indent_settings(ft: &str) -> (usize, bool) {
    match ft {
        "make" => (8, false), // tabs
        "go" => (4, false),   // tabs
        "yaml" | "json" | "html" | "css" | "scss" | "xml" => (2, true),
        _ => (4, true), // 4 spaces default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_rust() {
        assert_eq!(detect_filetype("main.rs", None), "rust");
    }

    #[test]
    fn detect_makefile() {
        assert_eq!(detect_filetype("Makefile", None), "make");
    }

    #[test]
    fn detect_shebang_python() {
        assert_eq!(
            detect_filetype("script", Some("#!/usr/bin/env python3")),
            "python",
        );
    }

    #[test]
    fn comment_for_rust() {
        assert_eq!(comment_string("rust"), "//");
    }

    #[test]
    fn comment_for_python() {
        assert_eq!(comment_string("python"), "#");
    }
}
