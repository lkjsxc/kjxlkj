# Verification: Conformance and Limitations (Iteration 36)

Back: [/docs/todo/current/wave-verification/README.md](/docs/todo/current/wave-verification/README.md)

## Purpose

Keep the “what exists” ledger accurate so target specs are not misread as implemented behavior.

Conformance and limitations updates are required whenever observable behavior changes.

## Checklist (normative)

### A. Update loop for each implemented slice

- [ ] Update [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md) and the relevant conformance sub-docs.
- [ ] Update [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) for any user-visible gaps.
- [ ] Ensure the conformance statement is backed by deterministic tests.

### B. Drift handling

- [ ] If the implementation does not match the target spec, record the divergence explicitly.
- [ ] If behavior is intentionally reduced-scope, record a durable rationale under `/docs/log/proposals/`.

### C. High-priority UX defects

- [ ] Ensure every “High-priority UX defect” listed in [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md) has a PTY-driven E2E regression test.
- [ ] Ensure each defect is either fixed or remains listed as a limitation with an active TODO leaf.

## Related

- Conformance ledger: [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- Known gaps: [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)
