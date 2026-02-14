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

- [ ] enforce CSRF/session/cookie hardening in production profile -> [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [ ] verify role boundary enforcement across all mutation routes -> [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [ ] harden setup/login/rule-trigger rate limiting -> [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Verification Tasks

- [ ] run security-focused integration suite -> [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [ ] run forbidden and session-expiry E2E scenarios -> [/docs/spec/security/README.md](/docs/spec/security/README.md)

## Evidence Placeholder

- [ ] `Check: CSRF/session/cookie hardening + role-denial + rate-limit security integration coverage` -> [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [ ] `Result: pass` -> [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [ ] `Proof: [/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md](/docs/log/audits/2026-02-13-stage-05-wave-051-security-hardening.md)`
