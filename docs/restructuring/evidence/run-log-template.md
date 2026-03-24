# Run Log Template

## Entry Format

```text
timestamp: <iso-8601>
stage_id: <Sxx>
wave_id: <Wyyy|none>
operation: <command or audit>
status: <pass|fail|blocked>
summary: <short deterministic outcome>
next_action: <explicit remediation or proceed>
```

## Example Entry

```text
timestamp: 2026-03-24T00:00:00Z
stage_id: S00
wave_id: W000
operation: structure-audit
status: pass
summary: all directories satisfy README and child-count rules
next_action: proceed to W001
```
