use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::topology::{scan_docs_topology, TopologyRule};
use super::CommandResult;

#[test]
fn scan_docs_topology_passes_when_contract_is_satisfied() {
    let root = create_temp_dir("topology-pass");
    let docs = root.join("docs");

    fs::create_dir_all(docs.join("guide")).unwrap();
    fs::write(docs.join("README.md"), "# docs").unwrap();
    fs::write(docs.join("guide/README.md"), "# guide").unwrap();
    fs::write(docs.join("guide/topic.md"), "topic").unwrap();

    let report = scan_docs_topology(&docs).unwrap();

    assert_eq!(report.directories_checked, 2);
    assert!(report.violations.is_empty());
    assert_eq!(report.result(), CommandResult::Pass);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scan_docs_topology_reports_missing_readme_and_child_count_failures() {
    let root = create_temp_dir("topology-fail");
    let docs = root.join("docs");

    fs::create_dir_all(docs.join("section")).unwrap();
    fs::write(docs.join("README.md"), "# docs").unwrap();
    fs::write(docs.join("section/article.md"), "article").unwrap();

    let report = scan_docs_topology(&docs).unwrap();

    assert_eq!(report.result(), CommandResult::Fail);
    assert_eq!(report.violations.len(), 2);
    assert!(report.violations.iter().any(|violation| {
        violation.path.ends_with("section")
            && violation.rule == TopologyRule::ExactlyOneReadme
            && violation.expected == 1
            && violation.actual == 0
    }));
    assert!(report.violations.iter().any(|violation| {
        violation.path.ends_with("section")
            && violation.rule == TopologyRule::MultipleChildren
            && violation.expected == 2
            && violation.actual == 1
    }));

    fs::remove_dir_all(root).unwrap();
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
