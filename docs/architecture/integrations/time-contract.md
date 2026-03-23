# Time Contract

## Timestamp Format

All record timestamps use UTC RFC3339 with trailing `Z`.

## Example

`2026-03-23T05:00:00Z`

## Determinism Rule

- Stored timestamps preserve full second precision.
- Responses must return the stored value verbatim.
