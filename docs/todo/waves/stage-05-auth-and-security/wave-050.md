# Wave 050: Security Hardening Baseline

Back: [/docs/todo/waves/stage-05-auth-and-security/README.md](/docs/todo/waves/stage-05-auth-and-security/README.md)

## Relevant Documents

- [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md)
- [/docs/spec/security/transport.md](/docs/spec/security/transport.md)
- [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
- [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Restructure Steps

- [x] restructure-step S05-W050-01: enforce authentication boundaries from [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- [x] restructure-step S05-W050-02: enforce secure session cookie and expiry semantics from [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- [x] restructure-step S05-W050-03: enforce CSRF validation policy from [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md)
- [x] restructure-step S05-W050-04: enforce transport-level security and header policy from [/docs/spec/security/transport.md](/docs/spec/security/transport.md)
- [x] restructure-step S05-W050-05: enforce role-denied error semantics from [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md) and [/docs/spec/api/errors.md](/docs/spec/api/errors.md)

## Verification Hooks

- [x] restructure-step S05-W050-V01: run security acceptance checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [x] restructure-step S05-W050-V02: sync security status in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
