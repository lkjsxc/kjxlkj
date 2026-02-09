# TODO: Verification

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate 0: Requirement Inventory

- [ ] Build or update a requirements matrix in `/docs/log/reconstruction/audits/`.
- [ ] Assign requirement IDs mapped to exact `/docs/spec/...` links.
- [ ] Mark each requirement as `verified`, `partial`, `scaffold-only`, or `unverified`.

## Gate 1: Implementation Reachability

- [ ] Confirm each claimed feature is reachable from real runtime input paths.
- [ ] Confirm no completion claim is based only on types or stubs.
- [ ] Confirm dispatch wiring exists for each implemented command/key workflow.

## Gate 2: Deterministic Verification

- [ ] Run targeted tests for touched domains.
- [ ] Run full reconstructed verification gate from [/docs/reference/CI.md](/docs/reference/CI.md).
- [ ] Capture result evidence in dated audit logs.

## Gate 3: Reference Synchronization

- [ ] Update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) with evidence-backed status.
- [ ] Update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) for remaining gaps.
- [ ] Update relevant sub-ledgers in [/docs/reference/conformance/README.md](/docs/reference/conformance/README.md).

## Gate 4: TODO Integrity

- [ ] Mark checkboxes complete only when evidence exists.
- [ ] Ensure no checked item conflicts with open limitations.
- [ ] Ensure `/docs/todo/doc-coverage/` reflects current documentation topology.
