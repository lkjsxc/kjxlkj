# Marks

Mark specifications for position bookmarking.

## Mark Keys

| Key | Action |
|-----|--------|
| `m{a-z}` | Set local mark |
| `m{A-Z}` | Set global mark |
| `` `{mark} `` | Jump to mark (exact) |
| `'{mark}` | Jump to mark (line) |
| `:marks` | List all marks |
| `:delmarks` | Delete marks |

## Special Marks

| Mark | Description |
|------|-------------|
| `` `` `` | Position before last jump |
| `'.` | Position of last change |
| `'^` | Position of last insert |
| `'[` | Start of last change |
| `']` | End of last change |
| `'<` | Start of last visual |
| `'>` | End of last visual |

## Documents

| Document | Content |
|----------|---------|
| [mark-types.md](mark-types.md) | Mark types |
| [mark-commands.md](mark-commands.md) | Commands |
| [mark-navigation.md](mark-navigation.md) | Navigation |
| [mark-persistence.md](mark-persistence.md) | Persistence |
| [automatic-marks.md](automatic-marks.md) | Auto marks |
| [special-marks.md](special-marks.md) | Special marks |
| [jump-marks.md](jump-marks.md) | Jump marks |
| [jumplist.md](jumplist.md) | Jump list |
| [changelist.md](changelist.md) | Change list |

## Related

- Editing: [docs/spec/editing/README.md](docs/spec/editing/README.md)
- Motions: [docs/spec/editing/motions/README.md](docs/spec/editing/motions/README.md)
