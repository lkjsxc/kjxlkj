//! kjxlkj-service-terminal: Terminal emulator grid and pane management.

pub mod terminal_full;
pub mod terminal_grid;

pub use terminal_full::{
    map_tmux_key, scrollback_capacity, PaneManager, TerminalPane, TmuxAction, TmuxState,
};
pub use terminal_grid::{parse_ansi_simple, AnsiAction, Cell, Style, TerminalGrid};
