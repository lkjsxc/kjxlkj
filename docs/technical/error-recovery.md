# Error Recovery (Guidance)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Guidance for handling failures without losing user work or corrupting files.

Status note: this document defines the desired posture. Current surface and gaps are tracked in:

- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Philosophy (normative)

- Never silently lose user edits.
- Prefer “keep the buffer intact in memory” over “try to be clever”.
- Surface errors with enough context to act, without leaking sensitive data.
- Make recovery paths testable (prefer headless/E2E repro scripts).

## Error categories and required posture

| Category | Examples | Required posture |
|---|---|---|
| Input/decode | invalid key sequences | ignore or report; never panic |
| File open/read | permission denied, transient IO | keep editor usable; show error; do not corrupt state |
| File write | disk full, permission denied | keep buffer dirty; show error; offer alternate write |
| Service failures | server crash, timeout | isolate; restartable; do not corrupt core |
| Resource exhaustion | OOM | best-effort emergency save; fail loudly |

## File open and missing paths (target)

Recommended behavior:

- missing path: treat as “new file” and open an empty buffer bound to that path
- other open errors: do not destroy existing buffers; show an actionable message

If open errors prevent startup entirely, the failure should still restore the terminal and produce a crash report when possible.

## Safe writing (target)

The editor should avoid partial writes and corruption:

- use a temp-file + atomic replace strategy when supported
- on any write failure, do not clear the dirty flag
- allow “write to alternate path” as an escape hatch

Related: [/docs/technical/network-fs.md](/docs/technical/network-fs.md)

## User notification (target)

Errors should be visible and actionable.

Suggested severity model:

| Level | Typical surface |
|---|---|
| Info | statusline/message area |
| Warning | persistent message until acknowledged |
| Error | persistent + includes next action suggestion |
| Critical | blocks destructive actions until resolved |

## Crash paths (target)

If the process panics:

- attempt to restore terminal state before exit
- create a local crash report (if implemented)

Related: [/docs/technical/crash-reporting.md](/docs/technical/crash-reporting.md)

## Testing recovery (recommended)

Recovery behavior should be covered by tests that:

- simulate write failures (permission denied, disk full) and assert dirty state remains
- simulate open failures and assert the editor stays usable
- ensure panics do not permanently break the terminal in common workflows
