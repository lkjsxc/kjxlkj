//! Performance benchmarks for kjxlkj.
//!
//! These benchmarks measure:
//! - Snapshot generation cost vs viewport size
//! - Rendering cost vs viewport area
//! - File open time-to-first-snapshot for large inputs

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Cursor, Mode, Position};
use kjxlkj_core_ui::{BufferSnapshot, EditorSnapshot, StatusLine, Viewport};

/// Generate a text buffer with the specified number of lines.
fn generate_buffer(line_count: usize, line_length: usize) -> TextBuffer {
    let line = "a".repeat(line_length) + "\n";
    let content: String = line.repeat(line_count);
    TextBuffer::from(&content)
}

/// Generate a buffer snapshot for the given viewport.
fn generate_buffer_snapshot(
    buffer: &TextBuffer,
    viewport: &Viewport,
) -> BufferSnapshot {
    let lines: Vec<String> = (viewport.top_line..viewport.top_line + viewport.height)
        .filter_map(|idx| buffer.line(idx).map(String::from))
        .collect();

    BufferSnapshot::new(
        BufferId::new(1),
        BufferName::Scratch,
        BufferVersion::new(1),
        buffer.line_count(),
        lines,
        viewport.clone(),
        false,
    )
}

/// Generate a full editor snapshot.
fn generate_editor_snapshot(
    buffer: &TextBuffer,
    viewport: &Viewport,
    width: u16,
    height: u16,
) -> EditorSnapshot {
    let buf_snap = generate_buffer_snapshot(buffer, viewport);
    let cursor = Cursor::new(Position::new(0, 0));
    let status = StatusLine::new(Mode::Normal, "test.txt".to_string(), false, &cursor, buffer.line_count());

    EditorSnapshot::new(
        buf_snap,
        cursor,
        Mode::Normal,
        None,
        status,
        None,
        None,
        width,
        height,
    )
}

fn benchmark_snapshot_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot_generation");

    // Test with various viewport sizes
    for viewport_height in [10, 50, 100, 200].iter() {
        let buffer = generate_buffer(10_000, 80);
        let viewport = Viewport::new(0, *viewport_height);

        group.bench_with_input(
            BenchmarkId::new("viewport_height", viewport_height),
            viewport_height,
            |b, _| {
                b.iter(|| {
                    let snap = generate_buffer_snapshot(black_box(&buffer), black_box(&viewport));
                    black_box(snap)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_snapshot_with_line_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("snapshot_line_length");

    // Test with various line lengths
    for line_length in [80, 500, 1000, 5000].iter() {
        let buffer = generate_buffer(1000, *line_length);
        let viewport = Viewport::new(0, 50);

        group.bench_with_input(
            BenchmarkId::new("line_length", line_length),
            line_length,
            |b, _| {
                b.iter(|| {
                    let snap = generate_buffer_snapshot(black_box(&buffer), black_box(&viewport));
                    black_box(snap)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_full_editor_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("editor_snapshot");

    // Test with various terminal dimensions
    for (width, height) in [(80, 24), (120, 40), (200, 60)].iter() {
        let buffer = generate_buffer(10_000, 80);
        let viewport = Viewport::new(0, *height as usize);

        group.bench_with_input(
            BenchmarkId::new("terminal_size", format!("{}x{}", width, height)),
            &(*width, *height),
            |b, &(w, h)| {
                b.iter(|| {
                    let snap = generate_editor_snapshot(
                        black_box(&buffer),
                        black_box(&viewport),
                        w,
                        h,
                    );
                    black_box(snap)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_large_file_open(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_file_open");

    // Measure time to generate first snapshot after opening a large file
    for line_count in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("line_count", line_count),
            line_count,
            |b, &lc| {
                b.iter(|| {
                    let buffer = generate_buffer(lc, 80);
                    let viewport = Viewport::new(0, 50);
                    let snap = generate_buffer_snapshot(&buffer, &viewport);
                    black_box(snap)
                })
            },
        );
    }

    group.finish();
}

fn benchmark_viewport_scroll(c: &mut Criterion) {
    let mut group = c.benchmark_group("viewport_scroll");

    let buffer = generate_buffer(10_000, 80);
    let viewport_height = 50;

    // Measure snapshot generation at different scroll positions
    for scroll_pos in [0, 1000, 5000, 9000].iter() {
        let viewport = Viewport::new(*scroll_pos, viewport_height);

        group.bench_with_input(
            BenchmarkId::new("scroll_position", scroll_pos),
            scroll_pos,
            |b, _| {
                b.iter(|| {
                    let snap = generate_buffer_snapshot(black_box(&buffer), black_box(&viewport));
                    black_box(snap)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_snapshot_generation,
    benchmark_snapshot_with_line_length,
    benchmark_full_editor_snapshot,
    benchmark_large_file_open,
    benchmark_viewport_scroll,
);
criterion_main!(benches);
