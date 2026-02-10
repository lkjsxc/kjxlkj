# Reconstruction Drift Baseline (2026-02-10)

Back: [/docs/log/reconstruction/audits/README.md](/docs/log/reconstruction/audits/README.md)

## Summary

Canonical drift tracking moved to:

- [/docs/reference/DRIFT_MATRIX.md](/docs/reference/DRIFT_MATRIX.md)
- [/docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md)
- [/docs/reference/LIMITATIONS.md](/docs/reference/LIMITATIONS.md)

## Open High-Risk Gaps

- terminal launch and PTY lifecycle wiring
- explorer launch and split-open wiring
- `Shift+a` normalization and `a` at EOL semantics
- Japanese IME composition and leader isolation
- long-line on-screen wrapping enforcement
