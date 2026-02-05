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
- [x] Minimal slice: implement one request/response + UI update path with tests.
  - [x] Initialize handshake (InitializeParams, ServerCapabilities)
  - [x] textDocument/didOpen (DidOpenTextDocumentParams, TextDocumentItem)
  - [x] textDocument/didChange (DidChangeTextDocumentParams, ContentChangeEvent)
- [x] Full conformance: implement all LSP feature documents and navigation subtrees.
  - [x] Hover (Hover, HoverContents, MarkupContent, MarkupKind, MarkedString)
  - [x] Signature Help (SignatureHelp, SignatureInformation, ParameterInformation)
  - [x] Code Actions (CodeAction, CodeActionKind, WorkspaceEdit, TextEdit, Command)
  - [x] Navigation (Location, LocationLink, DefinitionResponse)
  - [x] Rename (RenameParams, PrepareRenameResponse)
  - [x] Code Lens (CodeLens)
  - [x] Formatting (FormattingOptions)
  - [x] Symbols (SymbolKind, DocumentSymbol, SymbolInformation)
  - [x] Extended Completion (CompletionItemKind with icons, CompletionItemEx, CompletionList)
- [ ] Update conformance and limitations docs when user-visible.

