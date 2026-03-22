use std::fs;
use std::io;
use std::path::Path;

use super::collect::{collect_files, count_lines, is_markdown_file, is_source_file};
use super::model::{LineLimitReport, LineLimitViolation, LineScope, TermScanReport, TermViolation};

const DOC_LINE_LIMIT: usize = 300;
const SOURCE_LINE_LIMIT: usize = 200;

pub fn scan_line_limits(docs_root: &Path, src_root: &Path) -> io::Result<LineLimitReport> {
    let docs_files = collect_files(docs_root, is_markdown_file, false)?;
    let source_files = collect_files(src_root, is_source_file, false)?;
    let mut violations = Vec::new();
    for file in &docs_files {
        push_violation_if_needed(
            &mut violations,
            file,
            LineScope::DocsMarkdown,
            DOC_LINE_LIMIT,
        )?;
    }
    for file in &source_files {
        push_violation_if_needed(
            &mut violations,
            file,
            LineScope::SourceCode,
            SOURCE_LINE_LIMIT,
        )?;
    }
    Ok(LineLimitReport {
        docs_files_checked: docs_files.len(),
        source_files_checked: source_files.len(),
        violations,
    })
}

pub fn scan_text_for_terms(root: &Path, terms: &[&str]) -> io::Result<TermScanReport> {
    let files = collect_files(root, |_| true, false)?;
    let mut violations = Vec::new();
    let normalized_terms = terms
        .iter()
        .map(|term| term.to_ascii_lowercase())
        .collect::<Vec<_>>();
    for file in &files {
        let text = fs::read_to_string(file)?;
        for (index, line) in text.lines().enumerate() {
            let lowered = line.to_ascii_lowercase();
            for term in &normalized_terms {
                if lowered.contains(term) {
                    violations.push(TermViolation {
                        path: file.clone(),
                        term: term.clone(),
                        line: index + 1,
                    });
                }
            }
        }
    }
    Ok(TermScanReport {
        files_checked: files.len(),
        violations,
    })
}

fn push_violation_if_needed(
    violations: &mut Vec<LineLimitViolation>,
    file: &Path,
    scope: LineScope,
    limit: usize,
) -> io::Result<()> {
    let line_count = count_lines(file)?;
    if line_count > limit {
        violations.push(LineLimitViolation {
            path: file.to_path_buf(),
            scope,
            line_count,
            limit,
        });
    }
    Ok(())
}
