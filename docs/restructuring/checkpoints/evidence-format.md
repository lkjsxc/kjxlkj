# Evidence Format

## Required Fields

- `phase`
- `command`
- `status`
- `exit_code`
- `summary`

## Example

```json
{
  "phase": "phase-04-verify",
  "command": "docker compose --profile verify run --rm verify",
  "status": "pass",
  "exit_code": 0,
  "summary": "all gates passed"
}
```
