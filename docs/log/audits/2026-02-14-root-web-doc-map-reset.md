# 2026-02-14 Root Web, Doc Map, and TODO Reset

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Deliver a usable root web app, remove remaining API stubs, reset TODO checkboxes,
and rebuild documentation linkage so every documentation file is reachable from
TODO-driven execution.

## Scope

- implemented root web shell delivery (`/`, `/app/main.js`, `/app/styles.css`)
- added setup status endpoint and usable setup/login/workspace flows in frontend
- replaced dashboard/attachment/admin stub endpoints with executable handlers
- added backend and frontend deterministic tests for new paths
- consumed prior improvement-note ideas into TODO/spec/reference updates
- deleted prior `docs/log/improvements/` note set
- introduced `/docs/todo/doc-map/` with full documentation-link coverage

## Deterministic Evidence

### Check 1: backend integration tests

Command:

`cargo test -p kjxlkj-server`

Result:

- pass (`9 passed; 0 failed`)

### Check 2: frontend regression suite

Command:

`npm run -w src/frontend/app test`

Result:

- pass (`8 passed; 0 failed`)

### Check 3: frontend strict type gate

Command:

`npm run typecheck`

Result:

- pass (`tsc --noEmit`)

### Check 4: runtime source file line-size scan

Command:

`find src -path '*/node_modules' -prune -o -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 200 ]; then printf "%s:%s\n" "$1" "$lines"; fi' _ {} \;`

Result:

- no output (no runtime source files exceed 200 lines)

### Check 5: TODO unchecked-row scan

Command:

`grep -n "\[ \]" docs/todo/README.md`

Result:

- unchecked rows remain by design after reset baseline
- continuation is required through wave execution

## Outcome

- root web app is directly usable from site root
- dashboard/attachment/admin paths are no longer stub handlers
- TODO program is reset and doc-map linked to full documentation set
- prior standalone improvement-note files were removed after incorporation
