use kjxlkj_core_state::{MappingMode, MappingTable};

#[test]
fn add_and_lookup() {
    let mut t = MappingTable::new();
    t.add(MappingMode::Normal, "jk", "<Esc>", false);
    let m = t.get(MappingMode::Normal, "jk").unwrap();
    assert_eq!(m.rhs, "<Esc>");
    assert!(!m.recursive);
}

#[test]
fn remove_mapping() {
    let mut t = MappingTable::new();
    t.add(MappingMode::Normal, "jk", "<Esc>", false);
    assert!(t.remove(MappingMode::Normal, "jk"));
    assert!(t.get(MappingMode::Normal, "jk").is_none());
}

#[test]
fn all_mode_fallback() {
    let mut t = MappingTable::new();
    t.add(MappingMode::All, "jj", "escape", true);
    assert!(t.get(MappingMode::Normal, "jj").is_some());
    assert!(t.get(MappingMode::Insert, "jj").is_some());
}

#[test]
fn override_existing() {
    let mut t = MappingTable::new();
    t.add(MappingMode::Normal, "x", "old", false);
    t.add(MappingMode::Normal, "x", "new", false);
    assert_eq!(t.get(MappingMode::Normal, "x").unwrap().rhs, "new");
}

#[test]
fn list_mappings() {
    let mut t = MappingTable::new();
    t.add(MappingMode::Normal, "a", "b", false);
    t.add(MappingMode::Normal, "c", "d", true);
    assert_eq!(t.list(MappingMode::Normal).len(), 2);
}

#[test]
fn display_all() {
    let mut t = MappingTable::new();
    t.add(MappingMode::Normal, "<leader>e", ":explorer", false);
    let s = t.display_all();
    assert!(s.contains("nnoremap"));
    assert!(s.contains("<leader>e"));
}
