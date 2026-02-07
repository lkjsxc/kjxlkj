//! Golden snapshot testing utilities for rendered output.

use serde::{Deserialize, Serialize};

/// Wrapping mode for snapshot generation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotMode {
    NoWrap,
    SoftWrap,
    HardWrap,
}

/// Configuration for generating a golden snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    pub width: usize,
    pub height: usize,
    pub mode: SnapshotMode,
    pub show_line_numbers: bool,
}

/// Render lines into a snapshot according to `config`.
pub fn render_snapshot(lines: &[String], config: &SnapshotConfig) -> Vec<String> {
    let num_w = if config.show_line_numbers { 4 } else { 0 };
    let text_w = config.width.saturating_sub(num_w);
    let mut output = Vec::new();
    let mut line_idx = 0usize;

    while output.len() < config.height && line_idx < lines.len() {
        let raw = &lines[line_idx];
        match config.mode {
            SnapshotMode::NoWrap => {
                output.push(format_row(
                    line_idx,
                    raw,
                    num_w,
                    text_w,
                    config.show_line_numbers,
                ));
            }
            SnapshotMode::SoftWrap | SnapshotMode::HardWrap => {
                let chunks = wrap(raw, text_w);
                for (i, chunk) in chunks.iter().enumerate() {
                    if output.len() >= config.height {
                        break;
                    }
                    let num = if i == 0 { Some(line_idx) } else { None };
                    let prefix = match num {
                        Some(n) if config.show_line_numbers => format!("{:>3} ", n + 1),
                        _ if config.show_line_numbers => "    ".to_string(),
                        _ => String::new(),
                    };
                    output.push(pad(&format!("{prefix}{chunk}"), config.width));
                }
            }
        }
        line_idx += 1;
    }
    // Fill remaining rows with tildes
    while output.len() < config.height {
        let tilde = if config.show_line_numbers {
            "  ~ ".to_string()
        } else {
            "~".to_string()
        };
        output.push(pad(&tilde, config.width));
    }
    output
}

fn format_row(idx: usize, raw: &str, num_w: usize, text_w: usize, show_nums: bool) -> String {
    let prefix = if show_nums {
        format!("{:>3} ", idx + 1)
    } else {
        String::new()
    };
    let visible: String = raw.chars().take(text_w).collect();
    pad(&format!("{prefix}{visible}"), num_w + text_w)
}

fn wrap(s: &str, w: usize) -> Vec<String> {
    if w == 0 {
        return vec![s.to_string()];
    }
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return vec![String::new()];
    }
    chars.chunks(w).map(|c| c.iter().collect()).collect()
}

fn pad(s: &str, w: usize) -> String {
    if s.len() >= w {
        s[..w].to_string()
    } else {
        let mut out = s.to_string();
        out.extend(std::iter::repeat_n(' ', w - out.len()));
        out
    }
}

/// Compare expected vs actual snapshots, returning diff lines.
pub fn compare_snapshot(expected: &[String], actual: &[String]) -> Vec<String> {
    let max = expected.len().max(actual.len());
    let mut diffs = Vec::new();
    for i in 0..max {
        let e = expected.get(i).map(|s| s.as_str()).unwrap_or("<missing>");
        let a = actual.get(i).map(|s| s.as_str()).unwrap_or("<missing>");
        if e != a {
            diffs.push(format!("line {i}: expected={e:?} actual={a:?}"));
        }
    }
    diffs
}

/// Build a no-wrap test case. Returns config and expected output.
pub fn build_nowrap_test(lines: &[String], width: usize) -> (SnapshotConfig, Vec<String>) {
    let cfg = SnapshotConfig {
        width,
        height: lines.len().max(1),
        mode: SnapshotMode::NoWrap,
        show_line_numbers: false,
    };
    let expected = render_snapshot(lines, &cfg);
    (cfg, expected)
}

/// Build a soft-wrap test case.
pub fn build_wrap_test(lines: &[String], width: usize) -> (SnapshotConfig, Vec<String>) {
    let h = lines.len().max(1) * 2;
    let cfg = SnapshotConfig {
        width,
        height: h,
        mode: SnapshotMode::SoftWrap,
        show_line_numbers: false,
    };
    let expected = render_snapshot(lines, &cfg);
    (cfg, expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_wrap_basic() {
        let lines = vec!["hello".into(), "world".into()];
        let cfg = SnapshotConfig {
            width: 10,
            height: 3,
            mode: SnapshotMode::NoWrap,
            show_line_numbers: false,
        };
        let out = render_snapshot(&lines, &cfg);
        assert_eq!(out.len(), 3);
        assert!(out[0].starts_with("hello"));
        assert!(out[2].starts_with("~"));
    }

    #[test]
    fn with_line_numbers() {
        let lines = vec!["abc".into()];
        let cfg = SnapshotConfig {
            width: 12,
            height: 2,
            mode: SnapshotMode::NoWrap,
            show_line_numbers: true,
        };
        let out = render_snapshot(&lines, &cfg);
        assert!(out[0].starts_with("  1 "));
    }

    #[test]
    fn compare_identical() {
        let a = vec!["line1".into()];
        assert!(compare_snapshot(&a, &a).is_empty());
    }

    #[test]
    fn compare_diff() {
        let a = vec!["aaa".into()];
        let b = vec!["bbb".into()];
        let d = compare_snapshot(&a, &b);
        assert_eq!(d.len(), 1);
    }

    #[test]
    fn build_helpers() {
        let lines = vec!["test".into()];
        let (cfg, _) = build_nowrap_test(&lines, 10);
        assert_eq!(cfg.mode, SnapshotMode::NoWrap);
        let (cfg2, _) = build_wrap_test(&lines, 10);
        assert_eq!(cfg2.mode, SnapshotMode::SoftWrap);
    }
}
