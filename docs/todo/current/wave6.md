# Wave 6 — Regex, f/t Motions, Case Operators, Register Wiring

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Scope

Regex engine integration, find-char motions, case change
operators, and register-aware yank/delete/put.

## Requirements

| ID | Description | Spec Link | Status |
|---|---|---|---|
| REQ-REGEX-01 | Regex crate integration for search | `/docs/spec/editing/regex/regex.md` | `[x]` |
| REQ-REGEX-02 | Regex in :substitute | `/docs/spec/editing/regex/regex.md` | `[x]` |
| REQ-REGEX-03 | Vim magic → Rust regex translation | `/docs/spec/editing/regex/regex.md` | `[x]` |
| REQ-FT-01 | f{char} find char forward on line | `/docs/spec/editing/motions/motions.md` | `[x]` |
| REQ-FT-02 | F{char} find char backward on line | `/docs/spec/editing/motions/motions.md` | `[x]` |
| REQ-FT-03 | t{char}/T{char} find till char | `/docs/spec/editing/motions/motions.md` | `[x]` |
| REQ-FT-04 | ; and , repeat last f/t/F/T | `/docs/spec/editing/motions/motions.md` | `[x]` |
| REQ-CASE-01 | ~ toggle case under cursor | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-CASE-02 | gu{motion} lowercase | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-CASE-03 | gU{motion} uppercase | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-REG-02 | "{char}y uses selected register for yank | `/docs/spec/editing/registers/README.md` | `[x]` |
| REQ-REG-03 | "{char}d uses selected register for delete | `/docs/spec/editing/registers/README.md` | `[x]` |
| REQ-REG-04 | "{char}p uses selected register for put | `/docs/spec/editing/registers/README.md` | `[x]` |

## Exit Criteria

- `cargo build` clean
- `cargo test` passes all tests
- `cargo clippy` zero warnings
- All files ≤ 199 lines
- LIMITATIONS updated
