# Automation Command Contracts

## CLI Commands

- `docs validate-topology`
- `docs validate-terms`
- `quality check-lines`
- `compose verify`

## Output Rules

- Commands emit deterministic JSON lines.
- Final JSON line includes `command` and `status`.
- Non-zero process exit indicates contract failure.
