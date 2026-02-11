use super::*;
fn feed_str(p: &mut Parser, s: &mut Screen, input: &str) {
    for b in input.bytes() { p.feed(b, s); }
}
#[test]
fn print_ascii() {
    let mut s = Screen::new(10, 5);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "Hi");
    assert_eq!(s.cell(0, 0).ch, 'H');
    assert_eq!(s.cell(0, 1).ch, 'i');
    assert_eq!(s.cursor_col, 2);
}
#[test]
fn csi_cursor_move() {
    let mut s = Screen::new(20, 10);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "\x1b[5;3H");
    assert_eq!(s.cursor_row, 4);
    assert_eq!(s.cursor_col, 2);
}
#[test]
fn csi_erase_line() {
    let mut s = Screen::new(10, 5);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "hello\x1b[1G\x1b[K");
    assert_eq!(s.cell(0, 0).ch, ' ');
}
#[test]
fn sgr_bold() {
    let mut s = Screen::new(10, 5);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "\x1b[1mX");
    assert!(s.cell(0, 0).bold);
    assert_eq!(s.cell(0, 0).ch, 'X');
}
#[test]
fn osc_title() {
    let mut s = Screen::new(10, 5);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "\x1b]0;hello\x07");
    assert_eq!(s.title, "hello");
}
#[test]
fn linefeed_and_cr() {
    let mut s = Screen::new(10, 5);
    let mut p = Parser::new();
    feed_str(&mut p, &mut s, "ab\r\ncd");
    assert_eq!(s.cell(0, 0).ch, 'a');
    assert_eq!(s.cell(1, 0).ch, 'c');
}
