//! Types for xml_attrless protocol per /docs/spec/api/librarian-xml.md.
//! Split from xml_parser.rs per 200-line policy.

use serde::{Deserialize, Serialize};

/// Parsed librarian response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianResponse {
    pub request_id: String,
    pub status: String,
    pub summary: String,
    pub operations: Vec<ParsedOperation>,
    pub warnings: Vec<String>,
}

/// Parsed operation block per spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedOperation {
    pub operation_id: String,
    pub kind: String,
    pub target_note_id: Option<String>,
    pub target_path: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub reason: String,
    pub confidence: f32,
}

/// Parse error types per /docs/spec/api/librarian-xml.md.
#[derive(Debug, Clone)]
pub enum ParseError {
    ProtocolInvalid(String),
    ParseFailed(String),
    MissingTag(String),
    InvalidConfidence(String),
    TooManyOperations { max: usize, found: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProtocolInvalid(m) => write!(f, "LIBRARIAN_PROTOCOL_INVALID: {m}"),
            Self::ParseFailed(m) => write!(f, "LIBRARIAN_PARSE_FAILED: {m}"),
            Self::MissingTag(t) => write!(f, "LIBRARIAN_PARSE_FAILED: missing <{t}>"),
            Self::InvalidConfidence(v) => {
                write!(f, "LIBRARIAN_PARSE_FAILED: confidence '{v}' not in [0,1]")
            }
            Self::TooManyOperations { max, found } => {
                write!(f, "LIBRARIAN_OPERATION_REJECTED: {found} > max {max}")
            }
        }
    }
}

/// Valid operation kinds per /docs/spec/api/types.md.
pub const VALID_KINDS: &[&str] = &[
    "create_note",
    "rewrite_note",
    "retitle_note",
    "relink_note",
    "retag_note",
    "defer",
];

/// Strict-mode-only kinds per small-model compatibility.
pub const STRICT_KINDS: &[&str] = &["create_note", "rewrite_note"];
