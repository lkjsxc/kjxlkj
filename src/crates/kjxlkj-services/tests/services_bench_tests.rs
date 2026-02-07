use kjxlkj_services::BenchmarkKind;
use kjxlkj_services::benchmark_suite::{budget_for, compute_benchmark_result, default_suite};
use kjxlkj_services::profiling_workflow::{compute_stats, default_budgets, meets_budget};
use kjxlkj_services::ProfileTarget;

// --- Benchmark suite ---

#[test]
fn default_suite_has_6_entries() {
    assert_eq!(default_suite().len(), 6);
}

#[test]
fn budget_keystroke_is_8() {
    assert_eq!(budget_for(BenchmarkKind::Keystroke), 8);
}

#[test]
fn budget_file_open_is_50() {
    assert_eq!(budget_for(BenchmarkKind::FileOpen), 50);
}

#[test]
fn compute_result_stats() {
    let r = compute_benchmark_result(BenchmarkKind::EditBurst, vec![10, 5, 20, 15, 8]);
    assert_eq!(r.min, 5);
    assert_eq!(r.max, 20);
    assert!(r.avg > 0);
}

// --- Profiling workflow ---

#[test]
fn profiling_compute_stats_sorted() {
    let r = compute_stats(vec![300, 100, 200]);
    assert_eq!(r.min, 100);
    assert_eq!(r.max, 300);
    assert_eq!(r.avg, 200);
}

#[test]
fn profiling_meets_budget_pass() {
    let r = compute_stats(vec![100, 200, 300]);
    assert!(meets_budget(&r, 1000));
}

#[test]
fn profiling_meets_budget_fail() {
    let r = compute_stats(vec![100, 200, 5000]);
    assert!(!meets_budget(&r, 1000));
}

#[test]
fn profiling_default_budgets_count() {
    assert_eq!(default_budgets().len(), 7);
}

#[test]
fn profiling_keystroke_budget_value() {
    assert_eq!(default_budgets()[&ProfileTarget::Keystroke], 8_000);
}
