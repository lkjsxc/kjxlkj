//! Extended LSP protocol types: hover, signature help, code actions, symbols, etc.

use serde::{Deserialize, Serialize};

/// Markup kind for hover/signature documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkupKind { #[serde(rename = "plaintext")] PlainText, #[serde(rename = "markdown")] Markdown }

/// Markup content with optional kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent { pub kind: MarkupKind, pub value: String }

/// Hover result from textDocument/hover.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: HoverContents,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<LspRangeJson>,
}

/// Hover contents variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents { Markup(MarkupContent), MarkedString(MarkedString), Array(Vec<MarkedString>) }

/// A marked string (language + value or plain string).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkedString { Simple(String), LanguageString { language: String, value: String } }

/// JSON-serializable LSP range and position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspRangeJson { pub start: LspPosJson, pub end: LspPosJson }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspPosJson { pub line: u32, pub character: u32 }

/// Signature help result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_signature: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_parameter: Option<u32>,
}

/// Information about a callable signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInformation {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<MarkupContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterInformation>>,
}

/// Parameter information within a signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub label: ParameterLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<MarkupContent>,
}

/// Parameter label: either a string or an offset range.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterLabel { Simple(String), Offsets([u32; 2]) }

/// Code action kind constants.
pub mod code_action_kind {
    pub const QUICK_FIX: &str = "quickfix";
    pub const REFACTOR: &str = "refactor";
    pub const REFACTOR_EXTRACT: &str = "refactor.extract";
    pub const SOURCE: &str = "source";
    pub const SOURCE_ORGANIZE_IMPORTS: &str = "source.organizeImports";
    pub const SOURCE_FIX_ALL: &str = "source.fixAll";
}

/// Code action response from the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionResponse {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<WorkspaceEdit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<LspCommand>,
}

/// A workspace edit containing document changes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<std::collections::HashMap<String, Vec<TextEditJson>>>,
}

/// Text edit in JSON form.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEditJson { pub range: LspRangeJson, pub new_text: String }

/// An LSP command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspCommand {
    pub title: String, pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<serde_json::Value>>,
}

/// Location link for go-to-definition with origin/target ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_selection_range: Option<LspRangeJson>,
    pub target_uri: String, pub target_range: LspRangeJson, pub target_selection_range: LspRangeJson,
}

/// Rename params.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameParams {
    pub text_document: super::protocol::TextDocumentIdentifier,
    pub position: LspPosJson, pub new_name: String,
}

/// Prepare rename response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepareRenameResponse { pub range: LspRangeJson, pub placeholder: String }

/// Code lens — a command shown inline in the editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLens { pub range: LspRangeJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<LspCommand>,
}

/// Formatting options for textDocument/formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormattingOptions {
    pub tab_size: u32, pub insert_spaces: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_trailing_whitespace: Option<bool>,
}

/// Symbol kind from LSP spec (1-26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    File = 1, Module = 2, Namespace = 3, Package = 4, Class = 5, Method = 6,
    Property = 7, Field = 8, Constructor = 9, Enum = 10, Interface = 11,
    Function = 12, Variable = 13, Constant = 14, String = 15, Number = 16,
    Boolean = 17, Array = 18, Object = 19, Key = 20, Null = 21,
    EnumMember = 22, Struct = 23, Event = 24, Operator = 25, TypeParameter = 26,
}

/// Document symbol (tree-shaped, with optional children).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbol {
    pub name: String, pub kind: SymbolKind, pub range: LspRangeJson,
    pub selection_range: LspRangeJson,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocumentSymbol>>,
}

/// Symbol information (flat-shaped, with location).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInformation { pub name: String, pub kind: SymbolKind, pub location: LocationJson }
/// Location in JSON form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationJson { pub uri: String, pub range: LspRangeJson }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hover_and_signature() {
        let h = Hover { contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown, value: "# Hello".into() }), range: None };
        let j = serde_json::to_string(&h).unwrap();
        assert!(j.contains("markdown"));
        let sh = SignatureHelp { signatures: vec![SignatureInformation {
            label: "fn foo(x: i32)".into(), documentation: None,
            parameters: Some(vec![ParameterInformation {
                label: ParameterLabel::Simple("x: i32".into()), documentation: None }]) }],
            active_signature: Some(0), active_parameter: Some(0) };
        assert_eq!(sh.signatures.len(), 1);
    }
    #[test]
    fn code_action_and_symbols() {
        assert_eq!(code_action_kind::QUICK_FIX, "quickfix");
        assert!(WorkspaceEdit::default().changes.is_none());
        assert_eq!(SymbolKind::File as u8, 1);
        assert_eq!(SymbolKind::TypeParameter as u8, 26);
    }
}
