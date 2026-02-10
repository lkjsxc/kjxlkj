//! LSP service for language intelligence.
//!
//! Provides completion, diagnostics, hover, go-to-definition,
//! and other LSP protocol features via stdio-based language servers.

mod client;
mod response;
mod types;

#[cfg(test)]
mod lsp_tests;

pub use client::{LspService, ServerCapabilities, ServerConfig, ServerState};
pub use response::{HoverInfo, Location, LspResponse, TextEdit, WorkspaceEdit};
pub use types::{
    CompletionItem, CompletionKind, Diagnostic, DiagnosticSeverity, LspNotification, LspRequest,
    Position, Range,
};
