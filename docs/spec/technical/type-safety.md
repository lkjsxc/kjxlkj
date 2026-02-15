# Type Safety Contract

Back: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)

## Language Policy

- Backend runtime MUST be Rust.
- Frontend runtime MUST be TypeScript.
- Handwritten runtime JavaScript source is forbidden.

## Backend Requirements

- `cargo check --workspace` MUST pass.
- public payloads MUST use typed structs/enums.
- agent prompt JSON parsing MUST use typed schema models.

## Frontend Requirements

- `strict: true` in `tsconfig.json` is mandatory.
- `any` is forbidden in app-domain logic.
- search and editor contracts MUST use explicit interfaces.

## Verification Gate

1. backend compile gate
2. frontend type-check gate
3. no handwritten runtime JS

## Related

- Testing: [testing.md](testing.md)
- Agent prompt schema: [agent-prompt-json.md](agent-prompt-json.md)
