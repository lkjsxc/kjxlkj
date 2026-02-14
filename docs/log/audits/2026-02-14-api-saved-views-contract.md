# 2026-02-14 API Saved Views Contract

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Replace saved-view API stubs with executable CRUD behavior and deterministic
verification.

## Implementation Scope

- Added saved-view domain model (`SavedViewRecord`) and in-memory store binding.
- Implemented handlers for `GET/POST /api/views` and
  `PATCH/DELETE /api/views/{id}`.
- Enforced auth + csrf + role checks for mutating view operations.
- Added deterministic integration test for full saved-view lifecycle.

## Deterministic Checks

Command:

`cargo test -p kjxlkj-server -- --nocapture`

Result:

- pass (`tests_views::api_saved_view_crud_contract` plus existing suites)

Command:

`cargo check --workspace`

Result:

- pass

Command:

`npm run typecheck`

Result:

- pass

Command:

`find src -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 200 ]; then printf "%s:%s\n" "$1" "$lines"; fi' _ {} \;`

Result:

- no output (all runtime source files are <=200 lines)

## Ledger Impact

- conformance HTTP/API row remains `partial`, now explicitly including saved-view
  lifecycle verification.
- limitations API gap narrows from broad view stubs to remaining attachment/admin
  and broader acceptance-pack incompleteness.
