# Built-In Integrations Policy

Back: [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)

`kjxlkj` does not support loading external plugins at runtime.

## Normative Rules

- Features are implemented as built-in crates/modules.
- Dynamic plugin loading is out of scope.
- No remote plugin marketplace is supported.
- No runtime code execution from plugin files is supported.

## Extension Strategy

Feature growth happens through first-party modules and services:

- editor core modules
- service crates (FS/Git/Index/LSP/Terminal)
- shared UX conventions and command/key surfaces

## Rationale

- deterministic behavior
- simpler security model
- consistent UX
- reproducible builds

## Related

- Principles: [/docs/spec/overview/principles.md](/docs/spec/overview/principles.md)
- Crate topology: [/docs/spec/architecture/crates.md](/docs/spec/architecture/crates.md)
