# Wave 011: Auth and Session Baseline

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] implement setup lockout for owner bootstrap
- [x] implement login/logout/session with secure cookie rules
- [x] add rate limiting for setup/login

## Verification Tasks

- [x] run `API-AUTH-01` and `API-AUTH-02`
- [x] run negative-path authentication checks

## Evidence Placeholder

- [x] `Check: end-to-end API auth flow (setup lockout, session lifecycle, invalid credential rejection)`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md](/docs/log/audits/2026-02-13-stage-01-workspace-foundation.md)`
