# Failure Handling Contract

## Required Sequence

1. Stop additional code changes.
2. Capture failing command output.
3. Fix root cause.
4. Re-run full gate sequence from the beginning.

## Logging Rule

Errors are explicit and machine-readable; silent failure is forbidden.
