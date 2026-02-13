# Type Safety Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

Mandatory language and typing constraints for reconstructed runtime artifacts.

## Language Policy

- Backend runtime MUST be implemented in Rust.
- Frontend runtime MUST be implemented in TypeScript.
- Direct JavaScript runtime source MUST NOT be committed.

## Backend Requirements (Rust)

- Compilation MUST pass with `cargo check --workspace`.
- Public API payloads MUST use explicit typed structs/enums.
- Fallible operations MUST return typed error variants.

## Frontend Requirements (TypeScript)

- `strict: true` in `tsconfig.json` is mandatory.
- `any` is forbidden in app-domain logic.
- API and WS payloads MUST be represented by explicit interfaces/types.

## Contract Surface

Typed contracts MUST exist for:

- auth/session payloads
- note/workspace/project models
- automation/librarian payloads
- websocket message envelopes

## Verification Gate

A runtime claim is invalid unless all pass:

1. backend compile gate
2. frontend type-check gate
3. no direct JavaScript runtime source

## Related

- Architecture runtime: [/docs/spec/architecture/runtime.md](/docs/spec/architecture/runtime.md)
- API types: [/docs/spec/api/types.md](/docs/spec/api/types.md)
- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
