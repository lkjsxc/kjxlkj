//! kjxlkj-service-lsp: Language Server Protocol client types and utilities.

pub mod diagnostics;
pub mod lsp_features_ext;
pub mod protocol;
pub mod requests;

pub use diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticStore};
pub use lsp_features_ext::{CompletionItemEx, CompletionItemKind, CompletionList};
pub use protocol::{
    decode_message, encode_message, JsonRpcMessage, JsonRpcNotification, JsonRpcRequest,
    JsonRpcResponse,
};
pub use requests::{
    build_did_open, build_initialize_request, LspMethod, PendingRequests, ServerCapabilities,
};
