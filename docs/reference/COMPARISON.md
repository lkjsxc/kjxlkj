# Editor Comparison

Back: [/docs/reference/README.md](/docs/reference/README.md)

Non-normative framing of `kjxlkj` against other terminal editors.

Current runtime truth must always come from:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Feature Framing (Target)

| Area | Target Scope in `kjxlkj` | Current Status Source |
|---|---|---|
| modal editing | yes | reference ledgers above |
| operator + motion model | yes | reference ledgers above |
| LSP services | yes | reference ledgers above |
| git integration | yes | reference ledgers above |
| syntax and highlighting | yes | reference ledgers above |
| explorer/finder | yes | reference ledgers above |
| plugin runtime | no (built-ins by design) | [/docs/spec/README.md](/docs/spec/README.md) |

## Interaction Philosophy

| Editor Family | Bias |
|---|---|
| `kjxlkj` / Vim family | verb + motion/text-object composition |
| Helix/Kakoune family | selection-first editing |

## Configuration Philosophy

| Editor | Configuration Surface |
|---|---|
| `kjxlkj` | doc-defined options + command/mapping scripting |
| Neovim | Lua |
| Helix | TOML |
| Kakoune | custom DSL + shell |
