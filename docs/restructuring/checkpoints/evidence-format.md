# Evidence Format

## Required Fields

- `stage_id`
- `wave_id` (or `none` for global gates)
- `gate_id`
- `status` (`pass` | `fail` | `blocked`)
- `summary`
- `artifacts`
- `next_action`

## JSON Template

```json
{
  "stage_id": "S03",
  "wave_id": "W031",
  "gate_id": "LINK-AUDIT-01",
  "status": "pass",
  "summary": "all touched links resolve",
  "artifacts": ["docs/restructuring/stages/stage-03-web-shell-and-editor/wave-031.md"],
  "next_action": "proceed"
}
```

## Blocking Entry Rule

When status is `blocked`, `summary` must include the direct cause and `next_action` must be concrete.
