//! Performance measurement types: latency probes, memory snapshots, timing stats.

use std::time::{Duration, Instant};

/// A single latency measurement sample.
#[derive(Debug, Clone)]
pub struct LatencySample {
    pub label: &'static str,
    pub duration: Duration,
    pub timestamp: Instant,
}

/// Accumulated latency statistics for a named metric.
#[derive(Debug, Clone)]
pub struct LatencyStats {
    pub label: &'static str,
    pub count: u64,
    pub total: Duration,
    pub min: Duration,
    pub max: Duration,
}

impl LatencyStats {
    pub fn new(label: &'static str) -> Self {
        Self { label, count: 0, total: Duration::ZERO, min: Duration::MAX, max: Duration::ZERO }
    }

    /// Record a duration sample.
    pub fn record(&mut self, d: Duration) {
        self.count += 1;
        self.total += d;
        if d < self.min { self.min = d; }
        if d > self.max { self.max = d; }
    }

    /// Average latency.
    pub fn avg(&self) -> Duration {
        if self.count == 0 { Duration::ZERO } else { self.total / self.count as u32 }
    }

    /// Check if any sample exceeded a threshold.
    pub fn exceeds(&self, threshold: Duration) -> bool { self.max > threshold }
}

/// A scoped timer that records elapsed time when dropped.
pub struct ScopedTimer { label: &'static str, start: Instant, samples: *mut Vec<LatencySample> }

impl ScopedTimer {
    /// Create a new scoped timer. Safety: `samples` must live longer than the timer.
    pub unsafe fn new(label: &'static str, samples: &mut Vec<LatencySample>) -> Self {
        Self { label, start: Instant::now(), samples: samples as *mut _ }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        unsafe { (*self.samples).push(LatencySample { label: self.label, duration: elapsed, timestamp: self.start }); }
    }
}

/// A simple manual timer (no unsafe).
#[derive(Debug, Clone)]
pub struct ManualTimer { pub label: &'static str, start: Instant }

impl ManualTimer {
    pub fn start(label: &'static str) -> Self { Self { label, start: Instant::now() } }
    pub fn elapsed(&self) -> Duration { self.start.elapsed() }
    pub fn finish(self) -> LatencySample {
        LatencySample { label: self.label, duration: self.start.elapsed(), timestamp: self.start }
    }
}

/// Memory usage snapshot for tracking allocator health.
#[derive(Debug, Clone, Copy)]
pub struct MemorySnapshot {
    pub buffer_bytes: usize,
    pub undo_bytes: usize,
    pub viewport_bytes: usize,
    pub total_buffers: usize,
}

impl MemorySnapshot {
    pub fn total(&self) -> usize { self.buffer_bytes + self.undo_bytes + self.viewport_bytes }
}

/// Latency budget: defines acceptable timing for key operations.
#[derive(Debug, Clone)]
pub struct LatencyBudget {
    pub keystroke_to_render: Duration,
    pub file_open: Duration,
    pub search: Duration,
}

impl Default for LatencyBudget {
    fn default() -> Self { Self {
        keystroke_to_render: Duration::from_millis(16),
        file_open: Duration::from_millis(100),
        search: Duration::from_millis(50),
    } }
}

/// A latency tracker aggregating stats across multiple named metrics.
#[derive(Debug, Clone)]
pub struct LatencyTracker {
    pub stats: Vec<LatencyStats>,
    pub budget: LatencyBudget,
}

impl LatencyTracker {
    pub fn new() -> Self { Self { stats: Vec::new(), budget: LatencyBudget::default() } }

    /// Get or create a stat entry for a label.
    pub fn get_or_create(&mut self, label: &'static str) -> &mut LatencyStats {
        if let Some(pos) = self.stats.iter().position(|s| s.label == label) {
            &mut self.stats[pos]
        } else {
            self.stats.push(LatencyStats::new(label));
            self.stats.last_mut().unwrap()
        }
    }

    /// Record a sample for a metric.
    pub fn record(&mut self, label: &'static str, d: Duration) {
        self.get_or_create(label).record(d);
    }

    /// Get summary for a metric.
    pub fn summary(&self, label: &str) -> Option<&LatencyStats> {
        self.stats.iter().find(|s| s.label == label)
    }

    /// Check if any metric exceeds its budget.
    pub fn has_regressions(&self) -> bool {
        self.stats.iter().any(|s| match s.label {
            "keystroke" => s.exceeds(self.budget.keystroke_to_render),
            "file_open" => s.exceeds(self.budget.file_open),
            "search" => s.exceeds(self.budget.search),
            _ => false,
        })
    }
}

impl Default for LatencyTracker { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latency_stats_empty() {
        let s = LatencyStats::new("test");
        assert_eq!(s.count, 0);
        assert_eq!(s.avg(), Duration::ZERO);
    }

    #[test]
    fn latency_stats_record() {
        let mut s = LatencyStats::new("key");
        s.record(Duration::from_micros(100));
        s.record(Duration::from_micros(200));
        assert_eq!(s.count, 2);
        assert_eq!(s.avg(), Duration::from_micros(150));
        assert_eq!(s.min, Duration::from_micros(100));
        assert_eq!(s.max, Duration::from_micros(200));
    }

    #[test]
    fn manual_timer() {
        let t = ManualTimer::start("test");
        std::thread::sleep(Duration::from_millis(1));
        let sample = t.finish();
        assert!(sample.duration >= Duration::from_micros(500));
    }

    #[test]
    fn memory_snapshot_total() {
        let snap = MemorySnapshot { buffer_bytes: 1000, undo_bytes: 500, viewport_bytes: 200, total_buffers: 3 };
        assert_eq!(snap.total(), 1700);
    }

    #[test]
    fn latency_tracker_record_and_summary() {
        let mut tracker = LatencyTracker::new();
        tracker.record("keystroke", Duration::from_micros(500));
        tracker.record("keystroke", Duration::from_micros(800));
        let s = tracker.summary("keystroke").unwrap();
        assert_eq!(s.count, 2);
    }

    #[test]
    fn latency_budget_and_no_regressions() {
        let b = LatencyBudget::default();
        assert_eq!(b.keystroke_to_render, Duration::from_millis(16));
        let mut tracker = LatencyTracker::new();
        tracker.record("keystroke", Duration::from_micros(100));
        assert!(!tracker.has_regressions());
    }
}
