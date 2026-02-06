/// Input timing and debouncing — coalesce rapid events, detect idle.

use std::time::{Duration, Instant};

/// Debounce state for a specific event category.
#[derive(Debug, Clone)]
pub struct Debouncer {
    pub delay: Duration,
    last_trigger: Option<Instant>,
    pending: bool,
}

impl Debouncer {
    pub fn new(delay_ms: u64) -> Self {
        Self { delay: Duration::from_millis(delay_ms), last_trigger: None, pending: false }
    }

    /// Signal an event. Returns true if the event should fire now.
    pub fn signal(&mut self, now: Instant) -> bool {
        if let Some(last) = self.last_trigger {
            if now.duration_since(last) < self.delay {
                self.pending = true;
                return false;
            }
        }
        self.last_trigger = Some(now);
        self.pending = false;
        true
    }

    /// Check if there's a pending event that should fire (called on timer).
    pub fn check_pending(&mut self, now: Instant) -> bool {
        if !self.pending { return false; }
        if let Some(last) = self.last_trigger {
            if now.duration_since(last) >= self.delay {
                self.pending = false;
                self.last_trigger = Some(now);
                return true;
            }
        }
        false
    }

    pub fn reset(&mut self) { self.last_trigger = None; self.pending = false; }
    pub fn is_pending(&self) -> bool { self.pending }
}

/// Coalescer for rapid resize events.
#[derive(Debug)]
pub struct ResizeCoalescer {
    last_size: Option<(u16, u16)>,
    last_event: Option<Instant>,
    pub settle_ms: u64,
}

impl ResizeCoalescer {
    pub fn new(settle_ms: u64) -> Self {
        Self { last_size: None, last_event: None, settle_ms }
    }

    /// Record a resize event. Returns true if it should be processed immediately.
    pub fn record(&mut self, w: u16, h: u16, now: Instant) -> bool {
        self.last_size = Some((w, h));
        let should_process = match self.last_event {
            None => true,
            Some(prev) => now.duration_since(prev) >= Duration::from_millis(self.settle_ms),
        };
        self.last_event = Some(now);
        should_process
    }

    /// Get the latest size if settled.
    pub fn settled_size(&self, now: Instant) -> Option<(u16, u16)> {
        if let (Some(size), Some(last)) = (self.last_size, self.last_event) {
            if now.duration_since(last) >= Duration::from_millis(self.settle_ms) {
                return Some(size);
            }
        }
        None
    }
}

/// Idle detector — fires after no input for a specified duration.
#[derive(Debug)]
pub struct IdleDetector {
    pub timeout: Duration,
    last_activity: Instant,
    idle_fired: bool,
}

impl IdleDetector {
    pub fn new(timeout_ms: u64) -> Self {
        Self { timeout: Duration::from_millis(timeout_ms), last_activity: Instant::now(), idle_fired: false }
    }

    pub fn activity(&mut self, now: Instant) {
        self.last_activity = now;
        self.idle_fired = false;
    }

    pub fn check_idle(&mut self, now: Instant) -> bool {
        if self.idle_fired { return false; }
        if now.duration_since(self.last_activity) >= self.timeout {
            self.idle_fired = true;
            return true;
        }
        false
    }

    pub fn is_idle(&self, now: Instant) -> bool {
        now.duration_since(self.last_activity) >= self.timeout
    }
}

/// Input rate tracker for backpressure detection.
#[derive(Debug)]
pub struct InputRateTracker {
    events: Vec<Instant>,
    window: Duration,
}

impl InputRateTracker {
    pub fn new(window_ms: u64) -> Self {
        Self { events: Vec::new(), window: Duration::from_millis(window_ms) }
    }

    pub fn record(&mut self, now: Instant) {
        self.events.push(now);
        let cutoff = now - self.window;
        self.events.retain(|t| *t >= cutoff);
    }

    pub fn rate(&self) -> f64 {
        if self.events.len() < 2 { return 0.0; }
        let span = self.events.last().unwrap().duration_since(*self.events.first().unwrap());
        if span.is_zero() { return 0.0; }
        (self.events.len() as f64 - 1.0) / span.as_secs_f64()
    }

    pub fn is_burst(&self, threshold: f64) -> bool { self.rate() > threshold }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debounce_immediate_first() {
        let mut d = Debouncer::new(100);
        let now = Instant::now();
        assert!(d.signal(now));
    }

    #[test]
    fn debounce_suppresses_rapid() {
        let mut d = Debouncer::new(100);
        let now = Instant::now();
        assert!(d.signal(now));
        assert!(!d.signal(now + Duration::from_millis(10)));
        assert!(d.is_pending());
    }

    #[test]
    fn debounce_check_pending_fires() {
        let mut d = Debouncer::new(50);
        let now = Instant::now();
        d.signal(now);
        d.signal(now + Duration::from_millis(10));
        assert!(d.check_pending(now + Duration::from_millis(60)));
    }

    #[test]
    fn resize_coalescer() {
        let mut rc = ResizeCoalescer::new(50);
        let now = Instant::now();
        assert!(rc.record(80, 24, now));
        assert!(!rc.record(120, 40, now + Duration::from_millis(10)));
    }

    #[test]
    fn resize_settled() {
        let mut rc = ResizeCoalescer::new(50);
        let now = Instant::now();
        rc.record(80, 24, now);
        assert!(rc.settled_size(now + Duration::from_millis(100)).is_some());
    }

    #[test]
    fn idle_detector_fires_once() {
        let now = Instant::now();
        let mut id = IdleDetector::new(100);
        id.last_activity = now;
        assert!(!id.check_idle(now + Duration::from_millis(50)));
        assert!(id.check_idle(now + Duration::from_millis(150)));
        assert!(!id.check_idle(now + Duration::from_millis(200))); // Already fired
    }

    #[test]
    fn idle_reset_on_activity() {
        let now = Instant::now();
        let mut id = IdleDetector::new(100);
        id.last_activity = now;
        id.activity(now + Duration::from_millis(80));
        assert!(!id.check_idle(now + Duration::from_millis(150)));
    }

    #[test]
    fn input_rate_tracker() {
        let mut t = InputRateTracker::new(1000);
        let now = Instant::now();
        for i in 0..10 {
            t.record(now + Duration::from_millis(i * 100));
        }
        assert!(t.rate() > 5.0);
    }

    #[test]
    fn rate_not_burst() {
        let t = InputRateTracker::new(1000);
        assert!(!t.is_burst(100.0));
    }
}
