//! Terminal service.
//!
//! This crate provides PTY-backed terminal emulation.

mod parser;
mod pty;
mod screen;
mod service;

pub use parser::*;
pub use pty::*;
pub use screen::*;
pub use service::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_creation() {
        let screen = Screen::new(80, 24);
        let (width, height) = screen.dimensions();
        assert_eq!(width, 80);
        assert_eq!(height, 24);
    }

    #[test]
    fn test_screen_print() {
        let mut screen = Screen::new(80, 24);
        screen.print('H');
        screen.print('i');

        assert_eq!(screen.cell(0, 0).map(|c| c.ch), Some('H'));
        assert_eq!(screen.cell(1, 0).map(|c| c.ch), Some('i'));
        assert_eq!(screen.cursor(), (2, 0));
    }

    #[test]
    fn test_screen_newline() {
        let mut screen = Screen::new(80, 24);
        screen.print('A');
        screen.newline();
        screen.print('B');

        assert_eq!(screen.cell(0, 0).map(|c| c.ch), Some('A'));
        assert_eq!(screen.cell(0, 1).map(|c| c.ch), Some('B'));
    }

    #[test]
    fn test_screen_resize() {
        let mut screen = Screen::new(80, 24);
        screen.print('X');
        screen.resize(40, 12);

        let (width, height) = screen.dimensions();
        assert_eq!(width, 40);
        assert_eq!(height, 12);
    }

    #[test]
    fn test_screen_scroll() {
        let mut screen = Screen::new(80, 3);

        // Fill screen and trigger scroll.
        for i in 0..4 {
            screen.print((b'A' + i) as char);
            screen.newline();
        }

        // After scrolling, cursor should be at bottom.
        let (_, y) = screen.cursor();
        assert_eq!(y, 2);
    }

    #[test]
    fn test_parser_basic() {
        let mut parser = Parser::new();

        let actions = parser.parse(b'A');
        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], ParseAction::Print('A')));
    }

    #[test]
    fn test_parser_newline() {
        let mut parser = Parser::new();

        let actions = parser.parse(b'\n');
        assert!(actions.iter().any(|a| matches!(a, ParseAction::Newline)));
    }
}
