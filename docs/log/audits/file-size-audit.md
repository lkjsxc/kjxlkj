# File Size Audit

All source files verified under 200-line limit.

## Last Audit

Date: 2026-02-10 (post gap-closure)

### Largest Files (top 10)

| File | Lines |
|------|-------|
| kjxlkj-core-edit/src/insert_ops.rs | 198 |
| kjxlkj-core-edit/src/edit_tests.rs | 196 |
| kjxlkj-core-state/src/tests/gap_tests.rs | 195 |
| kjxlkj-service-index/src/service.rs | 191 |
| kjxlkj-service-lsp/src/client.rs | 190 |
| kjxlkj-core-mode/src/mode_tests.rs | 186 |
| kjxlkj-core-state/src/ops/editing_ops.rs | 183 |
| kjxlkj-core-mode/src/normal.rs | 182 |
| kjxlkj-core-state/src/tests/wiring_tests.rs | 179 |
| kjxlkj-core-state/src/explorer.rs | 117 |

### Files Over 200 Lines

None.

## Improvement Ideas

- insert_ops.rs (198) is near the limit; if new insert operations are added, consider splitting char-insert vs line-insert ops.
- edit_tests.rs (196) is near the limit; group tests into separate files if more editing tests are needed.
- gap_tests.rs (195) covers 11 tests across 7 gaps; if more gap tests are added, split by feature domain.
- register.rs was 254 lines before tests were extracted to register_tests.rs.
- explorer_ops.rs was 248 lines before ExplorerState was moved to explorer.rs.
