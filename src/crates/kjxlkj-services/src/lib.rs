//! Service supervisor for kjxlkj editor.
//!
//! This crate manages all background services.

mod bus;
mod supervisor;

pub use bus::{Message, MessageBus};
pub use supervisor::ServiceSupervisor;

// Re-exports
pub use kjxlkj_service_fs as fs;
pub use kjxlkj_service_git as git;
pub use kjxlkj_service_index as index;
pub use kjxlkj_service_lsp as lsp;
pub use kjxlkj_service_terminal as terminal;
