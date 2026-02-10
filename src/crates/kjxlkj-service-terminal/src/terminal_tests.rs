//! Terminal service tests per spec testing-unit.md.
//!
//! ST-01 through ST-12.

#[cfg(test)]
mod tests {
    use crate::cell::Color;
    use crate::parser::Parser;
    use crate::screen::Screen;
    use crate::task::TerminalService;

    fn feed(data: &[u8]) -> Screen {
        let mut s = Screen::new(80, 24);
        let mut p = Parser::new();
        p.feed_bytes(data, &mut s);
        s
    }

    /// ST-01: CUP positioning. ESC[5;10H moves to row 4, col 9 (0-based).
    #[test]
    fn st01_cup_positioning() {
        let s = feed(b"\x1b[5;10H");
        assert_eq!(s.cursor_row, 4);
        assert_eq!(s.cursor_col, 9);
    }

    /// ST-02: SGR color. ESC[38;2;255;0;0m sets fg to RGB(255,0,0).
    #[test]
    fn st02_sgr_color() {
        let s = feed(b"\x1b[38;2;255;0;0m");
        assert_eq!(s.attrs.fg, Color::Rgb(255, 0, 0));
    }

    /// ST-03: ED erase display. ESC[2J clears all cells.
    #[test]
    fn st03_ed_erase_display() {
        let mut s = Screen::new(10, 5);
        let mut p = Parser::new();
        p.feed_bytes(b"hello", &mut s);
        assert_eq!(s.cells[0][0].ch, 'h');
        p.feed_bytes(b"\x1b[2J", &mut s);
        assert_eq!(s.cells[0][0].ch, ' ');
    }

    /// ST-04: Scroll region. ESC[5;15r sets scroll top=4 bottom=14.
    #[test]
    fn st04_scroll_region() {
        let s = feed(b"\x1b[5;15r");
        assert_eq!(s.scroll_top, 4);
        assert_eq!(s.scroll_bottom, 14);
    }

    /// ST-05: Alternate screen. ESC[?1049h clears screen.
    #[test]
    fn st05_alternate_screen() {
        let mut s = Screen::new(80, 24);
        let mut p = Parser::new();
        p.feed_bytes(b"hello", &mut s);
        p.feed_bytes(b"\x1b[?1049h", &mut s);
        assert_eq!(s.cursor_row, 0);
        assert_eq!(s.cursor_col, 0);
        assert_eq!(s.cells[0][0].ch, ' ');
    }

    /// ST-06: Wide char in terminal. Writing あ occupies 2 cells.
    #[test]
    fn st06_wide_char() {
        let s = feed("あ".as_bytes());
        assert_eq!(s.cells[0][0].ch, 'あ');
        assert_eq!(s.cells[0][0].width, 2);
        assert!(s.cells[0][1].is_wide_continuation);
    }

    /// ST-07: PTY spawn and read (service-level).
    #[test]
    fn st07_pty_spawn_and_read() {
        let mut svc = TerminalService::new();
        let id = svc.spawn(80, 24);
        svc.feed_output(id, b"hello");
        let inst = svc.terminals.get(&id).unwrap();
        assert_eq!(inst.screen.cells[0][0].ch, 'h');
        assert_eq!(inst.screen.cells[0][4].ch, 'o');
    }

    /// ST-08: PTY resize.
    #[test]
    fn st08_pty_resize() {
        let mut svc = TerminalService::new();
        let id = svc.spawn(80, 24);
        svc.terminals.get_mut(&id).unwrap().resize(40, 12);
        let inst = svc.terminals.get(&id).unwrap();
        assert_eq!(inst.screen.cols, 40);
        assert_eq!(inst.screen.rows, 12);
    }

    /// ST-09: PTY cleanup. Remove terminal.
    #[test]
    fn st09_pty_cleanup() {
        let mut svc = TerminalService::new();
        let id = svc.spawn(80, 24);
        svc.remove(id);
        assert!(!svc.terminals.contains_key(&id));
    }

    /// ST-10: Crash resilience. Terminal exit.
    #[test]
    fn st10_crash_resilience() {
        let mut svc = TerminalService::new();
        let id = svc.spawn(80, 24);
        svc.mark_exited(id);
        assert!(svc.terminals.get(&id).unwrap().exited);
    }

    /// ST-11: OSC title. ESC]2;My Title BEL.
    #[test]
    fn st11_osc_title() {
        let s = feed(b"\x1b]2;My Title\x07");
        assert_eq!(s.title, "My Title");
    }

    /// ST-12: 256-color support. ESC[38;5;196m.
    #[test]
    fn st12_256_color() {
        let s = feed(b"\x1b[38;5;196m");
        assert_eq!(s.attrs.fg, Color::Indexed(196));
    }

    /// SGR reset test.
    #[test]
    fn test_sgr_reset() {
        let s = feed(b"\x1b[1;3m\x1b[0m");
        assert!(!s.attrs.bold);
        assert!(!s.attrs.italic);
    }

    /// PE-03: alternate screen enter/exit preserves primary.
    #[test]
    fn pe03_alternate_screen_roundtrip() {
        let mut s = Screen::new(80, 24);
        s.write_char('A', 1);
        assert_eq!(s.cells[0][0].ch, 'A');
        // Enter alternate screen
        s.enter_alternate();
        assert!(s.in_alternate);
        assert_eq!(s.cells[0][0].ch, ' '); // blank
        s.write_char('B', 1);
        assert_eq!(s.cells[0][0].ch, 'B');
        // Exit alternate screen
        s.exit_alternate();
        assert!(!s.in_alternate);
        assert_eq!(s.cells[0][0].ch, 'A'); // primary restored
    }
}
