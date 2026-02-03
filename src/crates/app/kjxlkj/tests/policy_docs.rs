use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn docs_policy_invariants_hold() {
    let root = repo_root();
    let docs = root.join("docs");
    assert!(docs.is_dir(), "missing docs/ directory");

    assert_all_files_under_line_limit(&docs, 200, &["md"]);
    assert_all_files_under_line_limit(&root.join("src"), 200, &["rs"]);
    assert_docs_no_non_mermaid_fences(&docs);
    assert_docs_no_parent_links(&docs);
    assert_docs_each_dir_has_one_readme(&docs);
    assert_docs_max_children_per_dir(&docs, 12);
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repo root not found")
        .to_path_buf()
}

fn assert_all_files_under_line_limit(root: &Path, limit: usize, exts: &[&str]) {
    walk(root, &mut |p| {
        if !p.is_file() {
            return;
        }
        let Some(ext) = p.extension().and_then(|e| e.to_str()) else {
            return;
        };
        if !exts.contains(&ext) {
            return;
        }
        let contents = fs::read_to_string(p).unwrap();
        let lines = contents.lines().count();
        assert!(lines <= limit, "file exceeds {limit} lines: {} ({lines})", p.display());
    });
}

fn assert_docs_no_non_mermaid_fences(docs: &Path) {
    walk(docs, &mut |p| {
        if p.extension().and_then(|e| e.to_str()) != Some("md") {
            return;
        }
        let contents = fs::read_to_string(p).unwrap();
        let mut in_mermaid = false;
        for (idx, line) in contents.lines().enumerate() {
            let t = line.trim_start();
            if !t.starts_with("```") {
                continue;
            }
            if t == "```mermaid" {
                assert!(!in_mermaid, "nested mermaid fence: {}:{}", p.display(), idx + 1);
                in_mermaid = true;
                continue;
            }
            if t == "```" && in_mermaid {
                in_mermaid = false;
                continue;
            }
            panic!("non-mermaid fence in docs: {}:{}", p.display(), idx + 1);
        }
        assert!(!in_mermaid, "unclosed mermaid fence: {}", p.display());
    });
}

fn assert_docs_no_parent_links(docs: &Path) {
    walk(docs, &mut |p| {
        if p.extension().and_then(|e| e.to_str()) != Some("md") {
            return;
        }
        let contents = fs::read_to_string(p).unwrap();
        assert!(
            !contents.contains("](../"),
            "parent link (../) found in {}",
            p.display()
        );
    });
}

fn assert_docs_each_dir_has_one_readme(docs: &Path) {
    walk(docs, &mut |p| {
        if !p.is_dir() {
            return;
        }
        let mut count = 0usize;
        for entry in fs::read_dir(p).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == "README.md" {
                count += 1;
            }
        }
        assert_eq!(count, 1, "docs dir must contain exactly one README.md: {}", p.display());
    });
}

fn assert_docs_max_children_per_dir(docs: &Path, max: usize) {
    walk(docs, &mut |p| {
        if !p.is_dir() {
            return;
        }
        let children = fs::read_dir(p).unwrap().count();
        assert!(
            children <= max,
            "docs dir exceeds {max} direct children: {} ({children})",
            p.display()
        );
    });
}

fn walk(root: &Path, f: &mut dyn FnMut(&Path)) {
    f(root);
    if !root.is_dir() {
        return;
    }
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        walk(&entry.path(), f);
    }
}
