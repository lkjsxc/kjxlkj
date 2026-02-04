# Editor Comparison

Back: [/docs/reference/README.md](/docs/reference/README.md)
High-level comparison of kjxlkj to other terminal editors.

This document is descriptive and non-normative. For “what exists right now”, see:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)

## Feature matrix (current vs target)

| Area | kjxlkj (current) | kjxlkj (target) | Notes |
|------|-------------------|-----------------|-------|
| Modal editing | Yes | Yes | Vim-like model |
| Operator+motion | Yes (subset) | Yes | Expanding toward spec |
| LSP | No | Yes | Placeholder service exists |
| Git integration | No | Yes | Placeholder service exists |
| Syntax highlighting | No | Yes | Planned |
| Explorer UI | No | Yes | Planned |
| Finder UI | No | Yes | Planned |
| Splits/windows | No | Yes | Planned |
| Config/remapping | No | Yes | Planned |
| Plugins | No (by design) | No (by design) | Built-ins only |

## Keybinding philosophy

### kjxlkj / (Neo)Vim

Verb → motion / text object (operators compose with targets).

### Helix / Kakoune

Selection-first (select → act).

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
