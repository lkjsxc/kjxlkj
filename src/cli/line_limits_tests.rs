use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::line_limits::{scan_line_limits, LineScope};
use super::CommandResult;

#[test]
fn scan_line_limits_reports_docs_and_test_source_violations() {
    let root = create_temp_dir("line-limits-fail");
    let docs = root.join("docs");
    let tests = root.join("src/tests");

    fs::create_dir_all(&docs).unwrap();
    fs::create_dir_all(&tests).unwrap();

    fs::write(docs.join("README.md"), "# docs\n").unwrap();
    fs::write(docs.join("long.md"), generate_lines(301)).unwrap();
    fs::write(tests.join("case.rs"), generate_lines(201)).unwrap();
    fs::write(tests.join("note.txt"), generate_lines(1000)).unwrap();

    let report = scan_line_limits(&docs, &tests).unwrap();

    assert_eq!(report.docs_files_checked, 2);
    assert_eq!(report.test_source_files_checked, 1);
    assert_eq!(report.result(), CommandResult::Fail);
    assert_eq!(report.violations.len(), 2);

    assert!(report.violations.iter().any(|violation| {
        violation.scope == LineScope::DocsMarkdown
            && violation.path.ends_with("long.md")
            && violation.line_count == 301
            && violation.limit == 300
    }));
    assert!(report.violations.iter().any(|violation| {
        violation.scope == LineScope::TestSource
            && violation.path.ends_with("case.rs")
            && violation.line_count == 201
            && violation.limit == 200
    }));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scan_line_limits_passes_when_src_tests_directory_is_missing() {
    let root = create_temp_dir("line-limits-pass");
    let docs = root.join("docs");
    let tests = root.join("src/tests");

    fs::create_dir_all(&docs).unwrap();
    fs::write(docs.join("README.md"), "# docs\n").unwrap();

    let report = scan_line_limits(&docs, &tests).unwrap();

    assert_eq!(report.docs_files_checked, 1);
    assert_eq!(report.test_source_files_checked, 0);
    assert_eq!(report.result(), CommandResult::Pass);

    fs::remove_dir_all(root).unwrap();
}

fn generate_lines(count: usize) -> String {
    let mut output = String::new();
    for _ in 0..count {
        output.push_str("line\n");
    }
    output
}

fn create_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("kjxlkj-{prefix}-{nanos}"));
    fs::create_dir_all(&path).unwrap();
    path
}
