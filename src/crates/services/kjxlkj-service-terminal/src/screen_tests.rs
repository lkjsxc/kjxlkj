use super::*;

#[test]
fn blank_screen() {
    let s = Screen::new(10, 5);
    assert_eq!(s.cell(0, 0).ch, ' ');
    assert_eq!(s.cursor_row, 0);
}

#[test]
fn put_char_advances() {
    let mut s = Screen::new(10, 5);
    s.put_char('A');
    assert_eq!(s.cell(0, 0).ch, 'A');
    assert_eq!(s.cursor_col, 1);
}

#[test]
fn erase_display_all() {
    let mut s = Screen::new(5, 3);
    s.put_char('X'); s.put_char('Y');
    s.erase_display(2);
    assert_eq!(s.cell(0, 0).ch, ' ');
    assert_eq!(s.cell(0, 1).ch, ' ');
}

#[test]
fn scroll_up_shifts() {
    let mut s = Screen::new(3, 3);
    s.put_char('A'); s.linefeed(); s.carriage_return();
    s.put_char('B'); s.linefeed(); s.carriage_return();
    s.put_char('C'); s.linefeed();
    assert_eq!(s.cell(0, 0).ch, 'B');
    assert_eq!(s.cell(1, 0).ch, 'C');
}

#[test]
fn save_restore_cursor() {
    let mut s = Screen::new(10, 5);
    s.cursor_row = 3; s.cursor_col = 7;
    s.save_cursor();
    s.cursor_row = 0; s.cursor_col = 0;
    s.restore_cursor();
    assert_eq!(s.cursor_row, 3);
    assert_eq!(s.cursor_col, 7);
}
