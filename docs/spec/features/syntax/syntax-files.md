# Syntax Files

Tree-sitter based syntax highlighting configuration.

## Built-in Languages (normative)

### Fully Supported

Rust, Python, JavaScript, TypeScript, Go, C, C++, Markdown, JSON, YAML, TOML, HTML, CSS, Bash, Lua.

For each fully-supported language, the editor ships a compiled tree-sitter parser and a highlight query file. A typical source file in any of these languages renders with non-empty highlight spans by default.

## Language Detection (normative)

Detection priority:

1. Explicit per-buffer override (`:set filetype=rust`)
2. File extension mapping (see table below)
3. Fallback to `plain` (no structured highlighting)

### Extension Mapping (normative)

| Extension | Language ID |
|---|---|
| `.rs` | `rust` |
| `.py` | `python` |
| `.js`, `.jsx` | `javascript` |
| `.ts`, `.tsx` | `typescript` |
| `.go` | `go` |
| `.c`, `.h` | `c` |
| `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hh`, `.hxx` | `cpp` |
| `.md` | `markdown` |
| `.json` | `json` |
| `.yaml`, `.yml` | `yaml` |
| `.toml` | `toml` |
| `.html`, `.htm` | `html` |
| `.css` | `css` |
| `.sh`, `.bash` | `bash` |
| `.lua` | `lua` |

## Highlight Groups (normative)

Standard capture names used in highlight queries:

| Group | Description |
|---|---|
| `@comment` | Comments |
| `@string` | String literals |
| `@function` | Function names |
| `@function.call` | Function call sites |
| `@keyword` | Language keywords |
| `@type` | Type names |
| `@variable` | Variables |
| `@constant` | Constants |
| `@operator` | Operators |
| `@punctuation` | Brackets, commas, semicolons |
| `@property` | Struct/object fields |
| `@number` | Numeric literals |
| `@boolean` | Boolean literals |

## Tree-sitter Queries

Queries are located in the runtime data directory under `queries/{language}/highlights.scm`. Each query file contains S-expression patterns that map tree-sitter node types to highlight groups.

## Language Injection

Embedded languages (SQL in strings, regex in strings, Markdown in doc comments) are supported via injection queries in `queries/{language}/injections.scm`.

## Incremental Parsing

Only the changed region of the buffer is re-parsed on each edit. The tree-sitter incremental parsing API receives the edit range and updates the syntax tree in-place.

## Viewport Priority

Highlighting is computed for the visible viewport first. Off-screen regions are parsed in the background at lower priority.

## Custom Languages

Users can add languages by placing a compiled parser (`.so`/`.dll`/`.dylib`) and query files in the runtime directory. Configuration maps file extensions to the new language ID.

## Related

- Filetype detection: [/docs/spec/features/config/filetype.md](/docs/spec/features/config/filetype.md)
- Semantic tokens: [/docs/spec/features/syntax/semantic-tokens.md](/docs/spec/features/syntax/semantic-tokens.md)
- Inlay hints: [/docs/spec/features/syntax/inlay-hints.md](/docs/spec/features/syntax/inlay-hints.md)
