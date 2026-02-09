# Oversized Source Files

Back: [/docs/log/README.md](/docs/log/README.md)

Files exceeding the 200-line target per [/docs/policy/STRUCTURE.md](/docs/policy/STRUCTURE.md).

## Non-splittable (enum/struct definitions)

| File | Lines | Reason |
|------|-------|--------|
| action.rs (core-types) | 312 | Single enum with 90+ variants; cannot split in Rust |
| editor.rs (core-state) | 296 | Struct with 80+ fields; constructor extracted to editor_init.rs |

## Splittable (impl + tests)

| File | Lines | Plan |
|------|-------|------|
| editor_op_resolve.rs | 267 | Extract test module |
| completion.rs | 248 | Extract test module |
| floating.rs | 234 | Extract test module |
| snippets.rs | 232 | Extract test module |
| editor_tabs.rs | 224 | Extract test module |
| editor_window_adv.rs | 223 | Extract test module |
| folds_advanced.rs | 221 | Extract test module |
| normal_g_z.rs (core-mode) | 221 | Extract test module |
| session_features.rs | 218 | Extract structs to separate file |
| buffer_options.rs | 218 | Extract test module |
| theming.rs | 214 | Extract test module |
| editor_auto_marks.rs | 211 | Extract test module |
| autocmd.rs | 204 | Marginal; tests are small |
| filetype.rs | 202 | Marginal; language list is inherently long |
| text_object_exec.rs (core-edit) | 202 | Marginal |
| file_explorer.rs | 201 | Marginal |
| auto_session.rs | 201 | Marginal |

## Already split

| Original | Extracted | Lines saved |
|----------|-----------|-------------|
| editor.rs | editor_types.rs, editor_init.rs | ~120 |
| editor_range_cmds.rs | editor_range_parse.rs | ~140 |

## Notes

- Files 200-210 lines are marginal and may not benefit from splitting
- Enum/struct files cannot be split in Rust without major restructuring
- Test extraction is the easiest split path for impl+tests files
