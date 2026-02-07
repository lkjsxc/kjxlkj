//! Integration tests for input parsing, keybinding tables, and mappings.

use kjxlkj_input::*;
use kjxlkj_core_types::{KeyCode, Mode};

#[test]
fn parse_key_ctrl_a() {
    let k = parse_key_notation("<C-a>").unwrap();
    assert!(k.ctrl);
    assert_eq!(k.key, "a");
}

#[test]
fn parse_key_alt() {
    let k = parse_key_notation("<A-x>").unwrap();
    assert!(k.alt);
    assert_eq!(k.key, "x");
}

#[test]
fn parse_key_leader() {
    let k = parse_key_notation("<Leader>").unwrap();
    assert_eq!(k.key, "Leader");
}

#[test]
fn parse_key_sequence_multi() {
    let seq = parse_key_sequence("<C-w>jk");
    assert_eq!(seq.len(), 3);
    assert!(seq[0].ctrl);
    assert_eq!(seq[1].key, "j");
    assert_eq!(seq[2].key, "k");
}

#[test]
fn parse_key_cr_enter() {
    let k = parse_key_notation("<CR>").unwrap();
    assert_eq!(k.key, "Enter");
}

#[test]
fn binding_table_normal_count() {
    let t = build_normal_table();
    assert!(t.entries.len() >= 60, "got {}", t.entries.len());
}

#[test]
fn leader_registry_default() {
    let bindings = default_leader_bindings();
    assert_eq!(bindings.len(), 17);
    let reg = LeaderRegistry { bindings };
    assert_eq!(reg.resolve("f").unwrap().action, "find_file");
}

#[test]
fn mapping_store_add_lookup() {
    let mut store = MappingStore::default();
    store.add(MappingEntry {
        mode: MapMode::Normal,
        from: "jj".into(),
        to: "<Esc>".into(),
        noremap: false,
        buffer_local: false,
    });
    let m = store.lookup(MapMode::Normal, "jj").unwrap();
    assert_eq!(m.to, "<Esc>");
}

#[test]
fn mapping_noremap() {
    let mut store = MappingStore::default();
    store.add(MappingEntry {
        mode: MapMode::Normal,
        from: "a".into(),
        to: "b".into(),
        noremap: true,
        buffer_local: false,
    });
    let result = expand_recursive(&store, MapMode::Normal, "a", 10).unwrap();
    assert_eq!(result, "b");
}

#[test]
fn mapping_recursive_limit() {
    let mut store = MappingStore::default();
    store.add(MappingEntry {
        mode: MapMode::Normal,
        from: "a".into(),
        to: "b".into(),
        noremap: false,
        buffer_local: false,
    });
    store.add(MappingEntry {
        mode: MapMode::Normal,
        from: "b".into(),
        to: "a".into(),
        noremap: false,
        buffer_local: false,
    });
    assert!(expand_recursive(&store, MapMode::Normal, "a", 10).is_err());
}

#[test]
fn layout_no_overlap() {
    let r = vec![
        LayoutRegion { x: 0, y: 0, w: 80, h: 22, name: "editor".into() },
        LayoutRegion { x: 0, y: 22, w: 80, h: 1, name: "status".into() },
        LayoutRegion { x: 0, y: 23, w: 80, h: 1, name: "cmdline".into() },
    ];
    assert!(check_no_overlap(&r).is_ok());
}

#[test]
fn layout_coverage() {
    let r = vec![
        LayoutRegion { x: 0, y: 0, w: 80, h: 22, name: "editor".into() },
        LayoutRegion { x: 0, y: 22, w: 80, h: 1, name: "status".into() },
        LayoutRegion { x: 0, y: 23, w: 80, h: 1, name: "cmdline".into() },
    ];
    assert!(check_coverage(&r, 80, 24));
}

#[test]
fn headless_script_parse() {
    let json = r#"[{"Key":{"code":"i","ctrl":false,"alt":false,"shift":false}}]"#;
    let steps = parse_script(json).unwrap();
    assert_eq!(steps.len(), 1);
}

#[test]
fn ux_coverage_normal() {
    let entries = build_normal_coverage();
    let summary = compute_summary(&entries);
    assert!(summary.total >= 30);
    assert!(summary.tested > 0);
}

#[test]
fn keybinding_dsl_special() {
    assert_eq!(resolve_special("Space"), Some("Space".into()));
    assert_eq!(resolve_special("Esc"), Some("Escape".into()));
    assert_eq!(resolve_special("BS"), Some("Backspace".into()));
    assert_eq!(resolve_special("Tab"), Some("Tab".into()));
    assert_eq!(resolve_special("F1"), Some("F1".into()));
}
