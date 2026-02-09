# Editor Comparison

Back: [/docs/reference/README.md](/docs/reference/README.md)
High-level comparison of kjxlkj to other terminal editors.

This document is descriptive and non-normative. For current implementation status, use:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Feature matrix (target framing)

| Area | Target scope in kjxlkj | Current status source | Notes |
|------|-------------------------|-----------------------|-------|
| Modal editing | Yes | `/docs/reference/conformance/CONFORMANCE_MODES.md` | Vim-like model |
| Operator+motion | Yes | `/docs/reference/conformance/CONFORMANCE_EDITING_OPERATORS.md` | Command/editing surface |
| LSP | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in service target |
| Git integration | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in service target |
| Syntax highlighting | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Explorer UI | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Finder UI | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Built-in feature target |
| Splits/windows | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Core editor model |
| Config/remapping | Yes | `/docs/reference/CONFORMANCE.md` + `/docs/reference/LIMITATIONS.md` | Config/scripting surface |
| Plugins | No (by design) | `/docs/spec/README.md` | Built-ins only |

## Keybinding philosophy

How key binding models differ across editors.

### kjxlkj / (Neo)Vim

Verb -> motion / text object (operators compose with targets).

### Helix / Kakoune

Selection-first (select -> act).

## Configuration philosophy

| Editor | Typical config | Scripting |
|--------|----------------|----------|
| kjxlkj | (planned) TOML | None (by design) |
| Neovim | Lua | Full |
| Helix | TOML | None |
| Kakoune | Custom | Shell |

## Performance claims

This repository does not currently include published benchmarks. Performance characteristics are a target, not a measured guarantee.

See `/docs/technical/` for technical notes and `/docs/spec/technical/` for target requirements.
