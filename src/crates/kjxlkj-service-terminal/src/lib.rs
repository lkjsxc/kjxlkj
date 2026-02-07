//! kjxlkj-service-terminal: Terminal emulator grid and pane management.

pub mod terminal_full;
pub mod terminal_grid;

pub use terminal_full::{PaneManager, TerminalPane, TmuxAction, TmuxState, map_tmux_key, scrollback_capacity};
pub use terminal_grid::{AnsiAction, Cell, Style, TerminalGrid, parse_ansi_simple};
