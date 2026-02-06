/// Latency measurement and performance probe utilities.
use std::time::{Duration, Instant};

/// A probe that tracks min/max/avg latency for a named operation.
#[derive(Debug, Clone)]
pub struct LatencyProbe {
    pub name: String,
    samples: Vec<Duration>,
    max_samples: usize,
}

impl LatencyProbe {
    pub fn new(name: impl Into<String>, max_samples: usize) -> Self {
        Self { name: name.into(), samples: Vec::new(), max_samples }
    }

    /// Record a duration sample.
    pub fn record(&mut self, d: Duration) {
        if self.samples.len() >= self.max_samples {
            self.samples.remove(0);
        }
        self.samples.push(d);
    }

    /// Record elapsed time since an instant.
    pub fn record_since(&mut self, start: Instant) {
        self.record(start.elapsed());
    }

    /// Minimum latency seen.
    pub fn min(&self) -> Option<Duration> { self.samples.iter().copied().min() }

    /// Maximum latency seen.
    pub fn max(&self) -> Option<Duration> { self.samples.iter().copied().max() }

    /// Average latency.
    pub fn avg(&self) -> Option<Duration> {
        if self.samples.is_empty() { return None; }
        let total: Duration = self.samples.iter().sum();
        Some(total / self.samples.len() as u32)
    }

    /// p95 latency (95th percentile).
    pub fn p95(&self) -> Option<Duration> {
        if self.samples.is_empty() { return None; }
        let mut sorted = self.samples.clone();
        sorted.sort();
        let idx = ((sorted.len() as f64) * 0.95).ceil() as usize;
        Some(sorted[idx.saturating_sub(1).min(sorted.len() - 1)])
    }

    /// Number of samples recorded.
    pub fn count(&self) -> usize { self.samples.len() }

    /// Reset all samples.
    pub fn reset(&mut self) { self.samples.clear(); }

    /// Format a human-readable summary.
    pub fn summary(&self) -> String {
        match (self.min(), self.max(), self.avg(), self.p95()) {
            (Some(min), Some(max), Some(avg), Some(p95)) => {
                format!("{}: min={:?} max={:?} avg={:?} p95={:?} n={}",
                    self.name, min, max, avg, p95, self.count())
            }
            _ => format!("{}: no samples", self.name),
        }
    }
}

/// Check whether a duration exceeds a latency budget.
pub fn exceeds_budget(d: Duration, budget: Duration) -> bool { d > budget }

/// Standard latency budgets from the spec.
pub struct LatencyBudgets;

impl LatencyBudgets {
    /// Keystroke-to-screen budget (target: responsive typing).
    pub const KEYSTROKE: Duration = Duration::from_millis(16);
    /// Render frame budget (target: 60fps).
    pub const FRAME: Duration = Duration::from_millis(16);
    /// Resize settle budget.
    pub const RESIZE: Duration = Duration::from_millis(50);
    /// Scroll coalescing budget.
    pub const SCROLL_COALESCE: Duration = Duration::from_millis(8);
}

/// A guard that records elapsed time into a probe on drop.
pub struct TimingGuard<'a> {
    probe: &'a mut LatencyProbe,
    start: Instant,
}

impl<'a> TimingGuard<'a> {
    pub fn start(probe: &'a mut LatencyProbe) -> Self {
        Self { probe, start: Instant::now() }
    }
}

impl<'a> Drop for TimingGuard<'a> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.probe.record(elapsed);
    }
}

/// Detect whether idle redraw is occurring (busy-loop check).
pub fn is_idle_busy_loop(redraw_count: u64, elapsed: Duration) -> bool {
    if elapsed.as_secs() == 0 { return false; }
    let rate = redraw_count as f64 / elapsed.as_secs_f64();
    rate > 120.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_stats() {
        let mut p = LatencyProbe::new("test", 100);
        p.record(Duration::from_micros(100));
        p.record(Duration::from_micros(200));
        p.record(Duration::from_micros(150));
        assert_eq!(p.count(), 3);
        assert_eq!(p.min(), Some(Duration::from_micros(100)));
        assert_eq!(p.max(), Some(Duration::from_micros(200)));
        assert_eq!(p.avg(), Some(Duration::from_micros(150)));
    }

    #[test]
    fn p95_single() {
        let mut p = LatencyProbe::new("p95", 100);
        p.record(Duration::from_millis(1));
        assert_eq!(p.p95(), Some(Duration::from_millis(1)));
    }

    #[test]
    fn max_samples_evicts() {
        let mut p = LatencyProbe::new("evict", 3);
        for i in 1..=5 { p.record(Duration::from_millis(i)); }
        assert_eq!(p.count(), 3);
        assert_eq!(p.min(), Some(Duration::from_millis(3)));
    }

    #[test]
    fn reset_clears() {
        let mut p = LatencyProbe::new("r", 100);
        p.record(Duration::from_millis(1));
        p.reset();
        assert_eq!(p.count(), 0);
        assert!(p.min().is_none());
    }

    #[test]
    fn exceeds_budget_check() {
        assert!(exceeds_budget(Duration::from_millis(20), Duration::from_millis(16)));
        assert!(!exceeds_budget(Duration::from_millis(10), Duration::from_millis(16)));
    }

    #[test]
    fn budget_constants() {
        assert_eq!(LatencyBudgets::KEYSTROKE.as_millis(), 16);
        assert_eq!(LatencyBudgets::FRAME.as_millis(), 16);
        assert_eq!(LatencyBudgets::RESIZE.as_millis(), 50);
    }

    #[test]
    fn summary_format() {
        let mut p = LatencyProbe::new("test", 100);
        p.record(Duration::from_micros(500));
        let s = p.summary();
        assert!(s.contains("test:"));
        assert!(s.contains("n=1"));
    }

    #[test]
    fn idle_busy_loop_detection() {
        assert!(is_idle_busy_loop(500, Duration::from_secs(1)));
        assert!(!is_idle_busy_loop(60, Duration::from_secs(1)));
        assert!(!is_idle_busy_loop(0, Duration::from_secs(0)));
    }

    #[test]
    fn empty_summary() {
        let p = LatencyProbe::new("empty", 100);
        assert!(p.summary().contains("no samples"));
    }
}
