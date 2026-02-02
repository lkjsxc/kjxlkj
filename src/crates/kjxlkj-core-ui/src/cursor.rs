//! Cursor appearance and blinking.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Cursor shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorShape {
    /// Block cursor.
    Block,
    /// Underline cursor.
    Underline,
    /// Vertical bar cursor.
    Bar,
}

impl Default for CursorShape {
    fn default() -> Self {
        Self::Block
    }
}

/// Cursor blink state manager.
#[derive(Debug, Clone)]
pub struct CursorBlink {
    /// Whether blinking is enabled.
    enabled: bool,
    /// Blink interval.
    interval: Duration,
    /// Last toggle time.
    last_toggle: Instant,
    /// Current visibility.
    visible: bool,
    /// Time of last keystroke (resets blink).
    last_input: Instant,
    /// Delay before starting to blink after input.
    input_delay: Duration,
}

impl Default for CursorBlink {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorBlink {
    /// Creates a new cursor blink state.
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            enabled: true,
            interval: Duration::from_millis(530),
            last_toggle: now,
            visible: true,
            last_input: now,
            input_delay: Duration::from_millis(500),
        }
    }

    /// Creates with blinking disabled.
    pub fn disabled() -> Self {
        let mut blink = Self::new();
        blink.enabled = false;
        blink.visible = true;
        blink
    }

    /// Returns if blinking is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables blinking.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.visible = true;
        }
    }

    /// Sets the blink interval.
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    /// Returns the current visibility.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Called on user input (resets blink, shows cursor).
    pub fn on_input(&mut self) {
        self.last_input = Instant::now();
        self.visible = true;
        self.last_toggle = self.last_input;
    }

    /// Updates the blink state. Returns true if visibility changed.
    pub fn update(&mut self) -> bool {
        if !self.enabled {
            return false;
        }

        let now = Instant::now();

        // Don't blink right after input
        if now.duration_since(self.last_input) < self.input_delay {
            if !self.visible {
                self.visible = true;
                return true;
            }
            return false;
        }

        // Check if we should toggle
        if now.duration_since(self.last_toggle) >= self.interval {
            self.visible = !self.visible;
            self.last_toggle = now;
            return true;
        }

        false
    }

    /// Resets to visible state.
    pub fn reset(&mut self) {
        self.visible = true;
        self.last_toggle = Instant::now();
        self.last_input = self.last_toggle;
    }
}

/// Cursor appearance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorAppearance {
    /// Shape in normal mode.
    pub normal_shape: CursorShape,
    /// Shape in insert mode.
    pub insert_shape: CursorShape,
    /// Shape in replace mode.
    pub replace_shape: CursorShape,
    /// Shape in visual mode.
    pub visual_shape: CursorShape,
}

impl Default for CursorAppearance {
    fn default() -> Self {
        Self {
            normal_shape: CursorShape::Block,
            insert_shape: CursorShape::Bar,
            replace_shape: CursorShape::Underline,
            visual_shape: CursorShape::Block,
        }
    }
}

impl CursorAppearance {
    /// Creates a new cursor appearance config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns shape for a mode name.
    pub fn shape_for_mode(&self, mode: &str) -> CursorShape {
        match mode.to_lowercase().as_str() {
            "normal" => self.normal_shape,
            "insert" => self.insert_shape,
            "replace" => self.replace_shape,
            "visual" | "visualline" | "visualblock" => self.visual_shape,
            _ => self.normal_shape,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_shape_default() {
        assert_eq!(CursorShape::default(), CursorShape::Block);
    }

    #[test]
    fn test_cursor_blink_new() {
        let blink = CursorBlink::new();
        assert!(blink.is_enabled());
        assert!(blink.is_visible());
    }

    #[test]
    fn test_cursor_blink_disabled() {
        let blink = CursorBlink::disabled();
        assert!(!blink.is_enabled());
        assert!(blink.is_visible());
    }

    #[test]
    fn test_cursor_blink_on_input() {
        let mut blink = CursorBlink::new();
        blink.visible = false;
        blink.on_input();
        assert!(blink.is_visible());
    }

    #[test]
    fn test_cursor_appearance_default() {
        let app = CursorAppearance::default();
        assert_eq!(app.normal_shape, CursorShape::Block);
        assert_eq!(app.insert_shape, CursorShape::Bar);
        assert_eq!(app.replace_shape, CursorShape::Underline);
    }

    #[test]
    fn test_cursor_appearance_shape_for_mode() {
        let app = CursorAppearance::default();
        assert_eq!(app.shape_for_mode("normal"), CursorShape::Block);
        assert_eq!(app.shape_for_mode("insert"), CursorShape::Bar);
        assert_eq!(app.shape_for_mode("visual"), CursorShape::Block);
    }
}
