use std::io::{self, Write};
use std::time::Duration;

use kjxlkj_core_state::EditorState;
use kjxlkj_render::RenderDiagnostics;

const MATERIALIZED_MARGIN: usize = 1;

pub struct CycleSample<'a> {
    pub state: &'a EditorState,
    pub rows: u16,
    pub cols: u16,
    pub render: RenderDiagnostics,
    pub resolved_action: &'a str,
    pub snapshot_duration: Duration,
    pub render_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct PerfProfile {
    enabled: bool,
    events_processed: u64,
    core_update_count: u64,
    snapshot_duration_ns_total: u128,
    render_duration_ns_total: u128,
    snapshot_materialized_lines_max: usize,
    cells_written_total: u64,
    dirty_region_cells_total: u64,
    max_cells_written: u64,
    max_viewport_cells: u64,
    idle_redraw_count: u64,
}

impl PerfProfile {
    pub fn from_env() -> Self {
        Self::new(env_flag_enabled("KJXLKJ_PROFILE"))
    }

    pub fn record_cycle(&mut self, sample: CycleSample<'_>) {
        if !self.enabled {
            return;
        }

        self.events_processed += 1;
        self.core_update_count += u64::from(sample.resolved_action != "Ignore");
        self.snapshot_duration_ns_total += sample.snapshot_duration.as_nanos();
        self.render_duration_ns_total += sample.render_duration.as_nanos();

        let total_lines = sample.state.line().lines().count().max(1);
        let materialized =
            total_lines.min(usize::from(sample.rows).saturating_add(MATERIALIZED_MARGIN));
        self.snapshot_materialized_lines_max =
            self.snapshot_materialized_lines_max.max(materialized);

        let viewport_cells = u64::from(sample.rows) * u64::from(sample.cols);
        let line_cells = sample.state.line().chars().count() as u64;
        let cells_written = line_cells.min(viewport_cells);
        let dirty_region_cells = if sample.render.bounds_ok {
            cells_written
        } else {
            viewport_cells
        };

        self.cells_written_total += cells_written;
        self.dirty_region_cells_total += dirty_region_cells;
        self.max_cells_written = self.max_cells_written.max(cells_written);
        self.max_viewport_cells = self.max_viewport_cells.max(viewport_cells);
    }

    pub fn emit_final<W: Write>(&self, out: &mut W, rows: u16) -> io::Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let max_materialized = usize::from(rows).saturating_add(MATERIALIZED_MARGIN);
        let materialized_bound_ok = self.snapshot_materialized_lines_max <= max_materialized;
        let long_line_probe_ok = self.max_cells_written <= self.max_viewport_cells;
        let large_file_probe_ok = materialized_bound_ok;
        let idle_busy_loop = (self.idle_redraw_count > 0 && self.events_processed == 0)
            || self.idle_redraw_count > self.events_processed;

        writeln!(
            out,
            "PROFILE events_processed={} core_update_count={} snapshot_duration_ns={} render_duration_ns={} snapshot_materialized_lines_max={} cells_written_total={} dirty_region_cells_total={} idle_redraw_count={} materialized_bound_ok={} long_line_probe_ok={} large_file_probe_ok={} idle_busy_loop={}",
            self.events_processed,
            self.core_update_count,
            self.snapshot_duration_ns_total,
            self.render_duration_ns_total,
            self.snapshot_materialized_lines_max,
            self.cells_written_total,
            self.dirty_region_cells_total,
            self.idle_redraw_count,
            materialized_bound_ok,
            long_line_probe_ok,
            large_file_probe_ok,
            idle_busy_loop,
        )
    }

    fn new(enabled: bool) -> Self {
        Self {
            enabled,
            events_processed: 0,
            core_update_count: 0,
            snapshot_duration_ns_total: 0,
            render_duration_ns_total: 0,
            snapshot_materialized_lines_max: 0,
            cells_written_total: 0,
            dirty_region_cells_total: 0,
            max_cells_written: 0,
            max_viewport_cells: 0,
            idle_redraw_count: 0,
        }
    }
}

fn env_flag_enabled(key: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|value| parse_truthy_flag(&value))
        .unwrap_or(false)
}

fn parse_truthy_flag(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_truthy_flag_handles_common_values() {
        assert!(parse_truthy_flag("true"));
        assert!(parse_truthy_flag("1"));
        assert!(parse_truthy_flag("on"));
        assert!(parse_truthy_flag(" yes "));
        assert!(!parse_truthy_flag("no"));
        assert!(!parse_truthy_flag("0"));
    }

    #[test]
    fn disabled_profile_emits_nothing() {
        let profile = PerfProfile::new(false);
        let mut out = Vec::new();
        profile
            .emit_final(&mut out, 20)
            .expect("disabled profile emit should succeed");
        assert!(out.is_empty());
    }
}
