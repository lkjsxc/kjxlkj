//! Filetype (language) detection from file extensions.
//! See /docs/spec/features/syntax/syntax-files.md.

/// Detect language ID from a filename or path.
pub fn detect_filetype(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("");
    match ext {
        "rs" => "rust",
        "py" => "python",
        "js" | "jsx" => "javascript",
        "ts" | "tsx" => "typescript",
        "go" => "go",
        "c" | "h" => "c",
        "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => "cpp",
        "md" => "markdown",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "html" | "htm" => "html",
        "css" => "css",
        "sh" | "bash" => "bash",
        "lua" => "lua",
        _ => "plain",
    }
}

/// Detect language from shebang line (secondary priority).
pub fn detect_from_shebang(first_line: &str) -> Option<&'static str> {
    if !first_line.starts_with("#!") { return None; }
    let line = first_line.trim();
    if line.contains("python") { return Some("python"); }
    if line.contains("node") { return Some("javascript"); }
    if line.contains("bash") || line.ends_with("/sh") { return Some("bash"); }
    if line.contains("lua") { return Some("lua"); }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ext_rust() { assert_eq!(detect_filetype("main.rs"), "rust"); }
    #[test]
    fn ext_python() { assert_eq!(detect_filetype("script.py"), "python"); }
    #[test]
    fn ext_js() { assert_eq!(detect_filetype("app.jsx"), "javascript"); }
    #[test]
    fn ext_ts() { assert_eq!(detect_filetype("index.tsx"), "typescript"); }
    #[test]
    fn ext_cpp() { assert_eq!(detect_filetype("main.cpp"), "cpp"); }
    #[test]
    fn ext_c_header() { assert_eq!(detect_filetype("stdio.h"), "c"); }
    #[test]
    fn ext_markdown() { assert_eq!(detect_filetype("README.md"), "markdown"); }
    #[test]
    fn ext_yaml() { assert_eq!(detect_filetype("config.yml"), "yaml"); }
    #[test]
    fn ext_toml() { assert_eq!(detect_filetype("Cargo.toml"), "toml"); }
    #[test]
    fn ext_html() { assert_eq!(detect_filetype("page.htm"), "html"); }
    #[test]
    fn ext_bash() { assert_eq!(detect_filetype("run.sh"), "bash"); }
    #[test]
    fn ext_lua() { assert_eq!(detect_filetype("init.lua"), "lua"); }
    #[test]
    fn ext_unknown() { assert_eq!(detect_filetype("file.xyz"), "plain"); }
    #[test]
    fn ext_no_ext() { assert_eq!(detect_filetype("Makefile"), "plain"); }
    #[test]
    fn ext_with_path() { assert_eq!(detect_filetype("src/main.rs"), "rust"); }
    #[test]
    fn shebang_python() { assert_eq!(detect_from_shebang("#!/usr/bin/env python3"), Some("python")); }
    #[test]
    fn shebang_bash() { assert_eq!(detect_from_shebang("#!/bin/sh"), Some("bash")); }
    #[test]
    fn shebang_node() { assert_eq!(detect_from_shebang("#!/usr/bin/env node"), Some("javascript")); }
    #[test]
    fn shebang_none() { assert_eq!(detect_from_shebang("hello world"), None); }
    #[test]
    fn shebang_lua() { assert_eq!(detect_from_shebang("#!/usr/bin/lua"), Some("lua")); }
}
