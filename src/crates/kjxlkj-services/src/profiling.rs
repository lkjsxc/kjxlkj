//! Profiling instrumentation — lightweight spans and counters for performance measurement.

use std::collections::HashMap;

/// A profiling span capturing duration of an operation.
#[derive(Debug, Clone)]
pub struct ProfilingSpan {
    pub name: String,
    pub start_us: u64,
    pub end_us: Option<u64>,
}

impl ProfilingSpan {
    pub fn start(name: &str, now_us: u64) -> Self {
        Self { name: name.into(), start_us: now_us, end_us: None }
    }
    pub fn finish(&mut self, now_us: u64) { self.end_us = Some(now_us); }
    pub fn duration_us(&self) -> Option<u64> { self.end_us.map(|e| e.saturating_sub(self.start_us)) }
}

/// A named counter for tracking occurrences.
#[derive(Debug, Clone, Default)]
pub struct Counter { pub name: String, pub count: u64 }

impl Counter {
    pub fn new(name: &str) -> Self { Self { name: name.into(), count: 0 } }
    pub fn increment(&mut self) { self.count += 1; }
    pub fn add(&mut self, n: u64) { self.count += n; }
}

/// A simple profiling context for collecting spans and counters.
#[derive(Debug, Clone)]
pub struct Profiler {
    enabled: bool,
    spans: Vec<ProfilingSpan>,
    counters: HashMap<String, Counter>,
    now_us: u64,
}

impl Profiler {
    pub fn new() -> Self { Self { enabled: false, spans: Vec::new(), counters: HashMap::new(), now_us: 0 } }
    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }
    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn set_time(&mut self, us: u64) { self.now_us = us; }

    /// Begin a new span.
    pub fn begin_span(&mut self, name: &str) -> usize {
        if !self.enabled { return 0; }
        let idx = self.spans.len();
        self.spans.push(ProfilingSpan::start(name, self.now_us));
        idx
    }

    /// End a span by index.
    pub fn end_span(&mut self, idx: usize) {
        if !self.enabled { return; }
        if let Some(span) = self.spans.get_mut(idx) { span.finish(self.now_us); }
    }

    /// Increment a named counter.
    pub fn count(&mut self, name: &str) {
        if !self.enabled { return; }
        self.counters.entry(name.into()).or_insert_with(|| Counter::new(name)).increment();
    }

    /// Get completed spans.
    pub fn completed_spans(&self) -> Vec<&ProfilingSpan> {
        self.spans.iter().filter(|s| s.end_us.is_some()).collect()
    }

    /// Generate a summary report.
    pub fn report(&self) -> String {
        let mut lines = Vec::new();
        let mut by_name: HashMap<&str, Vec<u64>> = HashMap::new();
        for span in &self.spans {
            if let Some(dur) = span.duration_us() {
                by_name.entry(&span.name).or_default().push(dur);
            }
        }
        let mut names: Vec<_> = by_name.keys().copied().collect();
        names.sort();
        for name in names {
            let durations = &by_name[name];
            let total: u64 = durations.iter().sum();
            let avg = total / durations.len() as u64;
            lines.push(format!("{}: count={} total={}us avg={}us", name, durations.len(), total, avg));
        }
        for (name, counter) in &self.counters {
            lines.push(format!("counter {}: {}", name, counter.count));
        }
        if lines.is_empty() { "no profiling data".into() } else { lines.join("\n") }
    }

    pub fn reset(&mut self) { self.spans.clear(); self.counters.clear(); }
}

impl Default for Profiler { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_noop() {
        let mut p = Profiler::new();
        p.begin_span("test");
        p.count("x");
        assert!(p.completed_spans().is_empty());
        assert_eq!(p.report(), "no profiling data");
    }

    #[test]
    fn span_timing() {
        let mut p = Profiler::new();
        p.enable();
        p.set_time(1000);
        let idx = p.begin_span("render");
        p.set_time(1500);
        p.end_span(idx);
        let spans = p.completed_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].duration_us(), Some(500));
    }

    #[test]
    fn counters() {
        let mut p = Profiler::new();
        p.enable();
        p.count("keypress");
        p.count("keypress");
        p.count("keypress");
        assert!(p.report().contains("keypress: 3"));
    }

    #[test]
    fn report_aggregation() {
        let mut p = Profiler::new();
        p.enable();
        p.set_time(0); p.begin_span("a"); p.set_time(100); p.end_span(0);
        p.set_time(200); p.begin_span("a"); p.set_time(400); p.end_span(1);
        let report = p.report();
        assert!(report.contains("count=2"));
        assert!(report.contains("total=300us"));
    }

    #[test]
    fn reset_clears() {
        let mut p = Profiler::new();
        p.enable();
        p.count("x");
        p.reset();
        assert_eq!(p.report(), "no profiling data");
    }

    #[test]
    fn enable_disable_toggle() {
        let mut p = Profiler::new();
        assert!(!p.is_enabled());
        p.enable();
        assert!(p.is_enabled());
        p.disable();
        assert!(!p.is_enabled());
    }
}
