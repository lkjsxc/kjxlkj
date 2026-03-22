use std::path::PathBuf;

use crate::cli::CommandResult;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermViolation {
    pub path: PathBuf,
    pub term: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermScanReport {
    pub files_checked: usize,
    pub violations: Vec<TermViolation>,
}

impl TermScanReport {
    pub fn result(&self) -> CommandResult {
        CommandResult::from_failure_count(self.violations.len())
    }
}
