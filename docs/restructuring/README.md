# Restructuring Program TOC

This section defines a deterministic documentation-restructuring program.

## Scope Contract

- This program only changes documentation under `docs/` and root `README.md` links.
- Runtime implementation code is intentionally out of scope.
- Execution order is fixed and must remain deterministic for LLM readability.
- Runtime convergence is validated through linked operations/quality contracts, not by changing this program scope.

## Directory TOC

| Directory | Purpose |
| --- | --- |
| [phases/README.md](phases/README.md) | Ordered phase contracts (`00`..`09`) with interleaved test gates |
| [tests/README.md](tests/README.md) | Fundamental intent catalog and deterministic interleaved test schedule |
| [coverage/README.md](coverage/README.md) | Coverage controls and full markdown reference matrix |
| [runtime-test-wave-evidence.md](runtime-test-wave-evidence.md) | Runtime/container verification and final validation evidence for persistent-runtime closure |

## Deterministic Reading Order

1. [phases/README.md](phases/README.md)
2. [tests/fundamental-intent-catalog.md](tests/fundamental-intent-catalog.md)
3. [tests/interleaved-schedule.md](tests/interleaved-schedule.md)
4. [coverage/matrix.md](coverage/matrix.md)
5. [runtime-test-wave-evidence.md](runtime-test-wave-evidence.md)
