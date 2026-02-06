/// Golden UI snapshot testing for viewport rendering.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SnapshotMode {
    NoWrap,
    SoftWrap,
    HardWrap,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SnapshotConfig {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) mode: SnapshotMode,
    pub(crate) show_line_numbers: bool,
}

impl SnapshotConfig {
    pub(crate) fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            mode: SnapshotMode::NoWrap,
            show_line_numbers: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GoldenSnapshot {
    pub(crate) name: String,
    pub(crate) config: SnapshotConfig,
    pub(crate) input_lines: Vec<String>,
    pub(crate) expected_output: Vec<String>,
}

pub(crate) fn render_snapshot(config: &SnapshotConfig, lines: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    for line in lines {
        match config.mode {
            SnapshotMode::NoWrap => {
                let truncated: String = line.chars().take(config.width).collect();
                output.push(truncated);
            }
            SnapshotMode::SoftWrap => {
                let chars: Vec<char> = line.chars().collect();
                if chars.is_empty() {
                    output.push(String::new());
                } else {
                    for chunk in chars.chunks(config.width) {
                        output.push(chunk.iter().collect());
                    }
                }
            }
            SnapshotMode::HardWrap => {
                let truncated: String = line.chars().take(config.width).collect();
                output.push(truncated);
            }
        }
        if output.len() >= config.height {
            output.truncate(config.height);
            break;
        }
    }
    output
}

pub(crate) fn compare_snapshot(expected: &[String], actual: &[String]) -> Vec<String> {
    let mut diffs = Vec::new();
    let max_len = expected.len().max(actual.len());
    for i in 0..max_len {
        let exp = expected.get(i).map(|s| s.as_str()).unwrap_or("<missing>");
        let act = actual.get(i).map(|s| s.as_str()).unwrap_or("<missing>");
        if exp != act {
            diffs.push(format!("Line {}: expected {:?}, got {:?}", i + 1, exp, act));
        }
    }
    diffs
}

pub(crate) fn build_nowrap_test(content: &str, width: usize) -> GoldenSnapshot {
    let config = SnapshotConfig { width, height: 24, mode: SnapshotMode::NoWrap, show_line_numbers: false };
    let input_lines: Vec<String> = content.lines().map(String::from).collect();
    let expected_output = render_snapshot(&config, &input_lines);
    GoldenSnapshot { name: "nowrap_test".into(), config, input_lines, expected_output }
}

pub(crate) fn build_wrap_test(content: &str, width: usize) -> GoldenSnapshot {
    let config = SnapshotConfig { width, height: 24, mode: SnapshotMode::SoftWrap, show_line_numbers: false };
    let input_lines: Vec<String> = content.lines().map(String::from).collect();
    let expected_output = render_snapshot(&config, &input_lines);
    GoldenSnapshot { name: "wrap_test".into(), config, input_lines, expected_output }
}

pub(crate) fn format_diff(diffs: &[String]) -> String {
    if diffs.is_empty() {
        return "No differences found.".to_string();
    }
    let mut out = format!("{} difference(s):\n", diffs.len());
    for d in diffs {
        out.push_str(&format!("  {}\n", d));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_nowrap_short() {
        let cfg = SnapshotConfig::default();
        let lines = vec!["hello".to_string()];
        let out = render_snapshot(&cfg, &lines);
        assert_eq!(out, vec!["hello"]);
    }

    #[test]
    fn render_nowrap_truncate() {
        let cfg = SnapshotConfig { width: 5, ..SnapshotConfig::default() };
        let lines = vec!["hello world".to_string()];
        let out = render_snapshot(&cfg, &lines);
        assert_eq!(out, vec!["hello"]);
    }

    #[test]
    fn render_softwrap() {
        let cfg = SnapshotConfig { width: 3, mode: SnapshotMode::SoftWrap, ..SnapshotConfig::default() };
        let lines = vec!["abcdef".to_string()];
        let out = render_snapshot(&cfg, &lines);
        assert_eq!(out, vec!["abc", "def"]);
    }

    #[test]
    fn compare_match() {
        let a = vec!["hello".to_string()];
        assert!(compare_snapshot(&a, &a).is_empty());
    }

    #[test]
    fn compare_mismatch() {
        let a = vec!["hello".to_string()];
        let b = vec!["world".to_string()];
        assert_eq!(compare_snapshot(&a, &b).len(), 1);
    }

    #[test]
    fn build_nowrap() {
        let snap = build_nowrap_test("hello", 80);
        assert_eq!(snap.config.mode, SnapshotMode::NoWrap);
        assert_eq!(snap.expected_output, vec!["hello"]);
    }

    #[test]
    fn build_wrap() {
        let snap = build_wrap_test("abcdef", 3);
        assert_eq!(snap.config.mode, SnapshotMode::SoftWrap);
        assert_eq!(snap.expected_output, vec!["abc", "def"]);
    }

    #[test]
    fn format_diff_test() {
        let diffs = vec!["Line 1: expected \"a\", got \"b\"".to_string()];
        let out = format_diff(&diffs);
        assert!(out.contains("1 difference"));
    }
}
