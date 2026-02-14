# Stage 01: Workspace and Auth Foundation

Back: [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Relevant Documents

- [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
- [/docs/spec/architecture/workspace-manifest.md](/docs/spec/architecture/workspace-manifest.md)
- [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [/docs/spec/domain/workspaces.md](/docs/spec/domain/workspaces.md)
- [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)

## Stage Objective

Build a typed runtime skeleton with deterministic auth/session and RBAC
foundations that later stages can safely extend.

## Ordered Wave Checklist

- [x] restructure-step S01-W010: complete runtime topology and crate skeleton in [wave-010.md](wave-010.md)
- [x] restructure-step S01-W011: complete auth/session and setup-lock behavior in [wave-011.md](wave-011.md)
- [x] restructure-step S01-W012: complete role/membership enforcement baseline in [wave-012.md](wave-012.md)

## Stage Exit Checklist

- [x] restructure-step S01-EXIT-01: runtime skeleton matches [/docs/spec/architecture/source-layout.md](/docs/spec/architecture/source-layout.md)
- [x] restructure-step S01-EXIT-02: auth/session contracts match [/docs/spec/security/auth.md](/docs/spec/security/auth.md) and [/docs/spec/security/sessions.md](/docs/spec/security/sessions.md)
- [x] restructure-step S01-EXIT-03: role checks align with [/docs/spec/domain/permissions.md](/docs/spec/domain/permissions.md)
