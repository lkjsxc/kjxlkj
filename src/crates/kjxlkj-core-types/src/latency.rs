//! Latency budget constants and measurement probes.

use std::time::{Duration, Instant};

/// Latency budget constants for different operation classes.
pub struct LatencyBudget;

impl LatencyBudget {
    /// Maximum latency for a keystroke response.
    pub const KEYSTROKE: Duration = Duration::from_millis(16);
    /// Maximum latency for a rendered frame.
    pub const FRAME: Duration = Duration::from_millis(16);
    /// Maximum latency for a terminal resize.
    pub const RESIZE: Duration = Duration::from_millis(50);
    /// Maximum latency for a scroll update.
    pub const SCROLL: Duration = Duration::from_millis(8);
}

/// Records timing samples and computes statistics.
#[derive(Debug, Clone)]
pub struct LatencyProbe {
    samples: Vec<Duration>,
}

impl LatencyProbe {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn record(&mut self, d: Duration) {
        self.samples.push(d);
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn min(&self) -> Option<Duration> {
        self.samples.iter().copied().min()
    }

    pub fn max(&self) -> Option<Duration> {
        self.samples.iter().copied().max()
    }

    pub fn avg(&self) -> Option<Duration> {
        if self.samples.is_empty() {
            return None;
        }
        let total: Duration = self.samples.iter().sum();
        Some(total / self.samples.len() as u32)
    }

    pub fn p95(&self) -> Option<Duration> {
        if self.samples.is_empty() {
            return None;
        }
        let mut sorted = self.samples.clone();
        sorted.sort();
        let idx = (sorted.len() as f64 * 0.95).ceil() as usize;
        Some(sorted[idx.min(sorted.len() - 1)])
    }

    /// Returns `true` if the p95 latency exceeds the given budget.
    pub fn exceeds_budget(&self, budget: Duration) -> bool {
        self.p95().is_some_and(|p| p > budget)
    }
}

impl Default for LatencyProbe {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII guard that records elapsed time into a `LatencyProbe` on drop.
pub struct TimingGuard<'a> {
    start: Instant,
    probe: &'a mut LatencyProbe,
}

impl<'a> TimingGuard<'a> {
    pub fn start(probe: &'a mut LatencyProbe) -> Self {
        Self {
            start: Instant::now(),
            probe,
        }
    }
}

impl Drop for TimingGuard<'_> {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        self.probe.record(elapsed);
    }
}

/// Returns `true` if the given FPS indicates an idle busy-loop (>120 FPS).
pub fn is_idle_busy_loop(fps: f64) -> bool {
    fps > 120.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_statistics() {
        let mut p = LatencyProbe::new();
        p.record(Duration::from_millis(5));
        p.record(Duration::from_millis(10));
        p.record(Duration::from_millis(15));
        assert_eq!(p.count(), 3);
        assert_eq!(p.min(), Some(Duration::from_millis(5)));
        assert_eq!(p.max(), Some(Duration::from_millis(15)));
        assert_eq!(p.avg(), Some(Duration::from_millis(10)));
    }

    #[test]
    fn exceeds_budget() {
        let mut p = LatencyProbe::new();
        p.record(Duration::from_millis(20));
        assert!(p.exceeds_budget(LatencyBudget::KEYSTROKE));
    }

    #[test]
    fn idle_busy_loop_check() {
        assert!(is_idle_busy_loop(144.0));
        assert!(!is_idle_busy_loop(60.0));
    }

    #[test]
    fn empty_probe() {
        let p = LatencyProbe::new();
        assert_eq!(p.min(), None);
        assert!(!p.exceeds_budget(LatencyBudget::FRAME));
    }
}
