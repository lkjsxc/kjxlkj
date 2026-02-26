# Architecture

Back: [/docs/spec/README.md](/docs/spec/README.md)

## Documents

| Document | Purpose |
|---|---|
| [configuration.md](configuration.md) | config and prompt JSON loading model |
| [runtime.md](runtime.md) | runtime topology and startup order |
| [crates.md](crates.md) | crate decomposition |
| [source-layout.md](source-layout.md) | module and file split constraints |
| [workspace-manifest.md](workspace-manifest.md) | Cargo workspace policy |
| [deployment.md](deployment.md) | runtime process and host deployment contract |
| [completion-file-map.md](completion-file-map.md) | required path map |
| [final-file-structure.md](final-file-structure.md) | docs-only and runtime target trees |
| [BUILD_SEQUENCE.md](BUILD_SEQUENCE.md) | strict reconstruction order and gates |
| [SPEC_INTERACTIONS.md](SPEC_INTERACTIONS.md) | cross-spec dependency graph |

## Invariants

- request handling MUST be async and non-blocking
- note mutation ordering MUST be deterministic per note stream
- agent and automation mutation ordering MUST be deterministic
- docs-only baseline and runtime target are both first-class states
- every TODO wave MUST map to this architecture set before runtime work starts

## Related

- Domain model: [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
- Technical contracts: [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
- TODO execution: [/docs/todo/README.md](/docs/todo/README.md)
