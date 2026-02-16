/// Metrics endpoint per /docs/spec/technical/performance.md (IMP-OPS-02)
///
/// GET /api/metrics — exposes request/latency/pool telemetry.
/// In-process counters; no external dependency needed.
use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global request metrics counters.
#[derive(Debug)]
pub struct Metrics {
    pub total_requests: AtomicU64,
    pub total_errors_4xx: AtomicU64,
    pub total_errors_5xx: AtomicU64,
    /// Cumulative latency in microseconds (for computing average)
    pub cumulative_latency_us: AtomicU64,
}

impl Metrics {
    /// Create a new zeroed metrics instance.
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            total_errors_4xx: AtomicU64::new(0),
            total_errors_5xx: AtomicU64::new(0),
            cumulative_latency_us: AtomicU64::new(0),
        }
    }

    /// Record a completed request.
    pub fn record(&self, status: u16, latency_us: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.cumulative_latency_us
            .fetch_add(latency_us, Ordering::Relaxed);
        if (400..500).contains(&status) {
            self.total_errors_4xx.fetch_add(1, Ordering::Relaxed);
        } else if status >= 500 {
            self.total_errors_5xx.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Snapshot current values.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let total = self.total_requests.load(Ordering::Relaxed);
        let cum_us = self.cumulative_latency_us.load(Ordering::Relaxed);
        MetricsSnapshot {
            total_requests: total,
            total_errors_4xx: self.total_errors_4xx.load(Ordering::Relaxed),
            total_errors_5xx: self.total_errors_5xx.load(Ordering::Relaxed),
            avg_latency_us: if total > 0 { cum_us / total } else { 0 },
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable snapshot of metrics.
#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub total_errors_4xx: u64,
    pub total_errors_5xx: u64,
    pub avg_latency_us: u64,
}

/// GET /api/metrics handler
/// Per /docs/spec/technical/performance.md: expose metrics.
pub async fn metrics_handler(
    State(state): State<crate::state::AppState>,
) -> impl IntoResponse {
    Json(state.metrics.snapshot())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_snapshot() {
        let m = Metrics::new();
        m.record(200, 1000);
        m.record(404, 2000);
        m.record(500, 3000);
        let snap = m.snapshot();
        assert_eq!(snap.total_requests, 3);
        assert_eq!(snap.total_errors_4xx, 1);
        assert_eq!(snap.total_errors_5xx, 1);
        assert_eq!(snap.avg_latency_us, 2000);
    }

    #[test]
    fn empty_metrics_zero() {
        let m = Metrics::new();
        let snap = m.snapshot();
        assert_eq!(snap.total_requests, 0);
        assert_eq!(snap.avg_latency_us, 0);
    }
}
