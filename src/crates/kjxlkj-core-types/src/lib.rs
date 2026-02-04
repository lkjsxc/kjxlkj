//! Core types shared across all kjxlkj crates.

mod buffer;
mod cursor;
mod mode;
mod position;
mod register;
mod selection;

pub use buffer::{BufferId, BufferVersion};
pub use cursor::Cursor;
pub use mode::Mode;
pub use position::Position;
pub use register::{Register, RegisterName};
pub use selection::{Selection, SelectionKind};
