/// Command-line window interactions with viewports.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CmdlineWindowState {
    pub(crate) history: Vec<String>,
    pub(crate) cursor_line: usize,
    pub(crate) cursor_col: usize,
    pub(crate) prompt: char,
    pub(crate) active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CmdlineViewport {
    pub(crate) top_line: usize,
    pub(crate) visible_lines: usize,
    pub(crate) width: usize,
}

impl CmdlineWindowState {
    pub(crate) fn new(prompt: char) -> Self {
        Self {
            history: Vec::new(),
            cursor_line: 0,
            cursor_col: 0,
            prompt,
            active: true,
        }
    }

    pub(crate) fn open(history: Vec<String>) -> Self {
        let cursor_line = if history.is_empty() { 0 } else { history.len() - 1 };
        Self {
            history,
            cursor_line,
            cursor_col: 0,
            prompt: ':',
            active: true,
        }
    }

    pub(crate) fn close(&mut self) -> Option<String> {
        self.active = false;
        self.history.get(self.cursor_line).cloned()
    }

    pub(crate) fn current_line(&self) -> &str {
        self.history.get(self.cursor_line).map(|s| s.as_str()).unwrap_or("")
    }

    pub(crate) fn move_cursor(&mut self, delta: isize) {
        if self.history.is_empty() {
            return;
        }
        let new_pos = self.cursor_line as isize + delta;
        let max = self.history.len().saturating_sub(1) as isize;
        self.cursor_line = new_pos.clamp(0, max) as usize;
    }

    pub(crate) fn edit_line(&mut self, text: &str) {
        if self.cursor_line < self.history.len() {
            self.history[self.cursor_line] = text.to_string();
        } else {
            self.history.push(text.to_string());
            self.cursor_line = self.history.len() - 1;
        }
        self.cursor_col = text.len();
    }
}

pub(crate) fn follow_cmdline_cursor(state: &CmdlineWindowState, viewport: &mut CmdlineViewport) {
    if state.cursor_line < viewport.top_line {
        viewport.top_line = state.cursor_line;
    } else if state.cursor_line >= viewport.top_line + viewport.visible_lines {
        viewport.top_line = state.cursor_line.saturating_sub(viewport.visible_lines - 1);
    }
}

pub(crate) fn render_cmdline_window(
    state: &CmdlineWindowState,
    viewport: &CmdlineViewport,
) -> Vec<String> {
    let end = (viewport.top_line + viewport.visible_lines).min(state.history.len());
    state.history[viewport.top_line..end]
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let prefix = if viewport.top_line + i == state.cursor_line { ">" } else { " " };
            let display = if line.len() > viewport.width {
                &line[..viewport.width]
            } else {
                line
            };
            format!("{}{}{}", state.prompt, prefix, display)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_history() -> Vec<String> {
        vec!["w".into(), "q".into(), "e file.rs".into(), "set nu".into()]
    }

    #[test]
    fn open_with_history() {
        let state = CmdlineWindowState::open(sample_history());
        assert_eq!(state.cursor_line, 3);
        assert!(state.active);
        assert_eq!(state.history.len(), 4);
    }

    #[test]
    fn close_returns_current() {
        let mut state = CmdlineWindowState::open(sample_history());
        state.cursor_line = 1;
        let result = state.close();
        assert_eq!(result, Some("q".into()));
        assert!(!state.active);
    }

    #[test]
    fn move_cursor_clamps() {
        let mut state = CmdlineWindowState::open(sample_history());
        state.move_cursor(-100);
        assert_eq!(state.cursor_line, 0);
        state.move_cursor(100);
        assert_eq!(state.cursor_line, 3);
    }

    #[test]
    fn edit_line() {
        let mut state = CmdlineWindowState::open(sample_history());
        state.cursor_line = 0;
        state.edit_line("wq");
        assert_eq!(state.current_line(), "wq");
        assert_eq!(state.cursor_col, 2);
    }

    #[test]
    fn follow_cursor() {
        let state = CmdlineWindowState::open(sample_history());
        let mut vp = CmdlineViewport { top_line: 0, visible_lines: 2, width: 40 };
        follow_cmdline_cursor(&state, &mut vp);
        assert_eq!(vp.top_line, 2); // scrolled to show cursor_line=3
    }

    #[test]
    fn render_basic() {
        let state = CmdlineWindowState::open(sample_history());
        let vp = CmdlineViewport { top_line: 0, visible_lines: 4, width: 40 };
        let lines = render_cmdline_window(&state, &vp);
        assert_eq!(lines.len(), 4);
        assert!(lines[3].contains(">"));
        assert!(lines[0].starts_with(':'));
    }

    #[test]
    fn empty_history() {
        let state = CmdlineWindowState::open(vec![]);
        assert_eq!(state.cursor_line, 0);
        assert_eq!(state.current_line(), "");
        let vp = CmdlineViewport { top_line: 0, visible_lines: 5, width: 40 };
        let lines = render_cmdline_window(&state, &vp);
        assert!(lines.is_empty());
    }
}
