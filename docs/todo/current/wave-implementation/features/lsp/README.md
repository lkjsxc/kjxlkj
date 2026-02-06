# Features: LSP (Iteration 34)

Back: [/docs/todo/current/wave-implementation/features/README.md](/docs/todo/current/wave-implementation/features/README.md)

## Scope

Implement built-in Language Server Protocol integration.

## Defining documents (direct, normative)

- LSP features index:
  - [/docs/spec/features/lsp/README.md](/docs/spec/features/lsp/README.md)

## Coverage traversal

- LSP subtree:
  - [/docs/todo/doc-coverage/spec/features/lsp/README.md](/docs/todo/doc-coverage/spec/features/lsp/README.md)

## Checklist

- [x] Placeholder scaffolding: define LSP service APIs and diagnostic surfaces.
  - LspService with server config management
  - LspServerConfig for rust-analyzer, typescript-language-server
  - Diagnostic and CompletionItem types
  - LspPosition, LspRange types
  - ServerCapabilities model
  - HoverInfo, CodeAction, TextEdit, Location types
- [ ] Minimal slice: implement one request/response + UI update path with tests.
  - [x] Initialize handshake (InitializeParams, ServerCapabilities)
    - make_initialize(), InitializeParams, ClientCapabilities, JSON-RPC Request/Response/Notification types
  - [x] textDocument/didOpen (DidOpenTextDocumentParams, TextDocumentItem)
    - make_did_open(), TextDocumentItem, encode_message()
  - [x] textDocument/didChange (DidChangeTextDocumentParams, ContentChangeEvent)
    - make_did_change(), VersionedTextDocId, ContentChange, DidCloseParams
- [ ] Full conformance: implement all LSP feature documents and navigation subtrees.
  - [x] Hover (Hover, HoverContents, MarkupContent, MarkupKind, MarkedString)
    - protocol_ext.rs: Hover, HoverContents (Markup/MarkedString/Array), MarkupContent, MarkupKind
  - [x] Signature Help (SignatureHelp, SignatureInformation, ParameterInformation)
    - SignatureHelp with active_signature/active_parameter, ParameterLabel (Simple/Offsets)
  - [x] Code Actions (CodeAction, CodeActionKind, WorkspaceEdit, TextEdit, Command)
    - CodeActionResponse, WorkspaceEdit, TextEditJson, LspCommand, code_action_kind consts
  - [x] Navigation (Location, LocationLink, DefinitionResponse)
    - LocationLink with origin/target ranges, LocationJson
  - [x] Rename (RenameParams, PrepareRenameResponse)
  - [x] Code Lens (CodeLens)
  - [x] Formatting (FormattingOptions)
  - [x] Symbols (SymbolKind, DocumentSymbol, SymbolInformation)
    - SymbolKind (26 variants), DocumentSymbol (tree), SymbolInformation (flat)
  - [ ] Extended Completion (CompletionItemKind with icons, CompletionItemEx, CompletionList)
- [ ] Update conformance and limitations docs when user-visible.

