# Log: Files Over 200 Lines

Updated automatically. Files exceeding the 200-line policy limit.

## Current violations

| File | Lines | Status |
|------|-------|--------|
| editor_range_cmds.rs | 293 | Needs split |
| dispatch_tests.rs | 273 | Test file (exempt) |
| editor_op_resolve.rs | 267 | Needs split |
| editor.rs | 264 | Needs split |
| action.rs | 254 | Enum definition (acceptable) |
| completion.rs | 248 | Needs split |
| floating.rs | 234 | Needs split |
| editor_tabs.rs | 224 | Needs split |
| editor_window_adv.rs | 223 | Needs split |
| normal_g_z.rs | 221 | Needs split |
| editor_auto_marks.rs | 211 | Needs split |
| autocmd.rs | 204 | Needs split |
| filetype.rs | 202 | Minor |
| text_object_exec.rs | 202 | Minor |
| file_explorer.rs | 201 | Minor |

## Resolved

| File | Action |
|------|--------|
| command_dispatch.rs | Split from 429 to 3 files (114+106+100+167) |
