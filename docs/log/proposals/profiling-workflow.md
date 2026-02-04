# Profiling Workflow

This document establishes a repeatable profiling workflow for detecting performance regressions.

## Overview

The kjxlkj editor includes opt-in profiling instrumentation as specified in [/docs/spec/technical/profiling.md](/docs/spec/technical/profiling.md).

## Enabling Profiling

### Compile-time

Enable the `profiling` feature when building:

```bash
cargo build --features profiling
```

### Runtime

Create a profiler and enable it:

```rust
use kjxlkj_core_types::{Profiler, ProfilingConfig};

let config = ProfilingConfig {
    enabled: true,
    log_to_stderr: true,  // Optional: print metrics
};
let mut profiler = Profiler::with_config(config);
```

## Recording Metrics

The profiler tracks per-cycle metrics:

```rust
profiler.start_cycle();
profiler.record_input_event();
profiler.record_core_update();
profiler.start_snapshot();
// ... snapshot generation ...
profiler.end_snapshot(lines_materialized);
profiler.start_render();
// ... render ...
profiler.end_render(Some(cells_written), Some(dirty_region));
profiler.end_cycle();
```

## Analysis

### Viewport-Bounded Check

Verify snapshot work is bounded by viewport:

```rust
let metrics = profiler.last_cycle().unwrap();
assert!(metrics.is_viewport_bounded(viewport_height, margin));
```

### Idle CPU Detection

Check for busy-loop redraw while idle:

```rust
// Check last 100 cycles
if profiler.detect_busy_loop(100) {
    eprintln!("Warning: Busy-loop detected while idle");
}
```

## Key Metrics

| Metric | Expected Behavior |
|--------|-------------------|
| `materialized_lines` | Should be ≤ viewport height + small margin |
| `render_duration` with no input | Should be zero when idle |
| `cells_written` | Should reflect actual changes, not full redraw |

## Probes for Testing

1. **Idle CPU probe**: Monitor render duration when no input for N cycles
2. **Long-line probe**: Verify `materialized_lines` doesn't grow with line length
3. **Large-file probe**: Verify `snapshot_duration` is independent of total buffer size

## Integration with CI

When CI is available, profiling metrics can be used as trend signals:

1. Run benchmarks with profiling enabled
2. Record metrics to a file
3. Compare against baseline
4. Flag if degradation exceeds threshold

Note: Absolute timings are machine-dependent. Prefer relative comparisons or O() complexity checks.
