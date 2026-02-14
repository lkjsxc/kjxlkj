# Wave 051: Security and Access Hardening

Back: [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [x] enforce CSRF/session/cookie hardening in production profile
- [x] verify role boundary enforcement across all mutation routes
- [x] harden setup/login/rule-trigger rate limiting

## Verification Tasks

- [x] run security-focused integration suite
- [x] run forbidden and session-expiry E2E scenarios

## Evidence Placeholder

- [x] `Check: CSRF/session/cookie hardening + role-denial + rate-limit security integration coverage`
- [x] `Result: pass`
- [x] `Proof: [/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md](/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md)`
