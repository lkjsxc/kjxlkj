use kjxlkj_service_terminal::{
    parse_ansi_simple, AnsiAction, PaneManager, Style, TerminalGrid,
};

// --- Terminal grid ---

#[test]
fn grid_creation_dimensions() {
    let g = TerminalGrid::new(80, 24);
    assert_eq!(g.width, 80);
    assert_eq!(g.height, 24);
    assert_eq!(g.cells.len(), 24);
    assert_eq!(g.cells[0].len(), 80);
}

#[test]
fn grid_put_char_writes() {
    let mut g = TerminalGrid::new(10, 5);
    g.put_char('A');
    assert_eq!(g.cells[0][0].ch, 'A');
    assert_eq!(g.cursor_x, 1);
}

#[test]
fn grid_put_char_advances_cursor() {
    let mut g = TerminalGrid::new(10, 5);
    g.put_char('H');
    g.put_char('i');
    assert_eq!(g.cells[0][0].ch, 'H');
    assert_eq!(g.cells[0][1].ch, 'i');
    assert_eq!(g.cursor_x, 2);
}

#[test]
fn grid_clear_resets_all() {
    let mut g = TerminalGrid::new(5, 3);
    g.put_char('X');
    g.clear();
    assert_eq!(g.cells[0][0].ch, ' ');
    assert_eq!(g.cursor_x, 0);
    assert_eq!(g.cursor_y, 0);
}

#[test]
fn grid_scroll_up() {
    let mut g = TerminalGrid::new(5, 3);
    g.cells[0][0].ch = 'A';
    g.cells[1][0].ch = 'B';
    g.cells[2][0].ch = 'C';
    g.scroll_up();
    assert_eq!(g.cells[0][0].ch, 'B');
    assert_eq!(g.cells[1][0].ch, 'C');
    assert_eq!(g.cells[2][0].ch, ' ');
}

#[test]
fn grid_clear_to_eol() {
    let mut g = TerminalGrid::new(5, 2);
    for i in 0..5 {
        g.cells[0][i].ch = 'X';
    }
    g.cursor_x = 2;
    g.cursor_y = 0;
    g.clear_to_eol();
    assert_eq!(g.cells[0][0].ch, 'X');
    assert_eq!(g.cells[0][1].ch, 'X');
    assert_eq!(g.cells[0][2].ch, ' ');
}

// --- ANSI parsing ---

#[test]
fn ansi_print_chars() {
    let actions = parse_ansi_simple("AB");
    assert_eq!(actions, vec![AnsiAction::Print('A'), AnsiAction::Print('B')]);
}

#[test]
fn ansi_cursor_move() {
    let actions = parse_ansi_simple("\x1b[5;10H");
    assert_eq!(actions, vec![AnsiAction::CursorMove(9, 4)]);
}

#[test]
fn ansi_sgr_bold() {
    let actions = parse_ansi_simple("\x1b[1m");
    match &actions[0] {
        AnsiAction::SetStyle(s) => assert!(s.bold),
        other => panic!("expected SetStyle, got {:?}", other),
    }
}

#[test]
fn ansi_clear_screen() {
    let actions = parse_ansi_simple("\x1b[2J");
    assert_eq!(actions, vec![AnsiAction::ClearScreen]);
}

#[test]
fn ansi_clear_to_eol() {
    let actions = parse_ansi_simple("\x1b[K");
    assert_eq!(actions, vec![AnsiAction::ClearToEol]);
}

// --- Pane management ---

#[test]
fn pane_create() {
    let mut pm = PaneManager::new();
    let id = pm.create_pane("shell", 80, 24);
    assert!(pm.get(id).is_some());
    assert_eq!(pm.get(id).unwrap().title, "shell");
}

#[test]
fn pane_close() {
    let mut pm = PaneManager::new();
    let id = pm.create_pane("shell", 80, 24);
    pm.close_pane(id);
    assert!(pm.get(id).is_none());
}

#[test]
fn pane_list_sorted() {
    let mut pm = PaneManager::new();
    let id1 = pm.create_pane("a", 80, 24);
    let id2 = pm.create_pane("b", 80, 24);
    let list = pm.list();
    assert_eq!(list, vec![id1, id2]);
}

#[test]
fn pane_set_active() {
    let mut pm = PaneManager::new();
    let _id1 = pm.create_pane("a", 80, 24);
    let id2 = pm.create_pane("b", 80, 24);
    pm.set_active(id2);
    pm.close_pane(id2);
    assert!(pm.get(id2).is_none());
}
