# File Size Compliance - 2026-02-13

## Files Exceeding 200 Lines

The following source files exceed the 200-line limit and should be refactored:

| File | Lines | Recommended Action |
|------|-------|-------------------|
| `src/crates/db/kjxlkj-db/src/notes.rs` | 357 | Split into separate repos: NoteRepo, NoteHistoryRepo, NoteMetadataRepo, NoteTagRepo, BacklinkRepo |
| `src/crates/db/kjxlkj-db/src/workspaces.rs` | 278 | Split into separate repos: WorkspaceRepo, MembershipRepo, ProjectRepo, SavedViewRepo |
| `src/crates/automation/kjxlkj-automation/src/lib.rs` | 250 | Split into modules: provider, parser, executor |
| `src/crates/db/kjxlkj-db/src/automation.rs` | 230 | Split into AutomationRuleRepo and AutomationRunRepo |
| `src/crates/db/kjxlkj-db/src/users.rs` | 225 | Split into UserRepo, SessionRepo, SecurityEventRepo |
| `src/crates/http/kjxlkj-http/src/dto.rs` | 218 | Group DTOs into submodules by domain |
| `src/crates/domain/kjxlkj-domain/src/automation.rs` | 204 | Split into rule.rs, run.rs, operation.rs |

## Status

- [ ] Refactor notes.rs
- [ ] Refactor workspaces.rs
- [ ] Refactor automation lib.rs
- [ ] Refactor db automation.rs
- [ ] Refactor users.rs
- [ ] Refactor dto.rs
- [ ] Refactor domain automation.rs
