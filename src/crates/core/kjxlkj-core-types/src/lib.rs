#![forbid(unsafe_code)]

mod ids;
mod mode;
mod position;

pub use ids::{BufferId, BufferVersion, WindowId};
pub use mode::Mode;
pub use position::{CursorPos, TextRange};

