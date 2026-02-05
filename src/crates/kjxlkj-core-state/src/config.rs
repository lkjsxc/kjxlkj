//! Configuration features: filetype detection and editor options.
//!
//! Implements filetype detection, per-filetype settings, and editor options.

use std::collections::HashMap;
use std::path::Path;

/// Detected filetype.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Filetype(pub String);

impl Filetype {
    /// Create a new filetype.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the filetype name.
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Default for Filetype {
    fn default() -> Self {
        Self::new("")
    }
}

/// Filetype detection result.
#[derive(Debug, Clone)]
pub struct FiletypeDetection {
    /// Detected filetype.
    pub filetype: Filetype,
    /// Detection method used.
    pub method: DetectionMethod,
}

/// How the filetype was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Detected by file extension.
    Extension,
    /// Detected by filename.
    Filename,
    /// Detected by shebang.
    Shebang,
    /// Detected by content.
    Content,
    /// Set by modeline.
    Modeline,
    /// Manually set.
    Manual,
    /// Unknown/default.
    Unknown,
}

/// Filetype detector.
#[derive(Debug, Default)]
pub struct FiletypeDetector {
    /// Extension to filetype mapping.
    extensions: HashMap<String, Filetype>,
    /// Filename to filetype mapping.
    filenames: HashMap<String, Filetype>,
    /// Shebang patterns.
    shebangs: Vec<(String, Filetype)>,
}

impl FiletypeDetector {
    /// Create a new filetype detector with defaults.
    pub fn new() -> Self {
        let mut detector = Self::default();
        detector.register_defaults();
        detector
    }

    fn register_defaults(&mut self) {
        // Common extensions.
        let exts = [
            ("rs", "rust"), ("py", "python"), ("js", "javascript"),
            ("ts", "typescript"), ("jsx", "javascriptreact"),
            ("tsx", "typescriptreact"), ("md", "markdown"),
            ("json", "json"), ("yaml", "yaml"), ("yml", "yaml"),
            ("toml", "toml"), ("html", "html"), ("css", "css"),
            ("scss", "scss"), ("lua", "lua"), ("vim", "vim"),
            ("sh", "sh"), ("bash", "bash"), ("zsh", "zsh"),
            ("c", "c"), ("h", "c"), ("cpp", "cpp"), ("hpp", "cpp"),
            ("go", "go"), ("rb", "ruby"), ("java", "java"),
            ("kt", "kotlin"), ("swift", "swift"), ("zig", "zig"),
            ("txt", "text"), ("log", "text"),
        ];

        for (ext, ft) in exts {
            self.extensions.insert(ext.to_string(), Filetype::new(ft));
        }

        // Common filenames.
        let names = [
            ("Makefile", "make"), ("makefile", "make"),
            ("Dockerfile", "dockerfile"), ("CMakeLists.txt", "cmake"),
            (".gitignore", "gitignore"), (".gitconfig", "gitconfig"),
            ("Cargo.toml", "toml"), ("package.json", "json"),
            (".bashrc", "bash"), (".zshrc", "zsh"),
        ];

        for (name, ft) in names {
            self.filenames.insert(name.to_string(), Filetype::new(ft));
        }

        // Shebang patterns.
        self.shebangs.push(("python".to_string(), Filetype::new("python")));
        self.shebangs.push(("bash".to_string(), Filetype::new("bash")));
        self.shebangs.push(("sh".to_string(), Filetype::new("sh")));
        self.shebangs.push(("zsh".to_string(), Filetype::new("zsh")));
        self.shebangs.push(("node".to_string(), Filetype::new("javascript")));
        self.shebangs.push(("ruby".to_string(), Filetype::new("ruby")));
        self.shebangs.push(("perl".to_string(), Filetype::new("perl")));
    }

    /// Register a new extension mapping.
    pub fn register_extension(&mut self, ext: impl Into<String>, ft: Filetype) {
        self.extensions.insert(ext.into(), ft);
    }

    /// Register a new filename mapping.
    pub fn register_filename(&mut self, name: impl Into<String>, ft: Filetype) {
        self.filenames.insert(name.into(), ft);
    }

    /// Detect filetype from path.
    pub fn detect_by_path(&self, path: &Path) -> Option<FiletypeDetection> {
        // Try filename first.
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(ft) = self.filenames.get(name) {
                return Some(FiletypeDetection {
                    filetype: ft.clone(),
                    method: DetectionMethod::Filename,
                });
            }
        }

        // Try extension.
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(ft) = self.extensions.get(ext) {
                return Some(FiletypeDetection {
                    filetype: ft.clone(),
                    method: DetectionMethod::Extension,
                });
            }
        }

        None
    }

    /// Detect filetype from first line (shebang).
    pub fn detect_by_shebang(&self, first_line: &str) -> Option<FiletypeDetection> {
        if !first_line.starts_with("#!") {
            return None;
        }

        for (pattern, ft) in &self.shebangs {
            if first_line.contains(pattern) {
                return Some(FiletypeDetection {
                    filetype: ft.clone(),
                    method: DetectionMethod::Shebang,
                });
            }
        }

        None
    }

    /// Detect filetype from path and content.
    pub fn detect(&self, path: &Path, first_line: Option<&str>) -> Option<FiletypeDetection> {
        // Try shebang first if content available.
        if let Some(line) = first_line {
            if let Some(result) = self.detect_by_shebang(line) {
                return Some(result);
            }
        }

        // Try path-based detection.
        self.detect_by_path(path)
    }
}

/// Editor options (vim-style set options).
#[derive(Debug, Clone)]
pub struct EditorOptions {
    /// Tab width.
    pub tabstop: usize,
    /// Shift width.
    pub shiftwidth: usize,
    /// Expand tabs to spaces.
    pub expandtab: bool,
    /// Auto-indent.
    pub autoindent: bool,
    /// Smart indent.
    pub smartindent: bool,
    /// Show line numbers.
    pub number: bool,
    /// Show relative line numbers.
    pub relativenumber: bool,
    /// Wrap lines.
    pub wrap: bool,
    /// Scroll offset.
    pub scrolloff: usize,
    /// Side scroll offset.
    pub sidescrolloff: usize,
    /// Show cursor line.
    pub cursorline: bool,
    /// Show cursor column.
    pub cursorcolumn: bool,
    /// Ignore case in search.
    pub ignorecase: bool,
    /// Smart case (ignore case unless upper present).
    pub smartcase: bool,
    /// Highlight search.
    pub hlsearch: bool,
    /// Incremental search.
    pub incsearch: bool,
    /// Show matching brackets.
    pub showmatch: bool,
    /// Hidden buffers.
    pub hidden: bool,
    /// Backup files.
    pub backup: bool,
    /// Swap files.
    pub swapfile: bool,
    /// Undo files.
    pub undofile: bool,
}

impl Default for EditorOptions {
    fn default() -> Self {
        Self {
            tabstop: 8,
            shiftwidth: 8,
            expandtab: false,
            autoindent: true,
            smartindent: false,
            number: false,
            relativenumber: false,
            wrap: true,
            scrolloff: 0,
            sidescrolloff: 0,
            cursorline: false,
            cursorcolumn: false,
            ignorecase: false,
            smartcase: false,
            hlsearch: true,
            incsearch: true,
            showmatch: false,
            hidden: false,
            backup: false,
            swapfile: true,
            undofile: false,
        }
    }
}

impl EditorOptions {
    /// Create new editor options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set an option by name.
    pub fn set(&mut self, name: &str, value: bool) -> bool {
        match name {
            "expandtab" | "et" => self.expandtab = value,
            "autoindent" | "ai" => self.autoindent = value,
            "smartindent" | "si" => self.smartindent = value,
            "number" | "nu" => self.number = value,
            "relativenumber" | "rnu" => self.relativenumber = value,
            "wrap" => self.wrap = value,
            "cursorline" | "cul" => self.cursorline = value,
            "cursorcolumn" | "cuc" => self.cursorcolumn = value,
            "ignorecase" | "ic" => self.ignorecase = value,
            "smartcase" | "scs" => self.smartcase = value,
            "hlsearch" | "hls" => self.hlsearch = value,
            "incsearch" | "is" => self.incsearch = value,
            "showmatch" | "sm" => self.showmatch = value,
            "hidden" | "hid" => self.hidden = value,
            "backup" | "bk" => self.backup = value,
            "swapfile" | "swf" => self.swapfile = value,
            "undofile" | "udf" => self.undofile = value,
            _ => return false,
        }
        true
    }

    /// Set a numeric option by name.
    pub fn set_num(&mut self, name: &str, value: usize) -> bool {
        match name {
            "tabstop" | "ts" => self.tabstop = value,
            "shiftwidth" | "sw" => self.shiftwidth = value,
            "scrolloff" | "so" => self.scrolloff = value,
            "sidescrolloff" | "siso" => self.sidescrolloff = value,
            _ => return false,
        }
        true
    }

    /// Get a boolean option value.
    pub fn get(&self, name: &str) -> Option<bool> {
        match name {
            "expandtab" | "et" => Some(self.expandtab),
            "autoindent" | "ai" => Some(self.autoindent),
            "smartindent" | "si" => Some(self.smartindent),
            "number" | "nu" => Some(self.number),
            "relativenumber" | "rnu" => Some(self.relativenumber),
            "wrap" => Some(self.wrap),
            "cursorline" | "cul" => Some(self.cursorline),
            "cursorcolumn" | "cuc" => Some(self.cursorcolumn),
            "ignorecase" | "ic" => Some(self.ignorecase),
            "smartcase" | "scs" => Some(self.smartcase),
            "hlsearch" | "hls" => Some(self.hlsearch),
            "incsearch" | "is" => Some(self.incsearch),
            "showmatch" | "sm" => Some(self.showmatch),
            "hidden" | "hid" => Some(self.hidden),
            "backup" | "bk" => Some(self.backup),
            "swapfile" | "swf" => Some(self.swapfile),
            "undofile" | "udf" => Some(self.undofile),
            _ => None,
        }
    }

    /// Get a numeric option value.
    pub fn get_num(&self, name: &str) -> Option<usize> {
        match name {
            "tabstop" | "ts" => Some(self.tabstop),
            "shiftwidth" | "sw" => Some(self.shiftwidth),
            "scrolloff" | "so" => Some(self.scrolloff),
            "sidescrolloff" | "siso" => Some(self.sidescrolloff),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filetype_new() {
        let ft = Filetype::new("rust");
        assert_eq!(ft.name(), "rust");
    }

    #[test]
    fn test_detector_extension() {
        let detector = FiletypeDetector::new();
        let result = detector.detect_by_path(&PathBuf::from("test.rs")).unwrap();
        assert_eq!(result.filetype.name(), "rust");
        assert_eq!(result.method, DetectionMethod::Extension);
    }

    #[test]
    fn test_detector_filename() {
        let detector = FiletypeDetector::new();
        let result = detector.detect_by_path(&PathBuf::from("Makefile")).unwrap();
        assert_eq!(result.filetype.name(), "make");
        assert_eq!(result.method, DetectionMethod::Filename);
    }

    #[test]
    fn test_detector_shebang() {
        let detector = FiletypeDetector::new();
        let result = detector.detect_by_shebang("#!/usr/bin/env python3").unwrap();
        assert_eq!(result.filetype.name(), "python");
        assert_eq!(result.method, DetectionMethod::Shebang);
    }

    #[test]
    fn test_detector_shebang_bash() {
        let detector = FiletypeDetector::new();
        let result = detector.detect_by_shebang("#!/bin/bash").unwrap();
        assert_eq!(result.filetype.name(), "bash");
    }

    #[test]
    fn test_detector_combined() {
        let detector = FiletypeDetector::new();
        // Shebang takes precedence.
        let path = PathBuf::from("script");
        let result = detector.detect(&path, Some("#!/usr/bin/env python")).unwrap();
        assert_eq!(result.filetype.name(), "python");
    }

    #[test]
    fn test_editor_options_default() {
        let opts = EditorOptions::default();
        assert_eq!(opts.tabstop, 8);
        assert!(!opts.expandtab);
        assert!(opts.autoindent);
    }

    #[test]
    fn test_editor_options_set() {
        let mut opts = EditorOptions::new();
        assert!(opts.set("expandtab", true));
        assert!(opts.expandtab);
        assert!(opts.set("et", false));
        assert!(!opts.expandtab);
    }

    #[test]
    fn test_editor_options_set_num() {
        let mut opts = EditorOptions::new();
        assert!(opts.set_num("tabstop", 4));
        assert_eq!(opts.tabstop, 4);
        assert!(opts.set_num("ts", 2));
        assert_eq!(opts.tabstop, 2);
    }

    #[test]
    fn test_editor_options_get() {
        let opts = EditorOptions::new();
        assert_eq!(opts.get("expandtab"), Some(false));
        assert_eq!(opts.get_num("tabstop"), Some(8));
        assert_eq!(opts.get("invalid"), None);
    }
}
