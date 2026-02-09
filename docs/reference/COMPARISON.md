# Editor Comparison

Back: [/docs/reference/README.md](/docs/reference/README.md)
High-level comparison of kjxlkj to other terminal editors.

This document is descriptive and non-normative. For current implementation status, use:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Feature Matrix (Target Framing)

| Area | Target scope in kjxlkj | Current status source | Notes |
|------|-------------------------|-----------------------|-------|
| Modal editing | Targeted | `/docs/reference/conformance/CONFORMANCE_MODES.md` | Vim-like model |
| Operator+motion | Targeted | `/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md` | Command/editing surface |
| LSP | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in service target |
| Git integration | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in service target |
| Syntax highlighting | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Explorer UI | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Finder UI | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Splits/windows | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Core editor model |
| Config/remapping | Targeted | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Config/scripting surface |
| Plugins | No (by design) | `/docs/spec/README.md` | Built-ins only |

## Keybinding philosophy

How key binding models differ across editors.

### kjxlkj / (Neo)Vim

Verb -> motion / text object (operators compose with targets).

### Helix / Kakoune

Selection-first (select -> act).

## Configuration and Scripting Philosophy

| Editor | Typical config | Scripting |
|--------|----------------|----------|
| kjxlkj | TOML-oriented target + command/mapping scripting surfaces | See `/docs/spec/scripting/` and status ledgers |
| Neovim | Lua | Full |
| Helix | TOML | None |
| Kakoune | Custom | Shell |

## Performance claims

This repository may be in docs-only or partially reconstructed states. Performance characteristics are target requirements unless evidence is linked in conformance/audits.

See `/docs/technical/` for technical notes and `/docs/spec/technical/` for target requirements.
