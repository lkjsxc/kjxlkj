use kjxlkj_render::compute_render_diagnostics;

#[test]
fn ascii_wrap_bounds_hold_for_long_line() {
    let line = "a".repeat(10_000);
    let diag = compute_render_diagnostics(&line, 0, 80, 40);
    assert!(diag.bounds_ok);
}

#[test]
fn cjk_wrap_bounds_hold_for_long_line() {
    let line = "漢".repeat(10_000);
    let diag = compute_render_diagnostics(&line, 0, 80, 40);
    assert!(diag.bounds_ok);
}

#[test]
fn wrap_signature_is_deterministic() {
    let line = "abc漢字".repeat(100);
    let a = compute_render_diagnostics(&line, 5, 22, 12);
    let b = compute_render_diagnostics(&line, 5, 22, 12);
    assert_eq!(a.wrap_signature, b.wrap_signature);
}

#[test]
fn cursor_never_targets_continuation_for_wide_char_owner() {
    let line = "x漢y";
    let diag = compute_render_diagnostics(line, 1, 4, 4);
    assert!(!diag.cursor_on_continuation);
    assert!(diag.cursor_visible);
}

#[test]
fn cursor_span_reports_wide_grapheme_as_two_cells() {
    let line = "a漢b";
    let diag = compute_render_diagnostics(line, 1, 10, 3);
    assert!(diag.cursor_visible);
    assert!(!diag.cursor_on_continuation);
    assert_eq!(diag.cursor_span, 2);
}

#[test]
fn cursor_wrap_boundary_keeps_wide_cursor_visible_and_atomic() {
    let line = "a漢b";
    let diag = compute_render_diagnostics(line, 1, 2, 4);
    assert!(diag.cursor_visible);
    assert!(!diag.cursor_on_continuation);
    assert_eq!(diag.cursor_span, 2);
}
