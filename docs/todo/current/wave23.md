# Wave 23 — Reconstruction Requirements

| Requirement ID | Description | Spec Link | Done |
|---|---|---|---|
| REQ-AUTOLOAD-01 | Autoload function resolution: `autoload#func()` splits on `#` to derive script path and function name | `/docs/spec/scripting/README.md` | `[x]` |
| REQ-TABBUFASSOC-01 | Tab-specific buffer associations: each tab stores its own buffer list in session data | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-CTXCOMPL-01 | Context-aware argument completion: detect command type and offer relevant candidates (help topics, mark names, register names, color/highlight names) | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-DICTITER-01 | Dict iteration functions `keys()` and `values()` return list of dict keys/values | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-VIMINFOAUTOSAVE-01 | Auto-save viminfo on quit: serialize global marks to viminfo file when editor exits | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-KEYWORDCOUNT-01 | K command with count: `2K` passes count as section argument to keywordprg | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-SNIPPETNEST-01 | Nested snippet placeholders: `${1:outer ${2:inner}}` allows tab-stops inside defaults | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-RANGEEXPRFUNC-01 | Complex expression addresses with function calls: `(line(".")+1)` evaluates function calls within range addresses | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
