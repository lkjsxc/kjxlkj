//! kjxlkj-service-lsp - LSP client service.
//!
//! This crate provides Language Server Protocol client functionality.

#![allow(dead_code)]

mod client;
mod protocol;

pub use client::LspClient;
pub use protocol::{LspRequest, LspResponse, LspNotification};
