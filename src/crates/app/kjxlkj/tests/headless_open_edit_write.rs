use kjxlkj_core::{Key, KeyCode, KeyMods};

#[derive(serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Step {
    Key { key: Key },
    WaitStatus { contains: String, timeout_ms: u64 },
}

#[test]
fn headless_can_open_edit_and_write() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("file.txt");
    let script_path = dir.path().join("script.json");

    std::fs::write(&file_path, "abc").unwrap();

    let mut steps = Vec::<Step>::new();
    steps.push(step_key(KeyCode::Char(':')));
    for c in format!("e {}", file_path.display()).chars() {
        steps.push(step_key(KeyCode::Char(c)));
    }
    steps.push(step_key(KeyCode::Enter));
    steps.push(Step::WaitStatus {
        contains: "opened".to_string(),
        timeout_ms: 2_000,
    });
    steps.push(step_key(KeyCode::Char('i')));
    steps.push(step_key(KeyCode::Char('Z')));
    steps.push(step_key(KeyCode::Esc));
    steps.push(step_key(KeyCode::Char(':')));
    steps.push(step_key(KeyCode::Char('w')));
    steps.push(step_key(KeyCode::Char('q')));
    steps.push(step_key(KeyCode::Enter));

    std::fs::write(&script_path, serde_json::to_string(&steps).unwrap()).unwrap();

    let mut cmd = assert_cmd::cargo_bin_cmd!("kjxlkj");
    cmd.args(["--headless", "--script"])
        .arg(&script_path)
        .assert()
        .success();

    let out = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(out, "Zabc");
}

fn step_key(code: KeyCode) -> Step {
    Step::Key {
        key: Key {
            code,
            mods: KeyMods::default(),
        },
    }
}

