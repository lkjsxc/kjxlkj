# Wave 000: API and WS Canonical Reset

Back: [/docs/todo/waves/stage-00-pivot-governance/README.md](/docs/todo/waves/stage-00-pivot-governance/README.md)

## Relevant Documents

- [/docs/spec/README.md](/docs/spec/README.md)
- [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
- [/docs/todo/README.md](/docs/todo/README.md)
- [/docs/todo/waves/README.md](/docs/todo/waves/README.md)

## Implementation Tasks

- [ ] set API base path to `/api` in canonical docs
- [ ] set WS endpoint to `/ws` in canonical docs
- [ ] align OpenAPI document with HTTP and WS contracts
- [ ] enforce `DELETE /notes/{id}` soft-delete path consistency

## Verification Tasks

- [ ] link-check `api/` docs and OpenAPI references
- [ ] verify no remaining version-labeled API paths in canonical docs

## Evidence Placeholder

- [ ] `Check: deterministic Stage 00 validation pack for API/WS/OpenAPI contract parity and path rules`
- [ ] `Result: pass`
- [ ] `Proof: [/docs/log/audits/2026-02-12-stage-00-canonical-reset.md](/docs/log/audits/2026-02-12-stage-00-canonical-reset.md)`
