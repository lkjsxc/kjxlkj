/// Deterministic latency regression probes.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ProbeKind {
    CursorVisibility,
    ViewportFollow,
    ScrollClamp,
    ResizeCursor,
    InputOrdering,
    BusyLoopDetection,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProbeResult {
    pub(crate) kind: ProbeKind,
    pub(crate) passed: bool,
    pub(crate) message: String,
    pub(crate) elapsed_us: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RegressionSuite {
    pub(crate) probes: Vec<ProbeResult>,
}

pub(crate) fn probe_cursor_visibility(cursor_line: usize, top_line: usize, visible_lines: usize) -> ProbeResult {
    let visible = cursor_line >= top_line && cursor_line < top_line + visible_lines;
    ProbeResult {
        kind: ProbeKind::CursorVisibility,
        passed: visible,
        message: if visible { "Cursor within viewport".into() } else { format!("Cursor at line {} outside viewport [{}, {})", cursor_line, top_line, top_line + visible_lines) },
        elapsed_us: 0,
    }
}

pub(crate) fn probe_viewport_follow(cursor_line: usize, top_line: usize, visible_lines: usize, scrolloff: usize) -> ProbeResult {
    let lower = top_line + scrolloff;
    let upper = if top_line + visible_lines > scrolloff { top_line + visible_lines - scrolloff } else { top_line };
    let ok = cursor_line >= lower && cursor_line < upper;
    ProbeResult {
        kind: ProbeKind::ViewportFollow,
        passed: ok,
        message: if ok { "Scrolloff honored".into() } else { format!("Cursor {} not within scrolloff bounds [{}, {})", cursor_line, lower, upper) },
        elapsed_us: 0,
    }
}

pub(crate) fn probe_scroll_clamp(top_line: usize, total_lines: usize, visible_lines: usize) -> ProbeResult {
    let max_top = if total_lines > visible_lines { total_lines - visible_lines } else { 0 };
    let ok = top_line <= max_top;
    ProbeResult {
        kind: ProbeKind::ScrollClamp,
        passed: ok,
        message: if ok { "Scroll clamped correctly".into() } else { format!("top_line {} exceeds max {}", top_line, max_top) },
        elapsed_us: 0,
    }
}

pub(crate) fn probe_resize_cursor(cursor_line: usize, cursor_col: usize, new_height: usize, new_width: usize) -> ProbeResult {
    let line_ok = cursor_line < new_height;
    let col_ok = cursor_col < new_width;
    let ok = line_ok && col_ok;
    ProbeResult {
        kind: ProbeKind::ResizeCursor,
        passed: ok,
        message: if ok { "Cursor valid after resize".into() } else { format!("Cursor ({},{}) outside {}x{}", cursor_line, cursor_col, new_width, new_height) },
        elapsed_us: 0,
    }
}

pub(crate) fn probe_input_ordering(events: &[u64]) -> ProbeResult {
    let ordered = events.windows(2).all(|w| w[0] <= w[1]);
    ProbeResult {
        kind: ProbeKind::InputOrdering,
        passed: ordered,
        message: if ordered { "Input timestamps monotonic".into() } else { "Input timestamps out of order".into() },
        elapsed_us: 0,
    }
}

pub(crate) fn probe_busy_loop(frame_count: usize, elapsed_ms: u64) -> ProbeResult {
    let fps = if elapsed_ms > 0 { (frame_count as f64 / elapsed_ms as f64) * 1000.0 } else { 0.0 };
    let busy = fps > 120.0;
    ProbeResult {
        kind: ProbeKind::BusyLoopDetection,
        passed: !busy,
        message: if busy { format!("Busy loop detected: {:.0} fps", fps) } else { format!("Normal frame rate: {:.0} fps", fps) },
        elapsed_us: 0,
    }
}

pub(crate) fn run_all_probes() -> RegressionSuite {
    RegressionSuite {
        probes: vec![
            probe_cursor_visibility(5, 0, 24),
            probe_viewport_follow(10, 0, 24, 3),
            probe_scroll_clamp(0, 100, 24),
            probe_resize_cursor(10, 40, 24, 80),
            probe_input_ordering(&[1, 2, 3, 4, 5]),
            probe_busy_loop(60, 1000),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_visible() {
        let r = probe_cursor_visibility(5, 0, 24);
        assert!(r.passed);
    }

    #[test]
    fn cursor_not_visible() {
        let r = probe_cursor_visibility(30, 0, 24);
        assert!(!r.passed);
    }

    #[test]
    fn viewport_follow_ok() {
        let r = probe_viewport_follow(10, 0, 24, 3);
        assert!(r.passed);
    }

    #[test]
    fn scroll_clamp_ok() {
        let r = probe_scroll_clamp(0, 100, 24);
        assert!(r.passed);
    }

    #[test]
    fn resize_cursor() {
        let r = probe_resize_cursor(10, 40, 24, 80);
        assert!(r.passed);
        let r2 = probe_resize_cursor(30, 40, 24, 80);
        assert!(!r2.passed);
    }

    #[test]
    fn input_ordered() {
        let r = probe_input_ordering(&[1, 2, 3]);
        assert!(r.passed);
        let r2 = probe_input_ordering(&[3, 1, 2]);
        assert!(!r2.passed);
    }

    #[test]
    fn busy_loop_detected() {
        let r = probe_busy_loop(500, 1000);
        assert!(!r.passed);
        let r2 = probe_busy_loop(60, 1000);
        assert!(r2.passed);
    }

    #[test]
    fn all_probes() {
        let suite = run_all_probes();
        assert_eq!(suite.probes.len(), 6);
        assert!(suite.probes.iter().all(|p| p.passed));
    }
}
