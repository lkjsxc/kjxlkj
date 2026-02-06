/// Benchmark suite for measuring editor performance.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BenchmarkKind {
    FileOpen,
    Keystroke,
    ScrollBurst,
    ResizeStorm,
    SnapshotRender,
    EditBurst,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BenchmarkConfig {
    pub(crate) kind: BenchmarkKind,
    pub(crate) iterations: usize,
    pub(crate) file_lines: usize,
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BenchmarkResult {
    pub(crate) kind: BenchmarkKind,
    pub(crate) iterations: usize,
    pub(crate) total_ms: f64,
    pub(crate) avg_ms: f64,
    pub(crate) p95_ms: f64,
    pub(crate) passed: bool,
}

pub(crate) fn default_suite() -> Vec<BenchmarkConfig> {
    vec![
        BenchmarkConfig { kind: BenchmarkKind::FileOpen, iterations: 100, file_lines: 10_000, description: "Open 10k-line file".into() },
        BenchmarkConfig { kind: BenchmarkKind::Keystroke, iterations: 200, file_lines: 1_000, description: "200 keystroke inserts".into() },
        BenchmarkConfig { kind: BenchmarkKind::ScrollBurst, iterations: 200, file_lines: 10_000, description: "Scroll 200 lines".into() },
        BenchmarkConfig { kind: BenchmarkKind::ResizeStorm, iterations: 100, file_lines: 1_000, description: "100 resize events".into() },
        BenchmarkConfig { kind: BenchmarkKind::SnapshotRender, iterations: 100, file_lines: 1_000, description: "100 snapshot renders".into() },
        BenchmarkConfig { kind: BenchmarkKind::EditBurst, iterations: 500, file_lines: 5_000, description: "500 rapid edits".into() },
    ]
}

pub(crate) fn compute_benchmark_result(kind: BenchmarkKind, samples: &[f64]) -> BenchmarkResult {
    if samples.is_empty() {
        return BenchmarkResult { kind, iterations: 0, total_ms: 0.0, avg_ms: 0.0, p95_ms: 0.0, passed: true };
    }
    let total_ms: f64 = samples.iter().sum();
    let avg_ms = total_ms / samples.len() as f64;
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p95_idx = ((samples.len() as f64) * 0.95).ceil() as usize;
    let p95_ms = sorted[p95_idx.min(sorted.len() - 1)];
    let budget = budget_for(&kind);
    BenchmarkResult {
        kind,
        iterations: samples.len(),
        total_ms,
        avg_ms,
        p95_ms,
        passed: p95_ms <= budget,
    }
}

pub(crate) fn budget_for(kind: &BenchmarkKind) -> f64 {
    match kind {
        BenchmarkKind::FileOpen => 500.0,
        BenchmarkKind::Keystroke => 16.0,
        BenchmarkKind::ScrollBurst => 8.0,
        BenchmarkKind::ResizeStorm => 50.0,
        BenchmarkKind::SnapshotRender => 16.0,
        BenchmarkKind::EditBurst => 16.0,
    }
}

pub(crate) fn format_benchmark_report(results: &[BenchmarkResult]) -> String {
    let mut out = String::from("Benchmark Report\n================\n");
    for r in results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        out.push_str(&format!(
            "[{}] {:?}: avg={:.2}ms p95={:.2}ms ({} iters)\n",
            status, r.kind, r.avg_ms, r.p95_ms, r.iterations
        ));
    }
    out
}

pub(crate) fn all_passed(results: &[BenchmarkResult]) -> bool {
    results.iter().all(|r| r.passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_suite_count() {
        assert_eq!(default_suite().len(), 6);
    }

    #[test]
    fn compute_result_basic() {
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let r = compute_benchmark_result(BenchmarkKind::Keystroke, &samples);
        assert_eq!(r.iterations, 5);
        assert!((r.avg_ms - 3.0).abs() < 0.01);
        assert!(r.passed); // p95 = 5.0 <= 16.0
    }

    #[test]
    fn budget_values() {
        assert_eq!(budget_for(&BenchmarkKind::Keystroke), 16.0);
        assert_eq!(budget_for(&BenchmarkKind::ScrollBurst), 8.0);
        assert_eq!(budget_for(&BenchmarkKind::FileOpen), 500.0);
        assert_eq!(budget_for(&BenchmarkKind::ResizeStorm), 50.0);
        assert_eq!(budget_for(&BenchmarkKind::SnapshotRender), 16.0);
        assert_eq!(budget_for(&BenchmarkKind::EditBurst), 16.0);
    }

    #[test]
    fn format_report_output() {
        let r = compute_benchmark_result(BenchmarkKind::FileOpen, &[10.0, 20.0]);
        let report = format_benchmark_report(&[r]);
        assert!(report.contains("PASS"));
        assert!(report.contains("FileOpen"));
    }

    #[test]
    fn all_passed_true() {
        let r1 = compute_benchmark_result(BenchmarkKind::Keystroke, &[1.0, 2.0]);
        let r2 = compute_benchmark_result(BenchmarkKind::FileOpen, &[10.0]);
        assert!(all_passed(&[r1, r2]));
    }

    #[test]
    fn all_passed_false() {
        let r = compute_benchmark_result(BenchmarkKind::ScrollBurst, &[100.0, 200.0]);
        assert!(!all_passed(&[r]));
    }

    #[test]
    fn config_descriptions() {
        for cfg in default_suite() {
            assert!(!cfg.description.is_empty());
            assert!(cfg.iterations > 0);
        }
    }
}
