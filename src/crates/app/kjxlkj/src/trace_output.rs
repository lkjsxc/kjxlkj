use std::io::{self, Write};

use kjxlkj_core_state::{ApplyResult, EditorState};
use kjxlkj_render::RenderDiagnostics;

pub fn emit_trace<W: Write>(
    stdout: &mut W,
    seq: u64,
    result: &ApplyResult,
    state: &EditorState,
    render: RenderDiagnostics,
    normalized_key: &str,
) -> io::Result<()> {
    let excerpt = frame_excerpt(state.line(), 80);
    let layout_summary = state.window_session_dump();
    writeln!(
        stdout,
        "TRACE event_seq={} mode_before={:?} focused_window_id={} focused_window_type={} normalized_key={} resolved_action={} cursor_before={} cursor_after={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} layout_summary={} line_len={} frame_excerpt={}",
        seq,
        result.mode_before,
        state.focused_window_id(),
        state.focused_window_kind(),
        normalized_key,
        result.resolved_action,
        result.cursor_before,
        result.cursor_after,
        state.window_geometry_ok(),
        render.bounds_ok,
        render.cursor_visible,
        render.cursor_on_continuation,
        render.cursor_span,
        render.wrap_signature,
        layout_summary,
        state.line().chars().count(),
        excerpt
    )
}

pub fn emit_final<W: Write>(
    stdout: &mut W,
    state: &EditorState,
    final_render: RenderDiagnostics,
    recent_joined: &str,
) -> io::Result<()> {
    let excerpt = frame_excerpt(state.line(), 120);
    writeln!(
        stdout,
        "FINAL mode={:?} cursor={} focused_window_id={} focused_window_type={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} line={} line_len={} frame_excerpt={} window_session={} recent_events={}",
        state.mode(),
        state.cursor(),
        state.focused_window_id(),
        state.focused_window_kind(),
        state.window_geometry_ok(),
        final_render.bounds_ok,
        final_render.cursor_visible,
        final_render.cursor_on_continuation,
        final_render.cursor_span,
        final_render.wrap_signature,
        state.line(),
        state.line().chars().count(),
        excerpt,
        state.window_session_dump(),
        recent_joined
    )
}

fn frame_excerpt(text: &str, max_chars: usize) -> String {
    let mut out = String::with_capacity(max_chars + 3);
    for ch in text.chars().take(max_chars) {
        if ch.is_whitespace() {
            out.push('_');
        } else if ch.is_control() {
            out.push('?');
        } else {
            out.push(ch);
        }
    }
    if text.chars().count() > max_chars {
        out.push_str("...");
    }
    out
}
