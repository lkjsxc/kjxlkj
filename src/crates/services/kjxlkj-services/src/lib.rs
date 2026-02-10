//! Services supervisor.
//!
//! This crate coordinates all service tasks.

mod supervisor;

pub use kjxlkj_service_explorer as explorer;
pub use kjxlkj_service_fs as fs;
pub use kjxlkj_service_git as git;
pub use kjxlkj_service_index as index;
pub use kjxlkj_service_lsp as lsp;
pub use kjxlkj_service_terminal as terminal;
pub use supervisor::*;
