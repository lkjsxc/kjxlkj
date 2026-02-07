//! Profiling workflow types and utilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Target of a profiling session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfileTarget {
    Startup,
    FileOpen,
    Keystroke,
    Scroll,
    Resize,
    Render,
    FullSession,
}

/// Profiling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub target: ProfileTarget,
    pub iterations: u32,
    pub warmup: u32,
    pub output_path: Option<String>,
}

/// Result of a profiling run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResult {
    pub samples: Vec<u64>,
    pub min: u64,
    pub max: u64,
    pub avg: u64,
    pub p95: u64,
}

/// Compute statistics from raw sample data (microseconds).
pub fn compute_stats(mut samples: Vec<u64>) -> ProfileResult {
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
    ProfileResult {
        samples,
        min,
        max,
        avg,
        p95,
    }
}

/// Format a profiling report.
pub fn format_report(results: &[(ProfileTarget, ProfileResult)]) -> String {
    let mut out = String::from("Profiling Report\n");
    out.push_str(&format!(
        "{:<16} {:>10} {:>10} {:>10} {:>10}\n",
        "Target", "Min(µs)", "Max(µs)", "Avg(µs)", "P95(µs)"
    ));
    out.push_str(&"-".repeat(60));
    out.push('\n');
    for (target, r) in results {
        out.push_str(&format!(
            "{:<16} {:>10} {:>10} {:>10} {:>10}\n",
            format!("{:?}", target),
            r.min,
            r.max,
            r.avg,
            r.p95
        ));
    }
    out
}

/// Check if a result meets the budget (in microseconds).
pub fn meets_budget(result: &ProfileResult, budget_us: u64) -> bool {
    result.p95 <= budget_us
}

/// Default latency budgets in microseconds per target.
pub fn default_budgets() -> HashMap<ProfileTarget, u64> {
    let mut m = HashMap::new();
    m.insert(ProfileTarget::Startup, 500_000); // 500ms
    m.insert(ProfileTarget::FileOpen, 50_000); // 50ms
    m.insert(ProfileTarget::Keystroke, 8_000); // 8ms
    m.insert(ProfileTarget::Scroll, 16_000); // 16ms
    m.insert(ProfileTarget::Resize, 16_000); // 16ms
    m.insert(ProfileTarget::Render, 16_000); // 16ms
    m.insert(ProfileTarget::FullSession, 2_000_000); // 2s
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_stats_basic() {
        let r = compute_stats(vec![100, 200, 300, 400, 500]);
        assert_eq!(r.min, 100);
        assert_eq!(r.max, 500);
        assert_eq!(r.avg, 300);
    }

    #[test]
    fn meets_budget_pass() {
        let r = compute_stats(vec![100, 200, 300]);
        assert!(meets_budget(&r, 1000));
    }

    #[test]
    fn meets_budget_fail() {
        let r = compute_stats(vec![100, 200, 5000]);
        assert!(!meets_budget(&r, 1000));
    }

    #[test]
    fn default_budgets_coverage() {
        let b = default_budgets();
        assert_eq!(b.len(), 7);
        assert_eq!(b[&ProfileTarget::Keystroke], 8_000);
    }

    #[test]
    fn format_report_not_empty() {
        let r = compute_stats(vec![100, 200]);
        let report = format_report(&[(ProfileTarget::Keystroke, r)]);
        assert!(report.contains("Keystroke"));
    }
}
