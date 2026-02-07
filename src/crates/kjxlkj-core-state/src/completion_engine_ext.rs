//! Completion helpers: command, path, and option completers.

/// Complete command names matching a prefix.
pub fn complete_commands(prefix: &str) -> Vec<String> {
    crate::completion_engine::COMMANDS
        .iter()
        .filter(|c| c.starts_with(prefix))
        .map(|c| c.to_string())
        .collect()
}

/// Complete file paths matching a prefix.
pub fn complete_paths(prefix: &str) -> Vec<String> {
    let dir = if prefix.is_empty() {
        "."
    } else {
        let p = std::path::Path::new(prefix);
        if p.is_dir() {
            prefix
        } else {
            p.parent().map(|p| p.to_str().unwrap_or(".")).unwrap_or(".")
        }
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut results = Vec::new();
    for entry in entries.flatten() {
        let name = entry.path().to_string_lossy().to_string();
        if name.starts_with(prefix) || prefix.is_empty() {
            results.push(name);
        }
    }
    results.sort();
    results
}

/// Complete editor option names matching a prefix.
pub fn complete_options(prefix: &str) -> Vec<String> {
    let opts = [
        "number",
        "relativenumber",
        "wrap",
        "tabstop",
        "shiftwidth",
        "expandtab",
        "scrolloff",
        "ignorecase",
        "smartcase",
        "hlsearch",
        "incsearch",
        "autoindent",
        "smartindent",
        "syntax",
        "cursorline",
    ];
    opts.iter()
        .filter(|o| o.starts_with(prefix))
        .map(|o| o.to_string())
        .collect()
}

/// Compute the longest common prefix among candidates.
pub fn common_prefix(candidates: &[String]) -> String {
    if candidates.is_empty() {
        return String::new();
    }
    let first = &candidates[0];
    let mut len = first.len();
    for c in &candidates[1..] {
        len = first
            .chars()
            .zip(c.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(len);
    }
    first[..first
        .char_indices()
        .nth(len)
        .map(|(i, _)| i)
        .unwrap_or(first.len())]
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::completion_engine::detect_source;
    use crate::CompletionSource;

    #[test]
    fn detect_cmd() {
        assert_eq!(detect_source(""), CompletionSource::Command);
        assert_eq!(detect_source("e foo"), CompletionSource::Path);
        assert_eq!(detect_source("set tab"), CompletionSource::Option);
    }

    #[test]
    fn complete_cmds() {
        let cmds = complete_commands("qu");
        assert!(cmds.contains(&"quit".to_string()));
    }

    #[test]
    fn common_prefix_test() {
        let v = vec!["foobar".into(), "foobaz".into()];
        assert_eq!(common_prefix(&v), "fooba");
    }
}
