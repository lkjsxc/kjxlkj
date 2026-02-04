//! Profiling and performance observability.
//!
//! This module provides opt-in instrumentation for performance analysis.
//! Per /docs/spec/technical/profiling.md, instrumentation must be:
//! - opt-in (disabled by default)
//! - low overhead when enabled
//! - zero/near-zero overhead when disabled
//! - deterministic (must not change edit semantics)

use std::time::{Duration, Instant};

/// Feature flag for profiling (compile-time).
/// Enable with: cargo build --features profiling
#[cfg(feature = "profiling")]
const PROFILING_ENABLED: bool = true;

#[cfg(not(feature = "profiling"))]
const PROFILING_ENABLED: bool = false;

/// Runtime profiling configuration.
#[derive(Debug, Clone, Default)]
pub struct ProfilingConfig {
    /// Whether profiling is enabled at runtime.
    pub enabled: bool,
    /// Whether to log metrics to stderr.
    pub log_to_stderr: bool,
}

/// Metrics for a single update cycle.
#[derive(Debug, Clone, Default)]
pub struct CycleMetrics {
    /// Number of input events processed.
    pub input_events: usize,
    /// Number of core state transitions applied.
    pub core_updates: usize,
    /// Time spent producing a snapshot.
    pub snapshot_duration: Duration,
    /// Time spent producing terminal output.
    pub render_duration: Duration,
    /// Number of buffer lines materialized into the snapshot.
    pub materialized_lines: usize,
    /// Number of terminal cells written (if cell-based renderer).
    pub cells_written: Option<usize>,
    /// Dirty region size (width * height of changed area).
    pub dirty_region_size: Option<usize>,
}

impl CycleMetrics {
    /// Create new empty metrics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if snapshot work is viewport-bounded.
    /// Returns true if materialized_lines <= viewport_height + margin.
    pub fn is_viewport_bounded(&self, viewport_height: usize, margin: usize) -> bool {
        self.materialized_lines <= viewport_height + margin
    }
}

/// Profiler for recording performance metrics.
#[derive(Debug)]
pub struct Profiler {
    config: ProfilingConfig,
    current_cycle: CycleMetrics,
    cycle_start: Option<Instant>,
    snapshot_start: Option<Instant>,
    render_start: Option<Instant>,
    /// Accumulated metrics for analysis.
    cycles: Vec<CycleMetrics>,
    /// Maximum cycles to retain.
    max_cycles: usize,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    /// Create a new profiler.
    pub fn new() -> Self {
        Self {
            config: ProfilingConfig::default(),
            current_cycle: CycleMetrics::new(),
            cycle_start: None,
            snapshot_start: None,
            render_start: None,
            cycles: Vec::new(),
            max_cycles: 1000,
        }
    }

    /// Create a profiler with configuration.
    pub fn with_config(config: ProfilingConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Check if profiling is enabled.
    #[inline]
    pub fn is_enabled(&self) -> bool {
        PROFILING_ENABLED && self.config.enabled
    }

    /// Enable profiling at runtime.
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }

    /// Disable profiling at runtime.
    pub fn disable(&mut self) {
        self.config.enabled = false;
    }

    /// Start a new update cycle.
    #[inline]
    pub fn start_cycle(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.current_cycle = CycleMetrics::new();
        self.cycle_start = Some(Instant::now());
    }

    /// Record an input event.
    #[inline]
    pub fn record_input_event(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.current_cycle.input_events += 1;
    }

    /// Record a core state update.
    #[inline]
    pub fn record_core_update(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.current_cycle.core_updates += 1;
    }

    /// Start timing snapshot generation.
    #[inline]
    pub fn start_snapshot(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.snapshot_start = Some(Instant::now());
    }

    /// End timing snapshot generation.
    #[inline]
    pub fn end_snapshot(&mut self, materialized_lines: usize) {
        if !self.is_enabled() {
            return;
        }
        if let Some(start) = self.snapshot_start.take() {
            self.current_cycle.snapshot_duration = start.elapsed();
            self.current_cycle.materialized_lines = materialized_lines;
        }
    }

    /// Start timing render.
    #[inline]
    pub fn start_render(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.render_start = Some(Instant::now());
    }

    /// End timing render.
    #[inline]
    pub fn end_render(&mut self, cells_written: Option<usize>, dirty_region_size: Option<usize>) {
        if !self.is_enabled() {
            return;
        }
        if let Some(start) = self.render_start.take() {
            self.current_cycle.render_duration = start.elapsed();
            self.current_cycle.cells_written = cells_written;
            self.current_cycle.dirty_region_size = dirty_region_size;
        }
    }

    /// End the current cycle and record metrics.
    #[inline]
    pub fn end_cycle(&mut self) {
        if !self.is_enabled() {
            return;
        }

        // Log if configured
        if self.config.log_to_stderr {
            eprintln!(
                "[PERF] inputs={} updates={} snapshot={:?} render={:?} lines={}",
                self.current_cycle.input_events,
                self.current_cycle.core_updates,
                self.current_cycle.snapshot_duration,
                self.current_cycle.render_duration,
                self.current_cycle.materialized_lines,
            );
        }

        // Store cycle
        if self.cycles.len() >= self.max_cycles {
            self.cycles.remove(0);
        }
        self.cycles.push(self.current_cycle.clone());

        self.cycle_start = None;
    }

    /// Get recorded cycles for analysis.
    pub fn cycles(&self) -> &[CycleMetrics] {
        &self.cycles
    }

    /// Clear recorded cycles.
    pub fn clear(&mut self) {
        self.cycles.clear();
    }

    /// Get the last cycle metrics.
    pub fn last_cycle(&self) -> Option<&CycleMetrics> {
        self.cycles.last()
    }

    /// Check if idle (no input events in last N cycles).
    pub fn is_idle(&self, window: usize) -> bool {
        if self.cycles.len() < window {
            return false;
        }
        self.cycles
            .iter()
            .rev()
            .take(window)
            .all(|c| c.input_events == 0)
    }

    /// Check for busy-loop while idle.
    /// Returns true if there are render cycles with no input events.
    pub fn detect_busy_loop(&self, window: usize) -> bool {
        if self.cycles.len() < window {
            return false;
        }
        let idle_with_render = self
            .cycles
            .iter()
            .rev()
            .take(window)
            .filter(|c| c.input_events == 0 && c.render_duration > Duration::ZERO)
            .count();
        // If most idle cycles have render work, it's a busy loop
        idle_with_render > window / 2
    }
}

/// Scoped timer that records duration on drop.
pub struct ScopedTimer<'a> {
    profiler: &'a mut Profiler,
    kind: TimerKind,
}

enum TimerKind {
    Snapshot { lines: usize },
    Render { cells: Option<usize>, dirty: Option<usize> },
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        match &self.kind {
            TimerKind::Snapshot { lines } => {
                self.profiler.end_snapshot(*lines);
            }
            TimerKind::Render { cells, dirty } => {
                self.profiler.end_render(*cells, *dirty);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_disabled_by_default() {
        let profiler = Profiler::new();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn test_profiler_enable_disable() {
        let mut profiler = Profiler::new();
        profiler.enable();
        // Note: is_enabled also checks PROFILING_ENABLED compile-time flag
        // In tests without the feature, this will still be false
        #[cfg(feature = "profiling")]
        assert!(profiler.is_enabled());
        profiler.disable();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn test_cycle_metrics_viewport_bounded() {
        let metrics = CycleMetrics {
            materialized_lines: 24,
            ..Default::default()
        };
        assert!(metrics.is_viewport_bounded(24, 0));
        assert!(metrics.is_viewport_bounded(20, 4));
        assert!(!metrics.is_viewport_bounded(20, 2));
    }

    #[test]
    fn test_profiler_cycle_recording() {
        let config = ProfilingConfig {
            enabled: true,
            log_to_stderr: false,
        };
        let mut profiler = Profiler::with_config(config);

        // Only works if PROFILING_ENABLED
        #[cfg(feature = "profiling")]
        {
            profiler.start_cycle();
            profiler.record_input_event();
            profiler.record_input_event();
            profiler.record_core_update();
            profiler.start_snapshot();
            profiler.end_snapshot(24);
            profiler.start_render();
            profiler.end_render(Some(1920), Some(100));
            profiler.end_cycle();

            let last = profiler.last_cycle().unwrap();
            assert_eq!(last.input_events, 2);
            assert_eq!(last.core_updates, 1);
            assert_eq!(last.materialized_lines, 24);
            assert_eq!(last.cells_written, Some(1920));
        }
    }
}
