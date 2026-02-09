//! Language Server Protocol client service.

mod client;
mod codec;
mod protocol;
mod service;

pub use client::LspClient;
pub use service::LspService;
