//! Snapshot-to-terminal rendering.

mod grid;
pub mod highlight;
mod lang_keywords;
mod paint;
mod painter;
mod render_tests;
pub mod syntax;
mod syntax_tests;
mod task;
pub mod wrap;
mod wrap_tests;

pub use task::spawn_render_task;
