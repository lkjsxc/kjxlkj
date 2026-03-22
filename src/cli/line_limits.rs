use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::CommandResult;

const DOC_LINE_LIMIT: usize = 300;
const SOURCE_LINE_LIMIT: usize = 200;
const SOURCE_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "cxx", "go", "h", "hpp", "java", "js", "jsx", "kt", "kts", "php", "py", "rb",
    "rs", "sh", "swift", "ts", "tsx",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineScope {
    DocsMarkdown,
    SourceCode,
}

impl LineScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DocsMarkdown => "docs_markdown",
            Self::SourceCode => "source_code",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineLimitViolation {
    pub path: PathBuf,
    pub scope: LineScope,
    pub line_count: usize,
    pub limit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineLimitReport {
    pub docs_files_checked: usize,
    pub source_files_checked: usize,
    pub violations: Vec<LineLimitViolation>,
}

impl LineLimitReport {
    pub fn result(&self) -> CommandResult {
        CommandResult::from_failure_count(self.violations.len())
    }
}

pub fn scan_line_limits(docs_root: &Path, src_root: &Path) -> io::Result<LineLimitReport> {
    let docs_files = collect_files(docs_root, is_markdown_file, false)?;
    let source_files = collect_files(src_root, is_source_file, false)?;
    let mut violations = Vec::new();

    for file in &docs_files {
        let line_count = count_lines(file)?;
        if line_count > DOC_LINE_LIMIT {
            violations.push(LineLimitViolation {
                path: file.clone(),
                scope: LineScope::DocsMarkdown,
                line_count,
                limit: DOC_LINE_LIMIT,
            });
        }
    }

    for file in &source_files {
        let line_count = count_lines(file)?;
        if line_count > SOURCE_LINE_LIMIT {
            violations.push(LineLimitViolation {
                path: file.clone(),
                scope: LineScope::SourceCode,
                line_count,
                limit: SOURCE_LINE_LIMIT,
            });
        }
    }

    Ok(LineLimitReport {
        docs_files_checked: docs_files.len(),
        source_files_checked: source_files.len(),
        violations,
    })
}

fn collect_files<F>(
    root: &Path,
    include_path: F,
    allow_missing_root: bool,
) -> io::Result<Vec<PathBuf>>
where
    F: Fn(&Path) -> bool + Copy,
{
    if !root.exists() {
        if allow_missing_root {
            return Ok(Vec::new());
        }

        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("directory not found: {}", root.display()),
        ));
    }

    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path is not a directory: {}", root.display()),
        ));
    }

    let mut files = Vec::new();
    collect_files_recursive(root, include_path, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_recursive<F>(
    directory: &Path,
    include_path: F,
    files: &mut Vec<PathBuf>,
) -> io::Result<()>
where
    F: Fn(&Path) -> bool + Copy,
{
    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        if is_hidden(&entry.file_name()) {
            continue;
        }

        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_files_recursive(&path, include_path, files)?;
        } else if file_type.is_file() && include_path(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn count_lines(path: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(path)?.lines().count())
}

fn is_markdown_file(path: &Path) -> bool {
    path.extension().and_then(OsStr::to_str) == Some("md")
}

fn is_source_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(OsStr::to_str) else {
        return false;
    };

    SOURCE_EXTENSIONS
        .iter()
        .any(|allowed| extension.eq_ignore_ascii_case(allowed))
}

fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with('.')
}
