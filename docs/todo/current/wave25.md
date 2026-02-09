# Wave 25 — Reconstruction Requirements

| Requirement ID | Description | Spec Link | Done |
|---|---|---|---|
| REQ-ATOMICGRP-01 | Regex atomic groups `\(>...\)` and possessive quantifiers `\{-}` translated to Rust regex | `/docs/spec/editing/regex/README.md` | `[x]` |
| REQ-IFELSE-01 | Conditional execution: `if`/`elseif`/`else`/`endif` in `:source` scripts | `/docs/spec/scripting/README.md` | `[x]` |
| REQ-AUTOSAVE-01 | Session auto-save: `autosave` option triggers periodic `:mksession` calls | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-VISUALSORT-01 | Visual `:sort` applies to selected line range, `:!` filter through external cmd | `/docs/spec/modes/visual.md` | `[x]` |
| REQ-PATHEXPAND-01 | Path completion expands `~` to HOME and handles relative paths properly | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-MATCHFUNC-01 | `match(str, pattern)` and `substitute(str, pat, rep, flags)` in expression evaluator | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-MACROSTEP-01 | Macro stepping via `:debug @a` executes one key at a time with display | `/docs/spec/editing/macros/README.md` | `[x]` |
| REQ-FTPLUGIN-01 | Filetype plugin: load `ftplugin/{ft}.vim` on filetype change for per-type settings | `/docs/spec/features/config/README.md` | `[x]` |
