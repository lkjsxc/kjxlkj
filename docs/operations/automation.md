# Operations Automation Contract

## Automation Goals

- Provide deterministic command interfaces for humans and agents.
- Fail fast with stable exit codes and concise diagnostics.
- Stop code-change work immediately when any gate returns non-zero.

## Canonical Entrypoints

- `cargo run --bin kjxlkj -- docs validate-topology`
- `cargo run --bin kjxlkj -- docs validate-terms`
- `cargo run --bin kjxlkj -- quality check-lines`
- `docker compose --profile verify run --rm verify`
- `cargo run --bin kjxlkj -- compose verify` (optional wrapper with JSON summaries)

## Required Gate Sequence

### 1) Docs topology gate

```bash
cargo run --bin kjxlkj -- docs validate-topology
```

Expected result:

- Exit code `0`.
- Final JSON event includes `"command":"docs.validate-topology"`, `"status":"pass"`, and `"violations":0`.

### 2) Language-compliance gate

```bash
cargo run --bin kjxlkj -- docs validate-terms
```

Expected result:

- Exit code `0`.
- Final JSON event includes `"command":"docs.validate-terms"`, `"status":"pass"`, and `"violations":0`.

### 3) Line-limit gate

```bash
cargo run --bin kjxlkj -- quality check-lines
```

Expected result:

- Exit code `0`.
- Final JSON event includes `"command":"quality.check-lines"`, `"status":"pass"`, and `"violations":0`.

### 4) Docker acceptance gate

1. Complete first-time startup in [../containers/compose/commands.md](../containers/compose/commands.md).
2. Complete setup/login/admin verification in [../containers/verification/local-runbook.md](../containers/verification/local-runbook.md).
3. Run docker acceptance checks:

```bash
docker compose --profile verify run --rm verify
```

Expected result:

- Exit code `0`.
- Verify profile completes formatting, linting, tests, build, docs topology, and line-limit checks.

Optional wrapper command (same contract, machine-readable output):

```bash
cargo run --bin kjxlkj -- compose verify
```

Expected summary on success:

```json
{"command":"compose.verify","status":"pass","steps_passed":4,"steps_total":4}
```

## Failure Handling Rule

If any gate fails:

1. Stop code changes.
2. Capture deterministic diagnostics:

```bash
docker compose logs --no-color --tail=120 app postgres
```

3. Fix the failing contract and rerun the full gate sequence.
