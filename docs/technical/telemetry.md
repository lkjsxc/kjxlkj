# Telemetry (Design and Policy)

Back: [/docs/technical/README.md](/docs/technical/README.md)
Telemetry is optional. This document defines the project’s posture and the constraints any telemetry implementation must obey.

Status note: the current shipped surface is recorded in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md). If telemetry is not implemented, that is not a bug; it should simply remain absent and default-off.

## Philosophy (normative)

- Opt-in only (no silent enablement).
- Privacy-first (minimize data; minimize retention; minimize identifiers).
- Transparent (users can inspect exactly what would be sent).
- Local-first (no network activity unless explicitly enabled).

## Hard prohibitions (normative)

Telemetry MUST NOT include:

- file contents
- full file paths
- user-identifying information (names, emails, tokens)
- clipboard contents

If any identifier is required for deduplication, it MUST be ephemeral and scoped to the local machine unless the user explicitly opts into something stronger.

## Allowed data (target)

Telemetry, when enabled, SHOULD be limited to coarse aggregate signals:

| Category | Examples | Purpose |
|---|---|---|
| Feature usage | command/mode counts | prioritize development |
| Performance | coarse timings and sizes | find regressions |
| Stability | crash and error rates | improve reliability |
| Environment | OS + terminal family | compatibility work |

If “performance metrics” are collected, they MUST avoid leaking sensitive text (e.g., never send raw buffer lines).

## User controls (target)

When telemetry exists, users SHOULD be able to:

- enable/disable it explicitly
- view pending events locally before send
- delete local telemetry data

## Storage and transmission (target)

Recommended posture:

- store a bounded local queue (cap by size and/or count)
- batch transmission (never per-keystroke networking)
- use authenticated TLS to a documented endpoint

Any network endpoint and retention policy MUST be documented inside `/docs/` (do not rely on external links).

## Documentation obligations (normative)

If telemetry is implemented:

- record it in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- record user-visible caveats in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- document the exact schema and redaction rules in `/docs/` (this file or a linked spec file)
