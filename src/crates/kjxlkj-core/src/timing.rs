//! Timing and debounce utilities for kjxlkj editor.
//!
//! Implements timing behavior as specified in `/docs/spec/scripting/timing-debounce.md`.
//!
//! This module provides:
//! - Debounce for delaying execution until input stops
//! - Throttle for limiting execution rate
//! - Timer abstraction for scheduling delayed actions

use std::time::{Duration, Instant};

/// Configuration for debounce behavior.
#[derive(Debug, Clone, Copy)]
pub struct DebounceConfig {
    /// Delay before triggering.
    pub delay: Duration,
}

impl Default for DebounceConfig {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(150),
        }
    }
}

impl DebounceConfig {
    /// Create a debounce config with specific delay in milliseconds.
    pub fn from_millis(ms: u64) -> Self {
        Self {
            delay: Duration::from_millis(ms),
        }
    }
}

/// State for tracking debounce.
#[derive(Debug)]
pub struct Debouncer {
    /// Configuration.
    config: DebounceConfig,
    /// When the last trigger occurred.
    last_trigger: Option<Instant>,
    /// Whether a trigger is pending.
    pending: bool,
}

impl Debouncer {
    /// Create a new debouncer.
    pub fn new(config: DebounceConfig) -> Self {
        Self {
            config,
            last_trigger: None,
            pending: false,
        }
    }

    /// Create a debouncer with default config.
    pub fn with_delay(delay: Duration) -> Self {
        Self::new(DebounceConfig { delay })
    }

    /// Trigger the debouncer. Returns true if ready to fire.
    pub fn trigger(&mut self) -> bool {
        self.last_trigger = Some(Instant::now());
        self.pending = true;
        false // Never fires immediately in debounce
    }

    /// Check if debouncer is ready to fire.
    pub fn is_ready(&self) -> bool {
        if !self.pending {
            return false;
        }
        match self.last_trigger {
            Some(t) => t.elapsed() >= self.config.delay,
            None => false,
        }
    }

    /// Fire the debouncer if ready. Returns true if fired.
    pub fn fire_if_ready(&mut self) -> bool {
        if self.is_ready() {
            self.pending = false;
            true
        } else {
            false
        }
    }

    /// Cancel any pending trigger.
    pub fn cancel(&mut self) {
        self.pending = false;
        self.last_trigger = None;
    }

    /// Get whether there's a pending trigger.
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Get time until ready (if pending).
    pub fn time_until_ready(&self) -> Option<Duration> {
        if !self.pending {
            return None;
        }
        self.last_trigger.map(|t| {
            let elapsed = t.elapsed();
            if elapsed >= self.config.delay {
                Duration::ZERO
            } else {
                self.config.delay - elapsed
            }
        })
    }
}

/// Configuration for throttle behavior.
#[derive(Debug, Clone, Copy)]
pub struct ThrottleConfig {
    /// Minimum interval between executions.
    pub interval: Duration,
}

impl Default for ThrottleConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(16), // 60fps
        }
    }
}

impl ThrottleConfig {
    /// Create a throttle config with specific interval in milliseconds.
    pub fn from_millis(ms: u64) -> Self {
        Self {
            interval: Duration::from_millis(ms),
        }
    }

    /// Create a throttle config for a target FPS.
    pub fn from_fps(fps: u32) -> Self {
        Self {
            interval: Duration::from_micros(1_000_000 / fps as u64),
        }
    }
}

/// State for tracking throttle.
#[derive(Debug)]
pub struct Throttler {
    /// Configuration.
    config: ThrottleConfig,
    /// When the last execution occurred.
    last_execution: Option<Instant>,
}

impl Throttler {
    /// Create a new throttler.
    pub fn new(config: ThrottleConfig) -> Self {
        Self {
            config,
            last_execution: None,
        }
    }

    /// Create a throttler with default config.
    pub fn with_interval(interval: Duration) -> Self {
        Self::new(ThrottleConfig { interval })
    }

    /// Try to execute. Returns true if allowed.
    pub fn try_execute(&mut self) -> bool {
        match self.last_execution {
            Some(t) if t.elapsed() < self.config.interval => false,
            _ => {
                self.last_execution = Some(Instant::now());
                true
            }
        }
    }

    /// Force reset the throttler.
    pub fn reset(&mut self) {
        self.last_execution = None;
    }

    /// Get time until next execution is allowed.
    pub fn time_until_allowed(&self) -> Duration {
        match self.last_execution {
            Some(t) => {
                let elapsed = t.elapsed();
                if elapsed >= self.config.interval {
                    Duration::ZERO
                } else {
                    self.config.interval - elapsed
                }
            }
            None => Duration::ZERO,
        }
    }
}

/// Configuration for key sequence timeout.
#[derive(Debug, Clone, Copy)]
pub struct KeyTimeoutConfig {
    /// Timeout for multi-key sequences.
    pub timeout: Duration,
    /// Timeout for terminal codes (usually shorter).
    pub ttimeout: Duration,
}

impl Default for KeyTimeoutConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(1000),
            ttimeout: Duration::from_millis(100),
        }
    }
}

/// State for tracking key sequence timeout.
#[derive(Debug)]
pub struct KeyTimeout {
    /// Configuration.
    config: KeyTimeoutConfig,
    /// When the key sequence started.
    sequence_start: Option<Instant>,
    /// Whether we're in a terminal code sequence.
    is_terminal_code: bool,
}

impl KeyTimeout {
    /// Create a new key timeout tracker.
    pub fn new(config: KeyTimeoutConfig) -> Self {
        Self {
            config,
            sequence_start: None,
            is_terminal_code: false,
        }
    }

    /// Start a new key sequence.
    pub fn start_sequence(&mut self, is_terminal: bool) {
        self.sequence_start = Some(Instant::now());
        self.is_terminal_code = is_terminal;
    }

    /// Check if the sequence has timed out.
    pub fn is_timed_out(&self) -> bool {
        match self.sequence_start {
            Some(t) => {
                let timeout = if self.is_terminal_code {
                    self.config.ttimeout
                } else {
                    self.config.timeout
                };
                t.elapsed() >= timeout
            }
            None => false,
        }
    }

    /// Reset the timeout tracker.
    pub fn reset(&mut self) {
        self.sequence_start = None;
        self.is_terminal_code = false;
    }

    /// Get the active timeout duration.
    pub fn active_timeout(&self) -> Option<Duration> {
        if self.sequence_start.is_some() {
            Some(if self.is_terminal_code {
                self.config.ttimeout
            } else {
                self.config.timeout
            })
        } else {
            None
        }
    }
}

/// Presets for common timing scenarios.
pub mod presets {
    use super::*;

    /// Debounce preset for search-as-you-type.
    pub fn search_debounce() -> DebounceConfig {
        DebounceConfig::from_millis(150)
    }

    /// Debounce preset for completion.
    pub fn completion_debounce() -> DebounceConfig {
        DebounceConfig::from_millis(50)
    }

    /// Debounce preset for diagnostics.
    pub fn diagnostics_debounce() -> DebounceConfig {
        DebounceConfig::from_millis(100)
    }

    /// Debounce preset for auto-save.
    pub fn autosave_debounce() -> DebounceConfig {
        DebounceConfig::from_millis(1000)
    }

    /// Throttle preset for rendering (60fps).
    pub fn render_throttle() -> ThrottleConfig {
        ThrottleConfig::from_fps(60)
    }

    /// Throttle preset for status line updates.
    pub fn statusline_throttle() -> ThrottleConfig {
        ThrottleConfig::from_millis(100)
    }

    /// Throttle preset for scroll events.
    pub fn scroll_throttle() -> ThrottleConfig {
        ThrottleConfig::from_millis(16)
    }

    /// Throttle preset for resize handling.
    pub fn resize_throttle() -> ThrottleConfig {
        ThrottleConfig::from_millis(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_debounce_config_default() {
        let config = DebounceConfig::default();
        assert_eq!(config.delay, Duration::from_millis(150));
    }

    #[test]
    fn test_debounce_config_from_millis() {
        let config = DebounceConfig::from_millis(200);
        assert_eq!(config.delay, Duration::from_millis(200));
    }

    #[test]
    fn test_debouncer_new() {
        let debouncer = Debouncer::new(DebounceConfig::default());
        assert!(!debouncer.is_pending());
        assert!(!debouncer.is_ready());
    }

    #[test]
    fn test_debouncer_trigger_not_immediate() {
        let mut debouncer = Debouncer::with_delay(Duration::from_millis(100));
        assert!(!debouncer.trigger());
        assert!(debouncer.is_pending());
        assert!(!debouncer.is_ready()); // Not ready immediately
    }

    #[test]
    fn test_debouncer_ready_after_delay() {
        let mut debouncer = Debouncer::with_delay(Duration::from_millis(10));
        debouncer.trigger();
        thread::sleep(Duration::from_millis(15));
        assert!(debouncer.is_ready());
        assert!(debouncer.fire_if_ready());
        assert!(!debouncer.is_pending());
    }

    #[test]
    fn test_debouncer_cancel() {
        let mut debouncer = Debouncer::with_delay(Duration::from_millis(100));
        debouncer.trigger();
        debouncer.cancel();
        assert!(!debouncer.is_pending());
    }

    #[test]
    fn test_debouncer_time_until_ready() {
        let mut debouncer = Debouncer::with_delay(Duration::from_millis(100));
        assert!(debouncer.time_until_ready().is_none());

        debouncer.trigger();
        let time = debouncer.time_until_ready().unwrap();
        assert!(time <= Duration::from_millis(100));
    }

    #[test]
    fn test_throttle_config_default() {
        let config = ThrottleConfig::default();
        assert_eq!(config.interval, Duration::from_millis(16));
    }

    #[test]
    fn test_throttle_config_from_fps() {
        let config = ThrottleConfig::from_fps(60);
        // 60fps = 16.666ms per frame
        assert!(config.interval >= Duration::from_millis(16));
        assert!(config.interval < Duration::from_millis(17));
    }

    #[test]
    fn test_throttler_first_execution_allowed() {
        let mut throttler = Throttler::new(ThrottleConfig::default());
        assert!(throttler.try_execute());
    }

    #[test]
    fn test_throttler_blocks_rapid_execution() {
        let mut throttler = Throttler::with_interval(Duration::from_millis(100));
        assert!(throttler.try_execute());
        assert!(!throttler.try_execute()); // Too soon
    }

    #[test]
    fn test_throttler_allows_after_interval() {
        let mut throttler = Throttler::with_interval(Duration::from_millis(10));
        assert!(throttler.try_execute());
        thread::sleep(Duration::from_millis(15));
        assert!(throttler.try_execute()); // Enough time passed
    }

    #[test]
    fn test_throttler_reset() {
        let mut throttler = Throttler::with_interval(Duration::from_millis(100));
        throttler.try_execute();
        throttler.reset();
        assert!(throttler.try_execute()); // Allowed after reset
    }

    #[test]
    fn test_throttler_time_until_allowed() {
        let mut throttler = Throttler::with_interval(Duration::from_millis(100));
        assert_eq!(throttler.time_until_allowed(), Duration::ZERO);

        throttler.try_execute();
        assert!(throttler.time_until_allowed() > Duration::ZERO);
    }

    #[test]
    fn test_key_timeout_config_default() {
        let config = KeyTimeoutConfig::default();
        assert_eq!(config.timeout, Duration::from_millis(1000));
        assert_eq!(config.ttimeout, Duration::from_millis(100));
    }

    #[test]
    fn test_key_timeout_not_started() {
        let timeout = KeyTimeout::new(KeyTimeoutConfig::default());
        assert!(!timeout.is_timed_out());
        assert!(timeout.active_timeout().is_none());
    }

    #[test]
    fn test_key_timeout_regular_sequence() {
        let mut timeout = KeyTimeout::new(KeyTimeoutConfig::default());
        timeout.start_sequence(false);
        assert!(!timeout.is_timed_out());
        assert_eq!(
            timeout.active_timeout(),
            Some(Duration::from_millis(1000))
        );
    }

    #[test]
    fn test_key_timeout_terminal_code() {
        let mut timeout = KeyTimeout::new(KeyTimeoutConfig::default());
        timeout.start_sequence(true);
        assert_eq!(timeout.active_timeout(), Some(Duration::from_millis(100)));
    }

    #[test]
    fn test_key_timeout_reset() {
        let mut timeout = KeyTimeout::new(KeyTimeoutConfig::default());
        timeout.start_sequence(false);
        timeout.reset();
        assert!(timeout.active_timeout().is_none());
    }

    #[test]
    fn test_presets_search() {
        let config = presets::search_debounce();
        assert_eq!(config.delay, Duration::from_millis(150));
    }

    #[test]
    fn test_presets_completion() {
        let config = presets::completion_debounce();
        assert_eq!(config.delay, Duration::from_millis(50));
    }

    #[test]
    fn test_presets_render_throttle() {
        let config = presets::render_throttle();
        // 60fps
        assert!(config.interval <= Duration::from_millis(17));
    }

    #[test]
    fn test_presets_autosave() {
        let config = presets::autosave_debounce();
        assert_eq!(config.delay, Duration::from_millis(1000));
    }
}
