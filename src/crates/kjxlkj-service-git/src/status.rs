//! Git status parsing.

/// Git file status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
}

/// A status entry for a file.
#[derive(Debug, Clone)]
pub struct StatusEntry {
    pub status: FileStatus,
    pub path: String,
    pub original_path: Option<String>,
}

/// Parse `git status --porcelain=v1` output.
pub fn parse_porcelain_status(output: &str) -> Vec<StatusEntry> {
    let mut entries = Vec::new();

    for line in output.lines() {
        if line.len() < 4 {
            continue;
        }
        let index = line.as_bytes()[0];
        let worktree = line.as_bytes()[1];
        let path = line[3..].to_string();

        let status = match (index, worktree) {
            (b'?', b'?') => FileStatus::Untracked,
            (b'!', b'!') => FileStatus::Ignored,
            (b'A', _) | (_, b'A') => FileStatus::Added,
            (b'D', _) | (_, b'D') => FileStatus::Deleted,
            (b'R', _) => FileStatus::Renamed,
            (b'C', _) => FileStatus::Copied,
            _ => FileStatus::Modified,
        };

        entries.push(StatusEntry {
            status,
            path,
            original_path: None,
        });
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_untracked() {
        let output = "?? new_file.txt\n";
        let entries = parse_porcelain_status(output);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].status, FileStatus::Untracked);
    }

    #[test]
    fn parse_modified() {
        let output = " M existing.txt\n";
        let entries = parse_porcelain_status(output);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].status, FileStatus::Modified);
    }
}
