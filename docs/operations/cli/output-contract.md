# CLI Output Contract

## JSON Stability

- Machine-oriented commands emit stable JSON keys.
- Output formats should not change without contract updates.

## Exit Code Rules

- Exit code `0` means success.
- Non-zero exit code means failure.

## Error Shape Rules

- Error identifiers should be concise and deterministic.
- Error output should be parseable in automation scripts.
