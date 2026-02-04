# Crash Reporting (Design)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Crash reporting is optional. This document defines what “good crash reporting” should look like for kjxlkj if/when implemented.

Status note: current shipped behavior is tracked in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md). If crash reporting is not implemented, this document serves as implementation guidance for reconstruction.

## Goals (normative)

- Prefer not crashing: panics should be treated as bugs.
- When a crash does occur, maximize diagnosability without leaking user data.
- Make it easy for users to report issues with a structured artifact.
- Attempt to restore terminal state before exit (raw mode / alternate screen safety).

## Report contents (target)

Crash reports SHOULD contain:

| Field | Why |
|---|---|
| build/version identifier | reproducibility |
| OS + architecture | platform-specific bugs |
| terminal family (best-effort) | rendering/input issues |
| panic message | root cause hint |
| backtrace (if available) | actionable stack context |
| recent editor intents (sanitized) | minimal “what happened” trail |

Crash reports MUST NOT contain:

- buffer contents
- full file paths
- secrets (tokens, keys)
- clipboard contents

If any “recent actions” are included, they MUST be a redacted, high-level log (e.g., command names, not typed text).

## Storage location (target)

Reports SHOULD be stored locally in a user-data directory appropriate for the platform.

Recommended posture:

- a per-user directory under the platform’s standard application state/data location
- bounded retention (cap number of stored reports)
- filenames include a timestamp and a short random suffix to avoid collisions

## Submission (target)

- No automatic submission unless explicitly opted-in.
- Manual workflow should be easy: “open report file, paste into issue”.

If network submission is ever added, it must follow the same opt-in/privacy posture defined in:

- [/docs/technical/telemetry.md](/docs/technical/telemetry.md)

## Recovery (target)

If session persistence is implemented, the editor MAY offer a “restore previous session” flow after a crash.

Until sessions exist, crash recovery should focus on:

- not corrupting files on disk
- preserving unsaved work when possible (e.g., emergency write to a recovery file)

Related: [/docs/technical/error-recovery.md](/docs/technical/error-recovery.md)
