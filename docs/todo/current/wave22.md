# Wave 22 — Reconstruction Requirements

| Requirement ID | Description | Spec Link | Done |
|---|---|---|---|
| REQ-SCRIPTLOCAL-01 | Script-local `s:` variable namespace and `function()` references in expression evaluator | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-TABWINLAYOUT-01 | Tab page window layouts serialized per-tab in session with `tablayout` entries | `/docs/spec/features/session/README.md` | `[x]` |
| REQ-POPUPINSERT-01 | Enter on popup menu inserts selected completion candidate text into the command line | `/docs/spec/commands/cmdline/completion.md` | `[x]` |
| REQ-DICTACCESS-01 | Dict key access via `dict["key"]` and `has_key(dict, "key")` builtin function in expression evaluator | `/docs/spec/scripting/user-functions.md` | `[x]` |
| REQ-VIMINFOMARKS-01 | Viminfo file stores global marks (A-Z) and numbered marks (0-9) across editor sessions | `/docs/spec/editing/marks/README.md` | `[x]` |
| REQ-KEYWORDPRG-01 | K command looks up keyword under cursor using `keywordprg` option (default: "man") | `/docs/spec/editing/text-manipulation/README.md` | `[x]` |
| REQ-SNIPPETMIRROR-01 | Snippet mirror tab-stops: `${1}` after `${1:default}` repeats the same text at multiple positions | `/docs/spec/features/editing/README.md` | `[x]` |
| REQ-RANGEEXPR-01 | Range addresses support simple expression evaluation for computed line numbers | `/docs/spec/commands/ranges/ranges.md` | `[x]` |
