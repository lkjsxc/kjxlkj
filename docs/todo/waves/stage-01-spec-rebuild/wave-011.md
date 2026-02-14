# Wave 011: Auth, Session, and Setup Lock Baseline

Back: [/docs/todo/waves/stage-01-spec-rebuild/README.md](/docs/todo/waves/stage-01-spec-rebuild/README.md)

## Relevant Documents

- [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md)
- [/docs/spec/security/transport.md](/docs/spec/security/transport.md)
- [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)

## Restructure Steps

- [ ] restructure-step S01-W011-01: implement setup/register and setup-lock behavior from [/docs/spec/api/http.md](/docs/spec/api/http.md)
- [ ] restructure-step S01-W011-02: implement login/logout/session semantics from [/docs/spec/security/auth.md](/docs/spec/security/auth.md)
- [ ] restructure-step S01-W011-03: enforce cookie/session lifecycle from [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- [ ] restructure-step S01-W011-04: enforce CSRF and transport boundaries from [/docs/spec/security/csrf.md](/docs/spec/security/csrf.md) and [/docs/spec/security/transport.md](/docs/spec/security/transport.md)
- [ ] restructure-step S01-W011-05: enforce setup-locked login-only UI behavior from [/docs/spec/ui/web-app.md](/docs/spec/ui/web-app.md)

## Verification Hooks

- [ ] restructure-step S01-W011-V01: run auth/session acceptance checks from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [ ] restructure-step S01-W011-V02: synchronize auth state in [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) and [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
