/// Repeatable profiling workflow.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ProfileTarget {
    Startup,
    FileOpen,
    Keystroke,
    Scroll,
    Resize,
    Render,
    FullSession,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProfileConfig {
    pub(crate) target: ProfileTarget,
    pub(crate) iterations: usize,
    pub(crate) warmup: usize,
    pub(crate) output_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProfileResult {
    pub(crate) target: ProfileTarget,
    pub(crate) samples: Vec<f64>,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) avg: f64,
    pub(crate) p95: f64,
}

impl ProfileConfig {
    pub(crate) fn new(target: ProfileTarget) -> Self {
        Self {
            target,
            iterations: 100,
            warmup: 10,
            output_path: None,
        }
    }
}

pub(crate) fn compute_stats(samples: &[f64], target: ProfileTarget) -> ProfileResult {
    if samples.is_empty() {
        return ProfileResult {
            target,
            samples: vec![],
            min: 0.0,
            max: 0.0,
            avg: 0.0,
            p95: 0.0,
        };
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let avg = sorted.iter().sum::<f64>() / sorted.len() as f64;
    let p95_idx = ((sorted.len() as f64 * 0.95).ceil() as usize).saturating_sub(1).min(sorted.len() - 1);
    let p95 = sorted[p95_idx];
    ProfileResult {
        target,
        samples: samples.to_vec(),
        min,
        max,
        avg,
        p95,
    }
}

pub(crate) fn format_report(results: &[ProfileResult]) -> String {
    let mut out = String::from("Profiling Report\n================\n");
    for r in results {
        out.push_str(&format!(
            "{:?}: min={:.2}ms max={:.2}ms avg={:.2}ms p95={:.2}ms (n={})\n",
            r.target, r.min, r.max, r.avg, r.p95, r.samples.len()
        ));
    }
    out
}

pub(crate) fn meets_budget(result: &ProfileResult, budget_ms: f64) -> bool {
    result.p95 <= budget_ms
}

pub(crate) fn default_budgets() -> Vec<(ProfileTarget, f64)> {
    vec![
        (ProfileTarget::Keystroke, 16.0),
        (ProfileTarget::Render, 16.0),
        (ProfileTarget::Resize, 50.0),
        (ProfileTarget::Scroll, 8.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_stats_basic() {
        let samples = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let result = compute_stats(&samples, ProfileTarget::Keystroke);
        assert!((result.min - 10.0).abs() < f64::EPSILON);
        assert!((result.max - 50.0).abs() < f64::EPSILON);
        assert!((result.avg - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn compute_stats_single() {
        let result = compute_stats(&[42.0], ProfileTarget::Startup);
        assert!((result.min - 42.0).abs() < f64::EPSILON);
        assert!((result.max - 42.0).abs() < f64::EPSILON);
        assert!((result.p95 - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn format_report_output() {
        let result = compute_stats(&[1.0, 2.0, 3.0], ProfileTarget::Render);
        let report = format_report(&[result]);
        assert!(report.contains("Render"));
        assert!(report.contains("Profiling Report"));
    }

    #[test]
    fn meets_budget_pass() {
        let result = compute_stats(&[5.0, 8.0, 10.0, 12.0, 14.0], ProfileTarget::Keystroke);
        assert!(meets_budget(&result, 16.0));
    }

    #[test]
    fn meets_budget_fail() {
        let result = compute_stats(&[10.0, 20.0, 30.0, 40.0, 50.0], ProfileTarget::Scroll);
        assert!(!meets_budget(&result, 8.0));
    }

    #[test]
    fn default_budgets_exist() {
        let budgets = default_budgets();
        assert!(budgets.len() >= 4);
        let keystroke = budgets.iter().find(|(t, _)| *t == ProfileTarget::Keystroke);
        assert!(keystroke.is_some());
    }

    #[test]
    fn config_defaults() {
        let cfg = ProfileConfig::new(ProfileTarget::FileOpen);
        assert_eq!(cfg.iterations, 100);
        assert_eq!(cfg.warmup, 10);
        assert!(cfg.output_path.is_none());
    }
}
