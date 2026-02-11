//! LSP client service.

pub mod diagnostic;
pub mod lifecycle;

/// Placeholder for LSP service state.
pub struct LspService;

impl LspService {
    pub fn new() -> Self {
        Self
    }
}
