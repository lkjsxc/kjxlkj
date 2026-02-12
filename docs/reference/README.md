# Reference Documentation

Back: [/docs/README.md](/docs/README.md)

`/docs/reference/` is the canonical truth for current verified state.

## Authority

For current-state claims, use this precedence:

1. [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
2. [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
3. [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
4. [/docs/reference/CI.md](/docs/reference/CI.md)
5. [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md)

`/docs/spec/` remains the target behavior contract.

## Snapshot (2026-02-12)

The repository is in reconstruction-prep mode.

- Documentation is canonical.
- Existing source code is non-authoritative and may be discarded.
- Runtime behavior must not be treated as verified unless explicitly proven.

## Evidence Rules

- strongest reproducible evidence wins
- user-reported runtime failure outranks stale passing tests
- blocker rows close only with deterministic tests and matching PTY `*R` evidence
- reference, TODO, and spec updates must stay synchronized

## Documents

| Document | Role |
|---|---|
| [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) | current verified/unverified status by domain |
| [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) | open user-visible blockers and closure requirements |
| [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) | requirement-level mismatch matrix |
| [/docs/reference/CI.md](/docs/reference/CI.md) | verification profile definitions |
| [/docs/reference/RELEASE.md](/docs/reference/RELEASE.md) | release gate and evidence rules |
| [/docs/reference/conformance/README.md](/docs/reference/conformance/README.md) | consolidation policy |
| [/docs/reference/COMPARISON.md](/docs/reference/COMPARISON.md) | non-normative comparison framing |
| [/docs/reference/PLUGIN_MAPPING.md](/docs/reference/PLUGIN_MAPPING.md) | non-normative plugin mapping framing |

## Synchronization Rule

Whenever a blocker status changes, update all of:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/todo/README.md](/docs/todo/README.md)

in one logical change.

## Related

- target behavior: [/docs/spec/README.md](/docs/spec/README.md)
- reconstruction execution: [/docs/todo/README.md](/docs/todo/README.md)
