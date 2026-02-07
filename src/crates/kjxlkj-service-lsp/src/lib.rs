//! kjxlkj-service-lsp: Language Server Protocol client types and utilities.

pub mod diagnostics;
pub mod lsp_features_ext;
pub mod protocol;
pub mod requests;

pub use diagnostics::{Diagnostic, DiagnosticSeverity, DiagnosticStore};
pub use lsp_features_ext::{CompletionItemEx, CompletionItemKind, CompletionList};
pub use protocol::{
    JsonRpcMessage, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, decode_message,
    encode_message,
};
pub use requests::{
    LspMethod, PendingRequests, ServerCapabilities, build_did_open, build_initialize_request,
};
