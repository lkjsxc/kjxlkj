//! Core types shared across all kjxlkj crates.
//!
//! This crate provides foundational types used by the editor core, UI,
//! render pipeline, and services.

mod buffer;
mod cursor;
mod event;
mod mode;
mod position;
pub mod profiling;
mod register;
mod selection;

pub use buffer::{BufferId, BufferName, BufferVersion};
pub use cursor::Cursor;
pub use event::{
    EditorEvent, ExplorerIntent, Intent, KeyCode, KeyEvent, KeyModifiers, MotionIntent,
    ScrollIntent,
};
pub use mode::Mode;
pub use position::Position;
pub use profiling::{CycleMetrics, Profiler, ProfilingConfig};
pub use register::{Register, RegisterName};
pub use selection::{Selection, SelectionKind};
