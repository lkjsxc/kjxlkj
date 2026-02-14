# 2026-02-14 Docs-Only Reset

Back: [/docs/log/audits/README.md](/docs/log/audits/README.md)

## Objective

Execute a full docs-first reset where documentation is canonical and all runtime
source/related artifacts are removed.

## Input References Used Then Retired

Improvement notes were used as rewrite input and remain deleted from repository:

- `docs/log/improvements/2026/2026-02-14-automation-librarian-followups.md`
- `docs/log/improvements/2026/2026-02-14-http-ws-followups.md`
- `docs/log/improvements/2026/2026-02-14-reconstruction-execution-notes.md`

## Canonical Documentation Changes

1. Rewrote policy/spec/reference contracts for docs-only canonical completion.
2. Rewrote final structure contract to include:
   - canonical docs-only completion tree
   - optional derived runtime projection tree
3. Added explicit responsive UX contract:
   - desktop (`>=1024px`): note list left, editor right
   - compact (`<1024px`): editor primary, top-left menu reveals note list
4. Added JSON-only librarian prompt-pack contract and canonical prompt files:
   - `manifest.json`
   - `stage-ingest.json`
   - `stage-plan.json`
   - `stage-propose.json`
   - `stage-validate-repair.json`
5. Rebuilt TODO linkage so every checkbox row links directly to docs.

## Repository Reset Actions

- Removed runtime tree: `src/`
- Removed runtime root manifests/artifacts:
  - `Cargo.toml`, `Cargo.lock`
  - `package.json`, `package-lock.json`, `tsconfig.json`
  - `Dockerfile`, `docker-compose.yml`, `.dockerignore`
- Removed local build/dependency directories:
  - `node_modules/`
  - `target/`

## Deterministic Checks

- `Check:` `rg -n "^- \[[ xX]\]" docs/todo | rg -v "\]\("`
  - `Result:` pass
  - `Proof:` no TODO checkbox rows without direct links

- `Check:` `grep -R -n "\[ \]" docs/todo | head`
  - `Result:` pass
  - `Proof:` unchecked rows remain, confirming reset baseline

- `Check:` `find docs -type f -name '*.md' -exec sh -c 'for f in "$@"; do n=$(wc -l < "$f"); if [ "$n" -gt 200 ]; then printf "%s:%s\n" "$f" "$n"; fi; done' sh {} +`
  - `Result:` pass
  - `Proof:` no docs markdown file exceeds 200 lines

- `Check:` `find src -type f \( -name '*.rs' -o -name '*.ts' -o -name '*.tsx' \) -exec sh -c 'for f in "$@"; do n=$(wc -l < "$f"); if [ "$n" -gt 200 ]; then printf "%s:%s\n" "$f" "$n"; fi; done' sh {} + 2>/dev/null || true`
  - `Result:` pass
  - `Proof:` no source files are present in docs-only baseline

- `Check:` `test ! -d src && test ! -f Cargo.toml && test ! -f package.json && test ! -f Dockerfile`
  - `Result:` pass
  - `Proof:` runtime source and related root artifacts are absent

## Follow-On

Reconstruction starts from [/docs/todo/README.md](/docs/todo/README.md) and waves
under [/docs/todo/waves/README.md](/docs/todo/waves/README.md).
