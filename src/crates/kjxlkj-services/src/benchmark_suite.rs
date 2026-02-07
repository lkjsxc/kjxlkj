//! Benchmark suite types and utilities.

use serde::{Deserialize, Serialize};

/// Kind of benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BenchmarkKind {
    FileOpen,
    Keystroke,
    ScrollBurst,
    ResizeStorm,
    SnapshotRender,
    EditBurst,
}

/// Configuration for a single benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub kind: BenchmarkKind,
    pub iterations: u32,
    pub warmup: u32,
}

/// Result of a benchmark run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub kind: BenchmarkKind,
    pub samples: Vec<u64>,
    pub min: u64,
    pub max: u64,
    pub avg: u64,
    pub p95: u64,
}

/// Default benchmark suite.
pub fn default_suite() -> Vec<BenchmarkConfig> {
    vec![
        BenchmarkConfig { kind: BenchmarkKind::FileOpen, iterations: 100, warmup: 5 },
        BenchmarkConfig { kind: BenchmarkKind::Keystroke, iterations: 1000, warmup: 10 },
        BenchmarkConfig { kind: BenchmarkKind::ScrollBurst, iterations: 200, warmup: 5 },
        BenchmarkConfig { kind: BenchmarkKind::ResizeStorm, iterations: 100, warmup: 5 },
        BenchmarkConfig { kind: BenchmarkKind::SnapshotRender, iterations: 100, warmup: 5 },
        BenchmarkConfig { kind: BenchmarkKind::EditBurst, iterations: 500, warmup: 10 },
    ]
}

/// Budget in milliseconds for a given benchmark kind.
pub fn budget_for(kind: BenchmarkKind) -> u64 {
    match kind {
        BenchmarkKind::FileOpen => 50,
        BenchmarkKind::Keystroke => 8,
        BenchmarkKind::ScrollBurst => 16,
        BenchmarkKind::ResizeStorm => 16,
        BenchmarkKind::SnapshotRender => 16,
        BenchmarkKind::EditBurst => 8,
    }
}

/// Compute benchmark result from raw samples.
pub fn compute_benchmark_result(kind: BenchmarkKind, mut samples: Vec<u64>) -> BenchmarkResult {
    samples.sort();
    let min = samples.first().copied().unwrap_or(0);
    let max = samples.last().copied().unwrap_or(0);
    let avg = if samples.is_empty() {
        0
    } else {
        samples.iter().sum::<u64>() / samples.len() as u64
    };
    let p95 = if samples.is_empty() {
        0
    } else {
        let idx = ((samples.len() as f64) * 0.95).ceil() as usize;
        samples[idx.min(samples.len() - 1)]
    };
    BenchmarkResult { kind, samples, min, max, avg, p95 }
}

/// Format a human-readable benchmark report.
pub fn format_benchmark_report(results: &[BenchmarkResult]) -> String {
    let mut out = String::from("Benchmark Report\n");
    out.push_str(&format!("{:<20} {:>8} {:>8} {:>8} {:>8} {:>8}\n",
        "Kind", "Min(ms)", "Max(ms)", "Avg(ms)", "P95(ms)", "Budget"));
    out.push_str(&"-".repeat(72));
    out.push('\n');
    for r in results {
        let budget = budget_for(r.kind);
        let status = if r.p95 <= budget { "OK" } else { "OVER" };
        out.push_str(&format!(
            "{:<20} {:>8} {:>8} {:>8} {:>8} {:>5}ms {}\n",
            format!("{:?}", r.kind), r.min, r.max, r.avg, r.p95, budget, status
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_suite_count() {
        assert_eq!(default_suite().len(), 6);
    }

    #[test]
    fn compute_result() {
        let r = compute_benchmark_result(BenchmarkKind::Keystroke, vec![5, 3, 8, 2, 10]);
        assert_eq!(r.min, 2);
        assert_eq!(r.max, 10);
        assert!(r.avg > 0);
    }

    #[test]
    fn budget_values() {
        assert_eq!(budget_for(BenchmarkKind::Keystroke), 8);
        assert_eq!(budget_for(BenchmarkKind::FileOpen), 50);
    }

    #[test]
    fn format_report_not_empty() {
        let r = compute_benchmark_result(BenchmarkKind::FileOpen, vec![10, 20, 30]);
        let report = format_benchmark_report(&[r]);
        assert!(report.contains("FileOpen"));
    }
}
