//! Tests for terminal mode, completion menu, latency tracking, and language detection.

use kjxlkj_core_types::{LanguageId, Mode};
use kjxlkj_core_types::perf::{LatencyStats, LatencyTracker, ManualTimer, MemorySnapshot};
use std::time::Duration;

// ──────── Terminal mode type tests ────────

#[test]
fn terminal_mode_exists() {
    let mode = Mode::Terminal;
    assert_eq!(format!("{}", mode), "TERMINAL");
    assert!(mode.is_terminal());
    assert!(!mode.is_visual());
    assert!(!mode.is_insert_like());
}

#[test]
fn terminal_mode_from_name() {
    assert_eq!(Mode::from_name("terminal"), Some(Mode::Terminal));
    assert_eq!(Mode::from_name("t"), Some(Mode::Terminal));
}

#[test]
fn terminal_mode_cursor_shape() {
    use kjxlkj_core_types::CursorShape;
    assert_eq!(Mode::Terminal.cursor_shape(), CursorShape::Bar);
}

// ──────── Latency tracking tests ────────

#[test]
fn latency_stats_record_and_avg() {
    let mut stats = LatencyStats::new("key");
    stats.record(Duration::from_micros(100));
    stats.record(Duration::from_micros(300));
    assert_eq!(stats.count, 2);
    assert_eq!(stats.avg(), Duration::from_micros(200));
    assert_eq!(stats.min, Duration::from_micros(100));
    assert_eq!(stats.max, Duration::from_micros(300));
}

#[test]
fn latency_stats_exceeds_threshold() {
    let mut stats = LatencyStats::new("search");
    stats.record(Duration::from_millis(60));
    assert!(stats.exceeds(Duration::from_millis(50)));
    assert!(!stats.exceeds(Duration::from_millis(100)));
}

#[test]
fn manual_timer_measures_elapsed() {
    let timer = ManualTimer::start("test");
    std::thread::sleep(Duration::from_millis(1));
    let sample = timer.finish();
    assert!(sample.duration >= Duration::from_micros(500));
    assert_eq!(sample.label, "test");
}

#[test]
fn memory_snapshot_total() {
    let snap = MemorySnapshot { buffer_bytes: 1024, undo_bytes: 512, viewport_bytes: 256, total_buffers: 2 };
    assert_eq!(snap.total(), 1792);
    assert_eq!(snap.total_buffers, 2);
}

#[test]
fn latency_tracker_multiple_metrics() {
    let mut tracker = LatencyTracker::new();
    tracker.record("keystroke", Duration::from_micros(100));
    tracker.record("keystroke", Duration::from_micros(200));
    tracker.record("file_open", Duration::from_millis(50));
    let ks = tracker.summary("keystroke").unwrap();
    assert_eq!(ks.count, 2);
    let fo = tracker.summary("file_open").unwrap();
    assert_eq!(fo.count, 1);
    assert!(tracker.summary("nonexistent").is_none());
}

#[test]
fn latency_tracker_regression_detection() {
    let mut tracker = LatencyTracker::new();
    tracker.record("keystroke", Duration::from_millis(5)); // within 16ms budget
    assert!(!tracker.has_regressions());
    tracker.record("keystroke", Duration::from_millis(20)); // exceeds 16ms budget
    assert!(tracker.has_regressions());
}

// ──────── Language detection integration tests ────────

#[test]
fn language_detection_extensions() {
    assert_eq!(LanguageId::from_extension("rs"), LanguageId::Rust);
    assert_eq!(LanguageId::from_extension("py"), LanguageId::Python);
    assert_eq!(LanguageId::from_extension("js"), LanguageId::JavaScript);
    assert_eq!(LanguageId::from_extension("ts"), LanguageId::TypeScript);
    assert_eq!(LanguageId::from_extension("go"), LanguageId::Go);
    assert_eq!(LanguageId::from_extension("java"), LanguageId::Java);
    assert_eq!(LanguageId::from_extension("cpp"), LanguageId::Cpp);
    assert_eq!(LanguageId::from_extension("zig"), LanguageId::Zig);
    assert_eq!(LanguageId::from_extension("lua"), LanguageId::Lua);
    assert_eq!(LanguageId::from_extension("unknown"), LanguageId::Plain);
}

#[test]
fn language_detection_filenames() {
    assert_eq!(LanguageId::from_filename("Dockerfile"), LanguageId::Dockerfile);
    assert_eq!(LanguageId::from_filename("Makefile"), LanguageId::Makefile);
    assert_eq!(LanguageId::from_filename("main.rs"), LanguageId::Rust);
    assert_eq!(LanguageId::from_filename("config.yaml"), LanguageId::Yaml);
}

#[test]
fn language_detection_paths() {
    assert_eq!(LanguageId::detect("/home/user/project/src/main.rs"), LanguageId::Rust);
    assert_eq!(LanguageId::detect("scripts/build.sh"), LanguageId::Shell);
    assert_eq!(LanguageId::detect("Makefile"), LanguageId::Makefile);
}

#[test]
fn language_lsp_ids() {
    assert_eq!(LanguageId::Rust.lsp_id(), "rust");
    assert_eq!(LanguageId::Python.lsp_id(), "python");
    assert_eq!(LanguageId::TypeScript.lsp_id(), "typescript");
    assert_eq!(LanguageId::Markdown.lsp_id(), "markdown");
    assert_eq!(LanguageId::Plain.lsp_id(), "plaintext");
}

#[test]
fn language_display() {
    assert_eq!(format!("{}", LanguageId::Rust), "rust");
    assert_eq!(format!("{}", LanguageId::Python), "python");
}
