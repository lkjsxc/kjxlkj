# TODO: Verification

Back: [/docs/todo/current/README.md](/docs/todo/current/README.md)

## Gate 0: Baseline Audit

- [ ] Run verification commands from [/docs/reference/CI.md](/docs/reference/CI.md)
- [ ] Build mismatch matrix (M1-M5) in `/docs/log/reconstruction/audits/`
- [ ] Prioritize by user impact and regression risk

## Gate 1: Slice Definition

- [ ] Select one coherent implementation slice
- [ ] Attach exact spec links for every selected behavior
- [ ] Define unit/integration/E2E/PTy coverage required for slice

## Gate 2: Implementation Integrity

- [ ] Ensure feature is reachable from runtime entrypoint
- [ ] Ensure no source file exceeds 200 lines after implementation
- [ ] Ensure source directory fan-out remains around 12 children
- [ ] Ensure terminal/explorer/window behavior is integrated as real windows

## Gate 3: Verification and Sync

- [ ] Run touched tests
- [ ] Run full `cargo test --workspace`
- [ ] Record proof of 523+ listed tests or updated total with command evidence
- [ ] Update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [ ] Update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Gate 4: Critical UX Regressions

- [ ] `A` / `Shift+a` append-at-EOL behavior is verified by tests
- [ ] Long lines wrap on-screen with no off-screen overflow
- [ ] Japanese IME composition does not leak into leader mappings
- [ ] Terminal spawn/resize/close lifecycle works in real PTY path
- [ ] Explorer open/split workflows are wired and test-covered

## Gate 5: TODO Integrity

- [ ] Every checked item has concrete evidence lines
- [ ] No checked item depends on known open limitation
- [ ] `/docs/todo/doc-coverage/` contains direct links to every documentation file
- [ ] All TODO changes remain unchecked unless explicitly proven complete

## Related

- Testing contract: [/docs/spec/technical/testing.md](/docs/spec/technical/testing.md)
- Boundary matrix: [/docs/spec/technical/testing-e2e.md](/docs/spec/technical/testing-e2e.md)
- Spec-code sync audit template: [/docs/log/reconstruction/audits/2026-02-09-doc-sync-matrix.md](/docs/log/reconstruction/audits/2026-02-09-doc-sync-matrix.md)
