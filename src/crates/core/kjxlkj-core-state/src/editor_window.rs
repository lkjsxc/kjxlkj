use crate::editor::EditorState;
use crate::windows::{Axis, Direction, WindowKind};

impl EditorState {
    pub(crate) fn apply_window_command(&mut self, ch: char) -> String {
        match ch {
            's' => {
                self.windows
                    .split_focused(Axis::Horizontal, WindowKind::Buffer);
                "WinSplitHorizontal".to_string()
            }
            'v' => {
                self.windows
                    .split_focused(Axis::Vertical, WindowKind::Buffer);
                "WinSplitVertical".to_string()
            }
            'E' => {
                self.windows
                    .split_focused(Axis::Vertical, WindowKind::Explorer);
                "WinSplitExplorer".to_string()
            }
            'T' => {
                self.windows
                    .split_focused(Axis::Vertical, WindowKind::Terminal);
                "WinSplitTerminal".to_string()
            }
            'H' => {
                self.windows
                    .split_focused(Axis::Horizontal, WindowKind::Terminal);
                "WinSplitTerminalHorizontal".to_string()
            }
            'c' | 'q' => {
                self.windows.close_focused();
                "WinClose".to_string()
            }
            'o' => {
                self.windows.only();
                "WinOnly".to_string()
            }
            'h' => {
                self.windows
                    .focus_direction(Direction::Left, self.window_area);
                "WinFocusLeft".to_string()
            }
            'j' => {
                self.windows
                    .focus_direction(Direction::Down, self.window_area);
                "WinFocusDown".to_string()
            }
            'k' => {
                self.windows
                    .focus_direction(Direction::Up, self.window_area);
                "WinFocusUp".to_string()
            }
            'l' => {
                self.windows
                    .focus_direction(Direction::Right, self.window_area);
                "WinFocusRight".to_string()
            }
            'w' => {
                self.windows.cycle_next();
                "WinCycleNext".to_string()
            }
            'W' => {
                self.windows.cycle_prev();
                "WinCyclePrev".to_string()
            }
            'p' => {
                self.windows.focus_previous();
                "WinPrevious".to_string()
            }
            't' => {
                self.windows.focus_top_left(self.window_area);
                "WinTopLeft".to_string()
            }
            'b' => {
                self.windows.focus_bottom_right(self.window_area);
                "WinBottomRight".to_string()
            }
            _ => "WinIgnore".to_string(),
        }
    }
}
