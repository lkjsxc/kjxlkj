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
    writeln!(
        stdout,
        "TRACE event_seq={} mode_before={:?} focused_window_id={} focused_window_type={} normalized_key={} resolved_action={} cursor_before={} cursor_after={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} line={}",
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
        state.line()
    )
}

pub fn emit_final<W: Write>(
    stdout: &mut W,
    state: &EditorState,
    final_render: RenderDiagnostics,
    recent_joined: &str,
) -> io::Result<()> {
    writeln!(
        stdout,
        "FINAL mode={:?} cursor={} focused_window_id={} focused_window_type={} geometry_ok={} render_bounds_ok={} cursor_visible={} cursor_continuation={} cursor_span={} wrap_sig={} line={} window_session={} recent_events={}",
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
        state.window_session_dump(),
        recent_joined
    )
}
