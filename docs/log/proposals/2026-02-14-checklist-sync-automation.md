# Proposal: Checklist Synchronization Automation

Back: [/docs/log/proposals/README.md](/docs/log/proposals/README.md)

## Date

2026-02-14

## Decision

Introduce deterministic automation for documentation checklist synchronization so
TODO/doc-map/reference drift is detected and corrected in smaller increments.

## Scope

- add a deterministic script that validates all `docs/todo` link targets
- add a script to compare wave-file completion against top-level TODO gate rows
- gate docs PRs on no stale `unchecked-baseline` phrasing when checklist rows are
  already checked
- record script outputs in audit notes for every checklist reconciliation change

## Rationale

- reduce checklist drift between wave files and top-level TODO
- avoid stale state statements in `reference` after checklist updates
- keep docs-only truthfulness while still enabling rapid reconstruction replay

## Follow-Up

Track implementation as a governance hardening slice under
[/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md).
