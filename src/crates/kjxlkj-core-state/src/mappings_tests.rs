use crate::mappings::{KeyMapping, MapMode, MappingLookup, MappingTable};
use kjxlkj_core_types::Key;

#[test]
fn test_add_and_lookup() {
    let mut table = MappingTable::new();
    table.add(
        MapMode::Normal,
        KeyMapping {
            from: vec![Key::char('j'), Key::char('k')],
            to: vec![Key::esc()],
            recursive: false,
            description: None,
        },
    );

    match table.lookup(MapMode::Normal, &[Key::char('j'), Key::char('k')]) {
        MappingLookup::Exact(m) => {
            assert_eq!(m.to, vec![Key::esc()]);
        }
        other => panic!("Expected Exact, got {:?}", other),
    }
}

#[test]
fn test_prefix_lookup() {
    let mut table = MappingTable::new();
    table.add(
        MapMode::Normal,
        KeyMapping {
            from: vec![Key::char('g'), Key::char('d')],
            to: vec![],
            recursive: false,
            description: None,
        },
    );

    match table.lookup(MapMode::Normal, &[Key::char('g')]) {
        MappingLookup::Prefix => {}
        other => panic!("Expected Prefix, got {:?}", other),
    }
}

#[test]
fn test_no_match() {
    let table = MappingTable::new();
    match table.lookup(MapMode::Normal, &[Key::char('x')]) {
        MappingLookup::NoMatch => {}
        other => panic!("Expected NoMatch, got {:?}", other),
    }
}

#[test]
fn test_remove() {
    let mut table = MappingTable::new();
    table.add(
        MapMode::Normal,
        KeyMapping {
            from: vec![Key::char('x')],
            to: vec![Key::char('d'), Key::char('l')],
            recursive: false,
            description: None,
        },
    );
    table.remove(MapMode::Normal, &[Key::char('x')]);
    match table.lookup(MapMode::Normal, &[Key::char('x')]) {
        MappingLookup::NoMatch => {}
        other => panic!("Expected NoMatch after remove, got {:?}", other),
    }
}

#[test]
fn test_mode_conversion() {
    use kjxlkj_core_types::Mode;
    assert_eq!(MapMode::from_mode(&Mode::Normal), MapMode::Normal);
    assert_eq!(MapMode::from_mode(&Mode::Insert), MapMode::Insert);
    assert_eq!(
        MapMode::from_mode(&Mode::Visual(kjxlkj_core_types::VisualKind::Char)),
        MapMode::Visual
    );
}
