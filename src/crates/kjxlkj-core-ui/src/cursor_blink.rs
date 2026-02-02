//! Cursor blinking state manager.

use std::time::{Duration, Instant};

/// Cursor blink state manager.
#[derive(Debug, Clone)]
pub struct CursorBlink {
    enabled: bool,
    interval: Duration,
    last_toggle: Instant,
    visible: bool,
    last_input: Instant,
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

        if now.duration_since(self.last_input) < self.input_delay {
            if !self.visible {
                self.visible = true;
                return true;
            }
            return false;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_cursor_blink_set_enabled() {
        let mut blink = CursorBlink::new();
        blink.set_enabled(false);
        assert!(!blink.is_enabled());
        assert!(blink.is_visible());
    }
}
