//! LSP client service for kjxlkj editor.
//!
//! This crate implements the Language Server Protocol client.

mod client;
mod protocol;
mod service;

#[cfg(test)]
mod tests;

pub use client::LspClient;
pub use protocol::{Diagnostic, DiagnosticSeverity};
pub use service::LspService;
