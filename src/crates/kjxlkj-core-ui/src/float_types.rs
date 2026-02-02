//! Floating window types and configuration.

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
