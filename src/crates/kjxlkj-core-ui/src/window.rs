//! Window management module.
//!
//! Implements the window tree model for splits, tabs, floating windows, and layouts.

use std::collections::HashMap;

/// Unique window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

impl WindowId {
    /// Create a new window ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the inner ID value.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Unique buffer identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the inner ID value.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Direction for splits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Horizontal split (windows stack vertically).
    Horizontal,
    /// Vertical split (windows stack horizontally).
    Vertical,
}

/// Direction for navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
}

/// Window command (wincmd) operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinCmd {
    /// Go to window in direction.
    Goto(Direction),
    /// Go to next window.
    Next,
    /// Go to previous window.
    Previous,
    /// Go to top-left window.
    TopLeft,
    /// Go to bottom-right window.
    BottomRight,
    /// Split horizontally.
    SplitHorizontal,
    /// Split vertically.
    SplitVertical,
    /// Close current window.
    Close,
    /// Close other windows.
    Only,
    /// Quit window (close and quit if last).
    Quit,
    /// Move window to edge.
    Move(Direction),
    /// Rotate windows.
    Rotate(bool), // true = reverse
    /// Exchange with next window.
    Exchange,
    /// Resize height.
    ResizeHeight(i32),
    /// Resize width.
    ResizeWidth(i32),
    /// Make all windows equal size.
    Equal,
    /// Maximize height.
    MaxHeight,
    /// Maximize width.
    MaxWidth,
    /// Move window to new tab.
    MoveToTab,
}

/// Floating window border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatBorder {
    /// No border.
    #[default]
    None,
    /// Single line border.
    Single,
    /// Double line border.
    Double,
    /// Rounded corners.
    Rounded,
    /// Solid block border.
    Solid,
    /// Shadow effect.
    Shadow,
}

impl FloatBorder {
    /// Get border characters: (top-left, top, top-right, left, right, bot-left, bot, bot-right).
    pub fn chars(&self) -> Option<[char; 8]> {
        match self {
            Self::None => None,
            Self::Single => Some(['┌', '─', '┐', '│', '│', '└', '─', '┘']),
            Self::Double => Some(['╔', '═', '╗', '║', '║', '╚', '═', '╝']),
            Self::Rounded => Some(['╭', '─', '╮', '│', '│', '╰', '─', '╯']),
            Self::Solid => Some(['█', '▀', '█', '█', '█', '█', '▄', '█']),
            Self::Shadow => Some(['▛', '▀', '▜', '▌', '▐', '▙', '▄', '▟']),
        }
    }
}

/// Floating window anchor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FloatAnchor {
    /// Anchor to cursor position.
    #[default]
    Cursor,
    /// Anchor to center of screen.
    Center,
    /// Anchor to editor position.
    Position { row: u16, col: u16 },
    /// Anchor to corner.
    Corner(Corner),
}

/// Screen corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Corner {
    /// Top-left.
    TopLeft,
    /// Top-right.
    TopRight,
    /// Bottom-left.
    BottomLeft,
    /// Bottom-right.
    BottomRight,
}

/// Floating window configuration.
#[derive(Debug, Clone)]
pub struct FloatConfig {
    /// Anchor position.
    pub anchor: FloatAnchor,
    /// Width (absolute or percentage).
    pub width: FloatSize,
    /// Height (absolute or percentage).
    pub height: FloatSize,
    /// Border style.
    pub border: FloatBorder,
    /// Title text.
    pub title: Option<String>,
    /// Footer text.
    pub footer: Option<String>,
    /// Z-index for layering.
    pub zindex: u8,
    /// Whether the float is focusable.
    pub focusable: bool,
    /// Close on focus loss.
    pub close_on_blur: bool,
}

/// Size specification for floating windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    /// Absolute size in cells.
    Absolute(u16),
    /// Percentage of screen.
    Percent(u8),
    /// Auto-size based on content.
    Auto,
}

impl Default for FloatConfig {
    fn default() -> Self {
        Self {
            anchor: FloatAnchor::Center,
            width: FloatSize::Percent(60),
            height: FloatSize::Percent(60),
            border: FloatBorder::Rounded,
            title: None,
            footer: None,
            zindex: 50,
            focusable: true,
            close_on_blur: false,
        }
    }
}

impl FloatConfig {
    /// Create a centered float.
    pub fn centered(width: u16, height: u16) -> Self {
        Self {
            anchor: FloatAnchor::Center,
            width: FloatSize::Absolute(width),
            height: FloatSize::Absolute(height),
            ..Default::default()
        }
    }

    /// Create a cursor-anchored float.
    pub fn at_cursor(width: u16, height: u16) -> Self {
        Self {
            anchor: FloatAnchor::Cursor,
            width: FloatSize::Absolute(width),
            height: FloatSize::Absolute(height),
            ..Default::default()
        }
    }

    /// Set title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set border.
    pub fn with_border(mut self, border: FloatBorder) -> Self {
        self.border = border;
        self
    }
}

/// A floating window.
#[derive(Debug, Clone)]
pub struct FloatingWindow {
    /// Window ID.
    pub id: WindowId,
    /// Configuration.
    pub config: FloatConfig,
    /// Buffer ID.
    pub buffer_id: BufferId,
    /// Computed bounds.
    pub bounds: FloatBounds,
}

/// Computed floating window bounds.
#[derive(Debug, Clone, Copy, Default)]
pub struct FloatBounds {
    /// Row position.
    pub row: u16,
    /// Column position.
    pub col: u16,
    /// Width.
    pub width: u16,
    /// Height.
    pub height: u16,
}

impl FloatBounds {
    /// Create new bounds.
    pub fn new(row: u16, col: u16, width: u16, height: u16) -> Self {
        Self { row, col, width, height }
    }

    /// Compute bounds from config and screen size.
    pub fn compute(config: &FloatConfig, screen_width: u16, screen_height: u16, cursor_row: u16, cursor_col: u16) -> Self {
        let width = match config.width {
            FloatSize::Absolute(w) => w.min(screen_width),
            FloatSize::Percent(p) => (screen_width as u32 * p as u32 / 100) as u16,
            FloatSize::Auto => 40, // Default
        };
        let height = match config.height {
            FloatSize::Absolute(h) => h.min(screen_height),
            FloatSize::Percent(p) => (screen_height as u32 * p as u32 / 100) as u16,
            FloatSize::Auto => 20, // Default
        };

        let (row, col) = match config.anchor {
            FloatAnchor::Center => (
                (screen_height.saturating_sub(height)) / 2,
                (screen_width.saturating_sub(width)) / 2,
            ),
            FloatAnchor::Cursor => (cursor_row + 1, cursor_col),
            FloatAnchor::Position { row, col } => (row, col),
            FloatAnchor::Corner(corner) => match corner {
                Corner::TopLeft => (0, 0),
                Corner::TopRight => (0, screen_width.saturating_sub(width)),
                Corner::BottomLeft => (screen_height.saturating_sub(height), 0),
                Corner::BottomRight => (
                    screen_height.saturating_sub(height),
                    screen_width.saturating_sub(width),
                ),
            },
        };

        Self { row, col, width, height }
    }
}

impl FloatingWindow {
    /// Create a new floating window.
    pub fn new(id: WindowId, buffer_id: BufferId, config: FloatConfig) -> Self {
        Self {
            id,
            config,
            buffer_id,
            bounds: FloatBounds::default(),
        }
    }

    /// Update bounds based on screen size.
    pub fn compute_bounds(&mut self, screen_width: u16, screen_height: u16, cursor_row: u16, cursor_col: u16) {
        self.bounds = FloatBounds::compute(&self.config, screen_width, screen_height, cursor_row, cursor_col);
    }
}

/// Window zoom state.
#[derive(Debug, Clone, Default)]
pub struct ZoomState {
    /// Whether zoom is active.
    pub active: bool,
    /// Zoomed window ID.
    pub window_id: Option<WindowId>,
    /// Saved layout before zoom.
    saved_layout: Option<WindowNode>,
}

impl ZoomState {
    /// Toggle zoom for a window.
    pub fn toggle(&mut self, window_id: WindowId, current_layout: &WindowNode) -> Option<WindowNode> {
        if self.active && self.window_id == Some(window_id) {
            // Unzoom: restore saved layout
            self.active = false;
            self.window_id = None;
            self.saved_layout.take()
        } else {
            // Zoom: save current layout
            self.saved_layout = Some(current_layout.clone());
            self.active = true;
            self.window_id = Some(window_id);
            None
        }
    }

    /// Check if a window is zoomed.
    pub fn is_zoomed(&self, window_id: WindowId) -> bool {
        self.active && self.window_id == Some(window_id)
    }
}

/// Window resize mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizeMode {
    /// Not in resize mode.
    #[default]
    None,
    /// Interactive resize mode.
    Interactive,
    /// Resize by steps.
    Step(u16),
}

/// Layout preset types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPreset {
    /// Single window (no splits).
    Single,
    /// Horizontal stack (windows side by side).
    HorizontalStack,
    /// Vertical stack (windows stacked vertically).
    VerticalStack,
    /// Grid layout (2x2).
    Grid,
    /// Main left with stacked right.
    MainLeft,
    /// Main right with stacked left.
    MainRight,
    /// Main top with stacked bottom.
    MainTop,
    /// Main bottom with stacked top.
    MainBottom,
}

/// Per-window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    /// Whether line numbers are shown.
    pub number: bool,
    /// Whether relative line numbers are shown.
    pub relativenumber: bool,
    /// Whether text wraps.
    pub wrap: bool,
    /// Scroll offset (lines from edge).
    pub scrolloff: usize,
    /// Side scroll offset (columns from edge).
    pub sidescrolloff: usize,
    /// Whether to show cursor line highlight.
    pub cursorline: bool,
    /// Whether to show cursor column highlight.
    pub cursorcolumn: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            number: false,
            relativenumber: false,
            wrap: true,
            scrolloff: 0,
            sidescrolloff: 0,
            cursorline: false,
            cursorcolumn: false,
        }
    }
}

/// Window cursor state.
#[derive(Debug, Clone, Default)]
pub struct WindowCursor {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column number (0-indexed).
    pub column: usize,
}

impl WindowCursor {
    /// Create a new cursor at line and column.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Window viewport state.
#[derive(Debug, Clone)]
pub struct WindowViewport {
    /// Top visible line (0-indexed).
    pub top_line: usize,
    /// Left visible column (0-indexed).
    pub left_column: usize,
    /// Height in lines.
    pub height: usize,
    /// Width in columns.
    pub width: usize,
}

impl WindowViewport {
    /// Create a new viewport.
    pub fn new(top_line: usize, height: usize, left_column: usize, width: usize) -> Self {
        Self {
            top_line,
            left_column,
            height,
            width,
        }
    }

    /// Check if a line is visible.
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.top_line && line < self.top_line + self.height
    }
}

impl Default for WindowViewport {
    fn default() -> Self {
        Self {
            top_line: 0,
            left_column: 0,
            height: 24,
            width: 80,
        }
    }
}

/// A single window.
#[derive(Debug, Clone)]
pub struct Window {
    /// Unique window ID.
    pub id: WindowId,
    /// Buffer displayed in this window.
    pub buffer_id: BufferId,
    /// Cursor state.
    pub cursor: WindowCursor,
    /// Viewport state.
    pub viewport: WindowViewport,
    /// Per-window options.
    pub options: WindowOptions,
}

impl Window {
    /// Create a new window.
    pub fn new(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            buffer_id,
            cursor: WindowCursor::default(),
            viewport: WindowViewport::default(),
            options: WindowOptions::default(),
        }
    }

    /// Set the viewport dimensions.
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        self.viewport.width = width;
        self.viewport.height = height;
    }

    /// Scroll to ensure cursor is visible.
    pub fn ensure_cursor_visible(&mut self) {
        let scrolloff = self.options.scrolloff;
        let sidescrolloff = self.options.sidescrolloff;

        // Vertical scrolling
        if self.cursor.line < self.viewport.top_line + scrolloff {
            self.viewport.top_line = self.cursor.line.saturating_sub(scrolloff);
        } else if self.cursor.line >= self.viewport.top_line + self.viewport.height - scrolloff {
            self.viewport.top_line =
                self.cursor.line.saturating_sub(self.viewport.height - 1 - scrolloff);
        }

        // Horizontal scrolling
        if self.cursor.column < self.viewport.left_column + sidescrolloff {
            self.viewport.left_column = self.cursor.column.saturating_sub(sidescrolloff);
        } else if self.cursor.column >= self.viewport.left_column + self.viewport.width - sidescrolloff {
            self.viewport.left_column =
                self.cursor.column.saturating_sub(self.viewport.width - 1 - sidescrolloff);
        }
    }
}

/// A node in the window tree.
#[derive(Debug, Clone)]
pub enum WindowNode {
    /// A leaf window.
    Leaf(Window),
    /// A split containing child nodes.
    Split {
        /// Direction of split.
        direction: SplitDirection,
        /// Child nodes.
        children: Vec<WindowNode>,
        /// Relative sizes (as percentages, sum to 100).
        sizes: Vec<u8>,
    },
}

impl WindowNode {
    /// Create a leaf node.
    pub fn leaf(window: Window) -> Self {
        Self::Leaf(window)
    }

    /// Create a split node.
    pub fn split(direction: SplitDirection, children: Vec<WindowNode>) -> Self {
        let count = children.len();
        let sizes = vec![100 / count as u8; count];
        Self::Split {
            direction,
            children,
            sizes,
        }
    }

    /// Find a window by ID.
    pub fn find(&self, id: WindowId) -> Option<&Window> {
        match self {
            Self::Leaf(w) if w.id == id => Some(w),
            Self::Leaf(_) => None,
            Self::Split { children, .. } => children.iter().find_map(|c| c.find(id)),
        }
    }

    /// Find a window by ID (mutable).
    pub fn find_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        match self {
            Self::Leaf(w) if w.id == id => Some(w),
            Self::Leaf(_) => None,
            Self::Split { children, .. } => children.iter_mut().find_map(|c| c.find_mut(id)),
        }
    }

    /// Get all window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        match self {
            Self::Leaf(w) => vec![w.id],
            Self::Split { children, .. } => children.iter().flat_map(|c| c.window_ids()).collect(),
        }
    }

    /// Count windows.
    pub fn window_count(&self) -> usize {
        match self {
            Self::Leaf(_) => 1,
            Self::Split { children, .. } => children.iter().map(|c| c.window_count()).sum(),
        }
    }
}

/// A tab page containing a window tree.
#[derive(Debug, Clone)]
pub struct TabPage {
    /// Tab index (1-indexed like Vim).
    pub index: usize,
    /// Root window node.
    pub root: WindowNode,
    /// Currently active window ID.
    pub active_window: WindowId,
}

impl TabPage {
    /// Create a new tab with a single window.
    pub fn new(index: usize, window: Window) -> Self {
        let active = window.id;
        Self {
            index,
            root: WindowNode::leaf(window),
            active_window: active,
        }
    }

    /// Get the active window.
    pub fn active(&self) -> Option<&Window> {
        self.root.find(self.active_window)
    }

    /// Get the active window (mutable).
    pub fn active_mut(&mut self) -> Option<&mut Window> {
        self.root.find_mut(self.active_window)
    }

    /// Get all window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        self.root.window_ids()
    }
}

/// Window manager.
#[derive(Debug)]
pub struct WindowManager {
    /// All tab pages.
    tabs: Vec<TabPage>,
    /// Current tab index (0-indexed).
    current_tab: usize,
    /// Next window ID.
    next_window_id: u64,
    /// Window-to-buffer mapping.
    window_buffers: HashMap<WindowId, BufferId>,
}

impl WindowManager {
    /// Create a new window manager with an initial window.
    pub fn new(buffer_id: BufferId) -> Self {
        let window_id = WindowId::new(1);
        let window = Window::new(window_id, buffer_id);
        let tab = TabPage::new(1, window);

        let mut window_buffers = HashMap::new();
        window_buffers.insert(window_id, buffer_id);

        Self {
            tabs: vec![tab],
            current_tab: 0,
            next_window_id: 2,
            window_buffers,
        }
    }

    /// Get current tab.
    pub fn current_tab(&self) -> Option<&TabPage> {
        self.tabs.get(self.current_tab)
    }

    /// Get current tab (mutable).
    pub fn current_tab_mut(&mut self) -> Option<&mut TabPage> {
        self.tabs.get_mut(self.current_tab)
    }

    /// Get active window.
    pub fn active_window(&self) -> Option<&Window> {
        self.current_tab().and_then(|t| t.active())
    }

    /// Get active window (mutable).
    pub fn active_window_mut(&mut self) -> Option<&mut Window> {
        self.current_tab_mut().and_then(|t| t.active_mut())
    }

    /// Create a new window ID.
    fn next_id(&mut self) -> WindowId {
        let id = WindowId::new(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Split the active window.
    pub fn split(&mut self, direction: SplitDirection, buffer_id: BufferId) -> WindowId {
        let new_id = self.next_id();
        let new_window = Window::new(new_id, buffer_id);
        self.window_buffers.insert(new_id, buffer_id);

        if let Some(tab) = self.current_tab_mut() {
            // Find and replace the active window with a split
            let active_id = tab.active_window;
            if let Some(active) = tab.root.find_mut(active_id) {
                let old_window = active.clone();
                let new_node = WindowNode::split(
                    direction,
                    vec![WindowNode::leaf(old_window), WindowNode::leaf(new_window)],
                );
                // This is simplified - real impl would need to replace in tree
                tab.root = new_node;
            }
        }

        new_id
    }

    /// Create a new tab.
    pub fn new_tab(&mut self, buffer_id: BufferId) -> usize {
        let new_id = self.next_id();
        let window = Window::new(new_id, buffer_id);
        self.window_buffers.insert(new_id, buffer_id);

        let index = self.tabs.len() + 1;
        let tab = TabPage::new(index, window);
        self.tabs.push(tab);
        self.current_tab = self.tabs.len() - 1;
        index
    }

    /// Go to next tab.
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.current_tab = (self.current_tab + 1) % self.tabs.len();
        }
    }

    /// Go to previous tab.
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.current_tab = if self.current_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.current_tab - 1
            };
        }
    }

    /// Go to a specific tab (1-indexed).
    pub fn goto_tab(&mut self, index: usize) {
        if index > 0 && index <= self.tabs.len() {
            self.current_tab = index - 1;
        }
    }

    /// Get tab count.
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Get window count in current tab.
    pub fn window_count(&self) -> usize {
        self.current_tab().map(|t| t.root.window_count()).unwrap_or(0)
    }

    /// Close current tab.
    pub fn close_tab(&mut self) -> bool {
        if self.tabs.len() <= 1 {
            return false;
        }

        self.tabs.remove(self.current_tab);
        if self.current_tab >= self.tabs.len() {
            self.current_tab = self.tabs.len() - 1;
        }
        true
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new(BufferId::new(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_id() {
        let id = WindowId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_window_new() {
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        assert_eq!(window.id.value(), 1);
        assert_eq!(window.buffer_id.value(), 1);
    }

    #[test]
    fn test_window_dimensions() {
        let mut window = Window::new(WindowId::new(1), BufferId::new(1));
        window.set_dimensions(120, 40);
        assert_eq!(window.viewport.width, 120);
        assert_eq!(window.viewport.height, 40);
    }

    #[test]
    fn test_window_ensure_cursor_visible() {
        let mut window = Window::new(WindowId::new(1), BufferId::new(1));
        window.set_dimensions(80, 10);
        window.cursor.line = 15;
        window.ensure_cursor_visible();
        assert!(window.viewport.is_line_visible(15));
    }

    #[test]
    fn test_window_node_leaf() {
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        let node = WindowNode::leaf(window);
        assert_eq!(node.window_count(), 1);
    }

    #[test]
    fn test_window_node_split() {
        let w1 = Window::new(WindowId::new(1), BufferId::new(1));
        let w2 = Window::new(WindowId::new(2), BufferId::new(1));
        let node = WindowNode::split(
            SplitDirection::Horizontal,
            vec![WindowNode::leaf(w1), WindowNode::leaf(w2)],
        );
        assert_eq!(node.window_count(), 2);
    }

    #[test]
    fn test_window_node_find() {
        let w1 = Window::new(WindowId::new(1), BufferId::new(1));
        let w2 = Window::new(WindowId::new(2), BufferId::new(1));
        let node = WindowNode::split(
            SplitDirection::Horizontal,
            vec![WindowNode::leaf(w1), WindowNode::leaf(w2)],
        );
        assert!(node.find(WindowId::new(1)).is_some());
        assert!(node.find(WindowId::new(2)).is_some());
        assert!(node.find(WindowId::new(3)).is_none());
    }

    #[test]
    fn test_window_node_window_ids() {
        let w1 = Window::new(WindowId::new(1), BufferId::new(1));
        let w2 = Window::new(WindowId::new(2), BufferId::new(1));
        let node = WindowNode::split(
            SplitDirection::Horizontal,
            vec![WindowNode::leaf(w1), WindowNode::leaf(w2)],
        );
        let ids = node.window_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&WindowId::new(1)));
        assert!(ids.contains(&WindowId::new(2)));
    }

    #[test]
    fn test_tab_page_new() {
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        let tab = TabPage::new(1, window);
        assert_eq!(tab.index, 1);
        assert_eq!(tab.active_window.value(), 1);
    }

    #[test]
    fn test_tab_page_active() {
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        let tab = TabPage::new(1, window);
        assert!(tab.active().is_some());
        assert_eq!(tab.active().unwrap().id.value(), 1);
    }

    #[test]
    fn test_window_manager_new() {
        let wm = WindowManager::new(BufferId::new(1));
        assert_eq!(wm.tab_count(), 1);
        assert_eq!(wm.window_count(), 1);
    }

    #[test]
    fn test_window_manager_new_tab() {
        let mut wm = WindowManager::new(BufferId::new(1));
        wm.new_tab(BufferId::new(2));
        assert_eq!(wm.tab_count(), 2);
        assert_eq!(wm.current_tab, 1);
    }

    #[test]
    fn test_window_manager_tab_navigation() {
        let mut wm = WindowManager::new(BufferId::new(1));
        wm.new_tab(BufferId::new(2));
        wm.new_tab(BufferId::new(3));

        assert_eq!(wm.current_tab, 2);

        wm.prev_tab();
        assert_eq!(wm.current_tab, 1);

        wm.next_tab();
        assert_eq!(wm.current_tab, 2);

        wm.goto_tab(1);
        assert_eq!(wm.current_tab, 0);
    }

    #[test]
    fn test_window_manager_close_tab() {
        let mut wm = WindowManager::new(BufferId::new(1));
        wm.new_tab(BufferId::new(2));

        assert_eq!(wm.tab_count(), 2);
        assert!(wm.close_tab());
        assert_eq!(wm.tab_count(), 1);
        assert!(!wm.close_tab()); // Can't close last tab
    }

    #[test]
    fn test_window_options_default() {
        let opts = WindowOptions::default();
        assert!(!opts.number);
        assert!(!opts.relativenumber);
        assert!(opts.wrap);
    }

    #[test]
    fn test_viewport_is_line_visible() {
        let vp = WindowViewport::new(10, 20, 0, 80);
        assert!(!vp.is_line_visible(9));
        assert!(vp.is_line_visible(10));
        assert!(vp.is_line_visible(29));
        assert!(!vp.is_line_visible(30));
    }

    // Floating window tests
    #[test]
    fn test_float_border_chars() {
        assert!(FloatBorder::None.chars().is_none());
        assert!(FloatBorder::Single.chars().is_some());
        assert!(FloatBorder::Rounded.chars().is_some());
    }

    #[test]
    fn test_float_config_centered() {
        let config = FloatConfig::centered(40, 20);
        assert_eq!(config.anchor, FloatAnchor::Center);
        assert_eq!(config.width, FloatSize::Absolute(40));
    }

    #[test]
    fn test_float_config_at_cursor() {
        let config = FloatConfig::at_cursor(30, 10);
        assert_eq!(config.anchor, FloatAnchor::Cursor);
    }

    #[test]
    fn test_float_config_with_title() {
        let config = FloatConfig::default().with_title("Test");
        assert_eq!(config.title, Some("Test".to_string()));
    }

    #[test]
    fn test_float_bounds_compute_center() {
        let config = FloatConfig::centered(40, 20);
        let bounds = FloatBounds::compute(&config, 100, 50, 0, 0);
        assert_eq!(bounds.width, 40);
        assert_eq!(bounds.height, 20);
        assert_eq!(bounds.col, 30); // (100-40)/2
        assert_eq!(bounds.row, 15); // (50-20)/2
    }

    #[test]
    fn test_float_bounds_compute_cursor() {
        let config = FloatConfig::at_cursor(20, 10);
        let bounds = FloatBounds::compute(&config, 100, 50, 5, 10);
        assert_eq!(bounds.row, 6); // cursor_row + 1
        assert_eq!(bounds.col, 10); // cursor_col
    }

    #[test]
    fn test_float_bounds_compute_corner() {
        let config = FloatConfig {
            anchor: FloatAnchor::Corner(Corner::BottomRight),
            width: FloatSize::Absolute(20),
            height: FloatSize::Absolute(10),
            ..Default::default()
        };
        let bounds = FloatBounds::compute(&config, 100, 50, 0, 0);
        assert_eq!(bounds.col, 80);
        assert_eq!(bounds.row, 40);
    }

    #[test]
    fn test_floating_window_new() {
        let fw = FloatingWindow::new(
            WindowId::new(1),
            BufferId::new(1),
            FloatConfig::centered(40, 20),
        );
        assert_eq!(fw.id.value(), 1);
    }

    #[test]
    fn test_floating_window_compute_bounds() {
        let mut fw = FloatingWindow::new(
            WindowId::new(1),
            BufferId::new(1),
            FloatConfig::centered(40, 20),
        );
        fw.compute_bounds(100, 50, 0, 0);
        assert_eq!(fw.bounds.width, 40);
    }

    // Zoom state tests
    #[test]
    fn test_zoom_state_default() {
        let zoom = ZoomState::default();
        assert!(!zoom.active);
        assert!(zoom.window_id.is_none());
    }

    #[test]
    fn test_zoom_state_toggle() {
        let mut zoom = ZoomState::default();
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        let layout = WindowNode::leaf(window);
        
        // First toggle: zoom in
        let restored = zoom.toggle(WindowId::new(1), &layout);
        assert!(zoom.active);
        assert_eq!(zoom.window_id, Some(WindowId::new(1)));
        assert!(restored.is_none());
        
        // Second toggle: zoom out
        let restored = zoom.toggle(WindowId::new(1), &layout);
        assert!(!zoom.active);
        assert!(restored.is_some());
    }

    #[test]
    fn test_zoom_state_is_zoomed() {
        let mut zoom = ZoomState::default();
        let window = Window::new(WindowId::new(1), BufferId::new(1));
        let layout = WindowNode::leaf(window);
        
        assert!(!zoom.is_zoomed(WindowId::new(1)));
        zoom.toggle(WindowId::new(1), &layout);
        assert!(zoom.is_zoomed(WindowId::new(1)));
        assert!(!zoom.is_zoomed(WindowId::new(2)));
    }

    // Resize mode and layout preset tests
    #[test]
    fn test_resize_mode_default() {
        assert_eq!(ResizeMode::default(), ResizeMode::None);
    }

    #[test]
    fn test_layout_preset_variants() {
        let presets = [
            LayoutPreset::Single,
            LayoutPreset::HorizontalStack,
            LayoutPreset::VerticalStack,
            LayoutPreset::Grid,
            LayoutPreset::MainLeft,
        ];
        assert_eq!(presets.len(), 5);
    }

    // WinCmd tests
    #[test]
    fn test_wincmd_variants() {
        let cmd = WinCmd::Goto(Direction::Left);
        assert_eq!(cmd, WinCmd::Goto(Direction::Left));
        
        let cmd = WinCmd::ResizeHeight(5);
        assert_eq!(cmd, WinCmd::ResizeHeight(5));
    }

    #[test]
    fn test_float_size_percent() {
        let config = FloatConfig {
            width: FloatSize::Percent(50),
            height: FloatSize::Percent(50),
            ..Default::default()
        };
        let bounds = FloatBounds::compute(&config, 100, 80, 0, 0);
        assert_eq!(bounds.width, 50);
        assert_eq!(bounds.height, 40);
    }

    #[test]
    fn test_float_size_auto() {
        let config = FloatConfig {
            width: FloatSize::Auto,
            height: FloatSize::Auto,
            ..Default::default()
        };
        let bounds = FloatBounds::compute(&config, 100, 80, 0, 0);
        assert_eq!(bounds.width, 40); // default
        assert_eq!(bounds.height, 20); // default
    }
}
