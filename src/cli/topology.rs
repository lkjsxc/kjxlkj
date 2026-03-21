use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::CommandResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyRule {
    ExactlyOneReadme,
    MultipleChildren,
}

impl TopologyRule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactlyOneReadme => "exactly_one_readme",
            Self::MultipleChildren => "multiple_children",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyViolation {
    pub path: PathBuf,
    pub rule: TopologyRule,
    pub expected: usize,
    pub actual: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReport {
    pub directories_checked: usize,
    pub violations: Vec<TopologyViolation>,
}

impl TopologyReport {
    pub fn result(&self) -> CommandResult {
        CommandResult::from_failure_count(self.violations.len())
    }
}

pub fn scan_docs_topology(root: &Path) -> io::Result<TopologyReport> {
    let directories = collect_directories(root)?;
    let mut violations = Vec::new();

    for directory in &directories {
        let children = collect_visible_children(directory)?;
        let readme_count = children
            .iter()
            .filter(|path| path.is_file() && path.file_name() == Some(OsStr::new("README.md")))
            .count();

        if readme_count != 1 {
            violations.push(TopologyViolation {
                path: directory.clone(),
                rule: TopologyRule::ExactlyOneReadme,
                expected: 1,
                actual: readme_count,
            });
        }

        if children.len() < 2 {
            violations.push(TopologyViolation {
                path: directory.clone(),
                rule: TopologyRule::MultipleChildren,
                expected: 2,
                actual: children.len(),
            });
        }
    }

    Ok(TopologyReport {
        directories_checked: directories.len(),
        violations,
    })
}

fn collect_directories(root: &Path) -> io::Result<Vec<PathBuf>> {
    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("directory not found: {}", root.display()),
        ));
    }

    let mut directories = vec![root.to_path_buf()];
    let mut index = 0;

    while index < directories.len() {
        let child_dirs = collect_visible_children(&directories[index])?
            .into_iter()
            .filter(|path| path.is_dir())
            .collect::<Vec<_>>();

        directories.extend(child_dirs);
        index += 1;
    }

    directories.sort();
    Ok(directories)
}

fn collect_visible_children(directory: &Path) -> io::Result<Vec<PathBuf>> {
    let mut children = Vec::new();

    for entry_result in fs::read_dir(directory)? {
        let entry = entry_result?;
        if is_hidden(&entry.file_name()) {
            continue;
        }

        children.push(entry.path());
    }

    children.sort();
    Ok(children)
}

fn is_hidden(name: &OsStr) -> bool {
    name.to_string_lossy().starts_with('.')
}
