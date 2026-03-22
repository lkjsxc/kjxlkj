# CLI Output Contract

## JSON Stability

- Machine-oriented commands emit stable JSON keys.
- Output formats should not change without contract updates.

## Command Event Shapes

- `docs validate-topology` emits per-violation events with keys:
  - `command`, `status`, `path`, `rule`, `expected`, `actual`
- `docs validate-topology` final summary keys:
  - `command`, `status`, `directories_checked`, `violations`
- `docs validate-terms` emits per-violation events with keys:
  - `command`, `status`, `path`, `term`, `line`
- `docs validate-terms` final summary keys:
  - `command`, `status`, `files_checked`, `violations`
- `quality check-lines` emits per-violation events with keys:
  - `command`, `status`, `scope`, `path`, `line_count`, `limit`
- `quality check-lines` final summary keys:
  - `command`, `status`, `docs_files_checked`, `source_files_checked`, `violations`
- `compose verify` emits step events with keys:
  - `command`, `step`, `status`
  - optional: `exit_code`, `detail`
- `compose verify` final summary keys:
  - `command`, `status`, `steps_passed`, `steps_total`
  - optional on failure: `failed_step`

## Exit Code Rules

- Exit code `0` means success.
- Non-zero exit code means failure.

## Error Shape Rules

- Error identifiers should be concise and deterministic.
- Error output should be parseable in automation scripts.
- Unsupported command output shape:
  - `{"error":"E_CLI_UNSUPPORTED_COMMAND","message":"unsupported command: ..."}`
