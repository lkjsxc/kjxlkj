# Restructuring Program TOC

This section defines a deterministic documentation-restructuring program.

## Scope Contract

- This program changes contracts under `docs/restructuring/` and aligned README links.
- Runtime, test, and CLI code are audited for convergence, even when they are not edited directly.
- Execution order is fixed and must remain deterministic for repeated replay.
- Any docs/runtime/tests/CLI mismatch blocks completion until contracts and evidence are realigned.

## Directory TOC

| Directory | Purpose |
| --- | --- |
| [phases/README.md](phases/README.md) | Ordered phase contracts (`00`..`09`) with interleaved test gates |
| [tests/README.md](tests/README.md) | Fundamental intent catalog and deterministic interleaved test schedule |
| [coverage/README.md](coverage/README.md) | Coverage controls and full markdown reference matrix |
| [runtime-test-wave-evidence.md](runtime-test-wave-evidence.md) | Deterministic docs+runtime+tests+CLI convergence contract and replay evidence |

## Deterministic Convergence Gates

Run this ordered gate sequence on every replay:

1. `cargo run --bin kjxlkj -- docs validate-topology`
2. `cargo run --bin kjxlkj -- docs validate-terms`
3. `cargo run --bin kjxlkj -- quality check-lines`
4. `cargo test -q`
5. `docker compose --profile verify run --rm verify`
6. `cargo run --bin kjxlkj -- compose verify` (optional wrapper; success summary must report `"steps_total":4`)

If any gate fails, stop and restart from gate 1 after fixing the failing contract surface.

## Deterministic Reading Order

1. [phases/README.md](phases/README.md)
2. [tests/fundamental-intent-catalog.md](tests/fundamental-intent-catalog.md)
3. [tests/interleaved-schedule.md](tests/interleaved-schedule.md)
4. [coverage/matrix.md](coverage/matrix.md)
5. [runtime-test-wave-evidence.md](runtime-test-wave-evidence.md)
