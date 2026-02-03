use kjxlkj_core::{Key, KeyCode, KeyMods};

#[test]
fn headless_can_insert_write_and_quit() {
    let dir = tempfile::tempdir().unwrap();
    let out_path = dir.path().join("out.txt");
    let script_path = dir.path().join("script.json");

    let mut keys = Vec::<Key>::new();
    keys.push(k(KeyCode::Char('i')));
    for c in "hello".chars() {
        keys.push(k(KeyCode::Char(c)));
    }
    keys.push(k(KeyCode::Esc));
    keys.push(k(KeyCode::Char(':')));
    for c in format!("wq {}", out_path.display()).chars() {
        keys.push(k(KeyCode::Char(c)));
    }
    keys.push(k(KeyCode::Enter));

    std::fs::write(&script_path, serde_json::to_string(&keys).unwrap()).unwrap();

    let mut cmd = assert_cmd::cargo_bin_cmd!("kjxlkj");
    cmd.args(["--headless", "--script"])
        .arg(&script_path)
        .assert()
        .success();

    let out = std::fs::read_to_string(out_path).unwrap();
    assert_eq!(out, "hello");
}

fn k(code: KeyCode) -> Key {
    Key {
        code,
        mods: KeyMods::default(),
    }
}
