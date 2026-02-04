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
- [ ] Minimal slice: implement one request/response + UI update path with tests.
  - [ ] Initialize handshake
  - [ ] textDocument/didOpen
  - [ ] textDocument/didChange
- [ ] Full conformance: implement all LSP feature documents and navigation subtrees.
- [ ] Update conformance and limitations docs when user-visible.

