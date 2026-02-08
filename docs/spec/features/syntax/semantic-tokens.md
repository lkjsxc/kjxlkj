# Semantic Tokens

Back: [/docs/spec/features/syntax/README.md](/docs/spec/features/syntax/README.md)

LSP-powered semantic highlighting that supplements Tree-sitter syntax highlighting.

## Overview

Semantic tokens provide language-aware highlighting from the LSP server. The server analyzes the code and assigns token types and modifiers to spans. These override or augment the default syntax highlighting.

## LSP protocol

The editor uses `textDocument/semanticTokens/full` for initial highlighting and `textDocument/semanticTokens/full/delta` for incremental updates.

### Request flow

1. On buffer open, request full semantic tokens.
2. On buffer edit, request delta tokens (changes since last response).
3. Apply token types and modifiers to highlight groups.
4. Re-render affected lines.

## Token types

Standard LSP semantic token types:

| Type | Description | Default highlight group |
|---|---|---|
| `namespace` | Package/module name | `@namespace` |
| `type` | Type name | `@type` |
| `class` | Class definition | `@type` |
| `enum` | Enum definition | `@type` |
| `interface` | Interface definition | `@type` |
| `struct` | Struct definition | `@type` |
| `typeParameter` | Generic parameter | `@type.parameter` |
| `parameter` | Function parameter | `@parameter` |
| `variable` | Variable | `@variable` |
| `property` | Object property | `@property` |
| `enumMember` | Enum variant | `@constant` |
| `function` | Function name | `@function` |
| `method` | Method name | `@method` |
| `macro` | Macro name | `@macro` |
| `keyword` | Language keyword | `@keyword` |
| `comment` | Comment | `@comment` |
| `string` | String literal | `@string` |
| `number` | Numeric literal | `@number` |
| `operator` | Operator | `@operator` |

## Token modifiers

Modifiers refine token types with additional semantic information:

| Modifier | Description | Effect |
|---|---|---|
| `declaration` | Token is a declaration | Bold |
| `definition` | Token is a definition | Bold |
| `readonly` | Immutable binding | Italic |
| `static` | Static member | Underline |
| `deprecated` | Deprecated symbol | Strikethrough |
| `async` | Async function | Italic |
| `modification` | Variable being mutated | Underline |
| `documentation` | Documentation comment | Italic |
| `defaultLibrary` | Standard library symbol | Different shade |

## Priority

Semantic tokens take priority over Tree-sitter syntax highlighting for the same text range. The rendering pipeline applies tokens in order:

1. Default foreground/background
2. Tree-sitter syntax highlighting
3. Semantic token highlighting (overrides Tree-sitter)
4. Diagnostic underlines (overlaid)

## Configuration

| Setting | Type | Default | Description |
|---|---|---|---|
| `semantic_tokens.enabled` | boolean | `true` | Enable semantic token highlighting |

## Performance

Semantic token requests run asynchronously. The editor does not block on semantic token responses. If the response is slow, Tree-sitter highlighting is used until semantic tokens arrive.

Delta requests minimize payload size for incremental edits.

## Server support

| Server | Semantic tokens |
|---|---|
| rust-analyzer | Full + delta |
| typescript-language-server | Full + delta |
| clangd | Full |
| gopls | Full |
| pyright | Full |

## Related

- Syntax highlighting: [/docs/spec/features/syntax/syntax.md](/docs/spec/features/syntax/syntax.md)
- Highlight groups: [/docs/spec/features/syntax/highlight-groups.md](/docs/spec/features/syntax/highlight-groups.md)
- LSP: [/docs/spec/features/lsp/lsp.md](/docs/spec/features/lsp/lsp.md)
