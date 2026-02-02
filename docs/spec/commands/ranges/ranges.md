# Ranges
Ranges select line spans for commands like substitute and filter.

## Requirements
- Range evaluation is deterministic and based on buffer snapshots.
- Pattern-based ranges use core search state (not service results).

## Common range forms

- (none): current line
- `1`: line 1
- `$`: last line
- `.`: current line
- `%`: all lines (1,$)
- `'<,'>`: visual selection
- `5,10`: explicit span
- `.+3`: relative offset
- `/pattern/`: next match
- `?pattern?`: previous match
