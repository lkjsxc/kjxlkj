# Wave 001: Cross-Spec Coherence Reset

Back: [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/api/README.md](/docs/spec/api/README.md)
- [/docs/spec/architecture/README.md](/docs/spec/architecture/README.md)
- [/docs/spec/domain/README.md](/docs/spec/domain/README.md)
- [/docs/spec/security/README.md](/docs/spec/security/README.md)
- [/docs/spec/technical/README.md](/docs/spec/technical/README.md)
- [/docs/spec/ui/README.md](/docs/spec/ui/README.md)

## Restructure Steps

- [ ] restructure-step S00-W001-01: synchronize leaf-document links across spec indexes in [/docs/spec/README.md](/docs/spec/README.md) [doc-link](/docs/spec/README.md)
- [ ] restructure-step S00-W001-02: enforce HTTP/WS/type/error/openapi consistency from [/docs/spec/api/http.md](/docs/spec/api/http.md), [/docs/spec/api/types.md](/docs/spec/api/types.md), [/docs/spec/api/errors.md](/docs/spec/api/errors.md), and [/docs/spec/api/openapi.md](/docs/spec/api/openapi.md) [doc-link](/docs/spec/api/http.md)
- [ ] restructure-step S00-W001-03: enforce JSON-only prompt source policy from [/docs/spec/technical/librarian-prompts/README.md](/docs/spec/technical/librarian-prompts/README.md) [doc-link](/docs/spec/technical/librarian-prompts/README.md)
- [ ] restructure-step S00-W001-04: enforce responsive split/menu UX requirements from [/docs/spec/ui/layout-and-interaction.md](/docs/spec/ui/layout-and-interaction.md) [doc-link](/docs/spec/ui/layout-and-interaction.md)
- [ ] restructure-step S00-W001-05: enforce findings-to-acceptance mapping in [/docs/spec/ui/findings-traceability.md](/docs/spec/ui/findings-traceability.md) [doc-link](/docs/spec/ui/findings-traceability.md)

## Verification Hooks

- [ ] restructure-step S00-W001-V01: run spec contradiction scan and resolve through [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md) [doc-link](/docs/reference/DRIFT_MATRIX.md)
- [ ] restructure-step S00-W001-V02: confirm acceptance IDs remain authoritative in [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) [doc-link](/docs/spec/technical/testing.md)

## Mandatory Build and Test Gate

- [ ] run wave build gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo build --workspace`
- [ ] run wave test gate from [/docs/reference/CI.md](/docs/reference/CI.md): `cargo test --workspace`
- [ ] run wave acceptance IDs from [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md) and archive proof in [/docs/reference/EVIDENCE_INDEX.md](/docs/reference/EVIDENCE_INDEX.md)
