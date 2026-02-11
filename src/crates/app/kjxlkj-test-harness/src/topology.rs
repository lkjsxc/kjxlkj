use std::fs;
use std::path::{Path, PathBuf};

const REQUIRED_GROUP_ROOTS: [&str; 4] = ["app", "core", "platform", "services"];
const REQUIRED_WORKSPACE_MEMBERS: [&str; 20] = [
    "src/crates/app/kjxlkj",
    "src/crates/app/kjxlkj-test-harness",
    "src/crates/core/kjxlkj-core",
    "src/crates/core/kjxlkj-core-types",
    "src/crates/core/kjxlkj-core-text",
    "src/crates/core/kjxlkj-core-edit",
    "src/crates/core/kjxlkj-core-mode",
    "src/crates/core/kjxlkj-core-undo",
    "src/crates/core/kjxlkj-core-ui",
    "src/crates/core/kjxlkj-core-state",
    "src/crates/platform/kjxlkj-host",
    "src/crates/platform/kjxlkj-input",
    "src/crates/platform/kjxlkj-render",
    "src/crates/services/kjxlkj-services",
    "src/crates/services/kjxlkj-service-explorer",
    "src/crates/services/kjxlkj-service-fs",
    "src/crates/services/kjxlkj-service-git",
    "src/crates/services/kjxlkj-service-index",
    "src/crates/services/kjxlkj-service-lsp",
    "src/crates/services/kjxlkj-service-terminal",
];

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("manifest path depth should include repo root")
        .to_path_buf()
}

pub fn missing_group_roots(root: &Path) -> Vec<String> {
    REQUIRED_GROUP_ROOTS
        .iter()
        .filter_map(|group| {
            let path = root.join("src").join("crates").join(group);
            if path.is_dir() {
                None
            } else {
                Some(path.display().to_string())
            }
        })
        .collect()
}

pub fn missing_workspace_members(root: &Path) -> Vec<String> {
    REQUIRED_WORKSPACE_MEMBERS
        .iter()
        .filter_map(|member| {
            let path = root.join(member);
            if path.is_dir() {
                None
            } else {
                Some((*member).to_string())
            }
        })
        .collect()
}

pub fn rust_source_files_over_line_limit(root: &Path, line_limit: usize) -> Vec<String> {
    let mut files = Vec::new();
    collect_rs_files(&root.join("src"), &mut files);
    files
        .into_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(&path).ok()?;
            let line_count = source.lines().count();
            if line_count > line_limit {
                let rel = path.strip_prefix(root).ok()?.display().to_string();
                Some(format!("{rel}:{line_count}"))
            } else {
                None
            }
        })
        .collect()
}

pub fn source_directories_over_fanout_limit(root: &Path, fanout_limit: usize) -> Vec<String> {
    let mut directories = Vec::new();
    collect_directories(&root.join("src"), &mut directories);
    directories
        .into_iter()
        .filter_map(|dir| {
            let Ok(entries) = fs::read_dir(&dir) else {
                return None;
            };
            let count = entries.flatten().count();
            if count > fanout_limit {
                let rel = dir.strip_prefix(root).ok()?.display().to_string();
                Some(format!("{rel}:{count}"))
            } else {
                None
            }
        })
        .collect()
}

fn collect_rs_files(dir: &Path, output: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, output);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            output.push(path);
        }
    }
}

fn collect_directories(dir: &Path, output: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }
    output.push(dir.to_path_buf());
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_directories(&path, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_group_roots_exist() {
        let missing = missing_group_roots(&repo_root());
        assert!(
            missing.is_empty(),
            "missing grouped roots: {}",
            missing.join(", ")
        );
    }

    #[test]
    fn required_workspace_members_exist() {
        let missing = missing_workspace_members(&repo_root());
        assert!(
            missing.is_empty(),
            "missing workspace members: {}",
            missing.join(", ")
        );
    }

    #[test]
    fn rust_sources_stay_within_200_lines() {
        let over_limit = rust_source_files_over_line_limit(&repo_root(), 200);
        assert!(
            over_limit.is_empty(),
            "source files over 200 lines: {}",
            over_limit.join(", ")
        );
    }

    #[test]
    fn source_directories_stay_within_12_direct_children() {
        let over_limit = source_directories_over_fanout_limit(&repo_root(), 12);
        assert!(
            over_limit.is_empty(),
            "source directories over 12 direct children: {}",
            over_limit.join(", ")
        );
    }
}
