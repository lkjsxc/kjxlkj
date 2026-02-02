//! Floating window support.
//!
//! Provides floating windows for hover info, previews, etc.

/// Floating window anchor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatAnchor {
    /// Northwest corner.
    NW,
    /// Northeast corner.
    NE,
    /// Southwest corner.
    SW,
    /// Southeast corner.
    SE,
    /// Center.
    Center,
}

impl Default for FloatAnchor {
    fn default() -> Self {
        Self::NW
    }
}

/// Floating window border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatBorder {
    /// No border.
    None,
    /// Single line border.
    Single,
    /// Double line border.
    Double,
    /// Rounded border.
    Rounded,
    /// Shadow effect.
    Shadow,
}

impl Default for FloatBorder {
    fn default() -> Self {
        Self::None
    }
}

/// Floating window configuration.
#[derive(Debug, Clone)]
pub struct FloatConfig {
    /// Relative position type.
    pub relative: FloatRelative,
    /// Row offset.
    pub row: i32,
    /// Column offset.
    pub col: i32,
    /// Width (None = auto).
    pub width: Option<usize>,
    /// Height (None = auto).
    pub height: Option<usize>,
    /// Anchor position.
    pub anchor: FloatAnchor,
    /// Border style.
    pub border: FloatBorder,
    /// Z-index (higher = on top).
    pub zindex: usize,
    /// Whether focusable.
    pub focusable: bool,
}

/// What the float is positioned relative to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatRelative {
    /// Editor window.
    Editor,
    /// Current window.
    Window,
    /// Cursor position.
    Cursor,
}

impl Default for FloatConfig {
    fn default() -> Self {
        Self {
            relative: FloatRelative::Cursor,
            row: 0,
            col: 0,
            width: None,
            height: None,
            anchor: FloatAnchor::NW,
            border: FloatBorder::None,
            zindex: 50,
            focusable: false,
        }
    }
}

impl FloatConfig {
    /// Creates a new float config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the position.
    pub fn at(mut self, row: i32, col: i32) -> Self {
        self.row = row;
        self.col = col;
        self
    }

    /// Sets the size.
    pub fn size(mut self, width: usize, height: usize) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets the border.
    pub fn with_border(mut self, border: FloatBorder) -> Self {
        self.border = border;
        self
    }

    /// Sets focusable.
    pub fn focusable(mut self) -> Self {
        self.focusable = true;
        self
    }
}

/// A floating window.
#[derive(Debug, Clone)]
pub struct FloatWindow {
    /// Window ID.
    pub id: usize,
    /// Configuration.
    pub config: FloatConfig,
    /// Buffer ID.
    pub buffer_id: usize,
}

impl FloatWindow {
    /// Creates a new floating window.
    pub fn new(id: usize, buffer_id: usize, config: FloatConfig) -> Self {
        Self {
            id,
            config,
            buffer_id,
        }
    }
}

/// Floating window state.
#[derive(Debug, Clone, Default)]
pub struct FloatState {
    /// Windows by ID.
    windows: std::collections::HashMap<usize, FloatWindow>,
    /// Next ID.
    next_id: usize,
}

impl FloatState {
    /// Creates new float state.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Opens a floating window.
    pub fn open(&mut self, buffer_id: usize, config: FloatConfig) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.windows.insert(id, FloatWindow::new(id, buffer_id, config));
        id
    }

    /// Closes a floating window.
    pub fn close(&mut self, id: usize) -> bool {
        self.windows.remove(&id).is_some()
    }

    /// Gets a floating window.
    pub fn get(&self, id: usize) -> Option<&FloatWindow> {
        self.windows.get(&id)
    }

    /// Returns all windows sorted by zindex.
    pub fn all_sorted(&self) -> Vec<&FloatWindow> {
        let mut windows: Vec<_> = self.windows.values().collect();
        windows.sort_by_key(|w| w.config.zindex);
        windows
    }

    /// Returns the topmost window.
    pub fn topmost(&self) -> Option<&FloatWindow> {
        self.windows.values().max_by_key(|w| w.config.zindex)
    }

    /// Closes all windows.
    pub fn close_all(&mut self) {
        self.windows.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_config_builder() {
        let config = FloatConfig::new()
            .at(10, 20)
            .size(40, 10)
            .with_border(FloatBorder::Rounded)
            .focusable();

        assert_eq!(config.row, 10);
        assert_eq!(config.col, 20);
        assert_eq!(config.border, FloatBorder::Rounded);
        assert!(config.focusable);
    }

    #[test]
    fn test_float_state_open() {
        let mut state = FloatState::new();
        let id = state.open(1, FloatConfig::new());

        assert!(state.get(id).is_some());
    }

    #[test]
    fn test_float_state_close() {
        let mut state = FloatState::new();
        let id = state.open(1, FloatConfig::new());

        assert!(state.close(id));
        assert!(state.get(id).is_none());
    }

    #[test]
    fn test_float_state_topmost() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig { zindex: 10, ..Default::default() });
        state.open(2, FloatConfig { zindex: 50, ..Default::default() });

        let top = state.topmost().unwrap();
        assert_eq!(top.config.zindex, 50);
    }

    #[test]
    fn test_float_state_sorted() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig { zindex: 50, ..Default::default() });
        state.open(2, FloatConfig { zindex: 10, ..Default::default() });

        let sorted = state.all_sorted();
        assert_eq!(sorted[0].config.zindex, 10);
        assert_eq!(sorted[1].config.zindex, 50);
    }

    #[test]
    fn test_float_state_close_all() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig::new());
        state.open(2, FloatConfig::new());

        state.close_all();
        assert!(state.topmost().is_none());
    }
}
