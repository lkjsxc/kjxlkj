# Live Grep

Interactive real-time project-wide search with ripgrep integration.

## Opening Live Grep

| Key | Action |
|-----|--------|
| `<Leader>lg` | Open live grep picker |
| `<Leader>fw` | Find word in project |
| `<Leader>/` | Search in project |
| `:LiveGrep` | Command to open |

## Interface


## Real-Time Features

- Results update as you type
- Debounced input (configurable delay)
- Streaming results
- Progress indicator

## Configuration


## Ripgrep Arguments


## Picker Keybindings

| Key | Action |
|-----|--------|
| `Enter` | Open file at match |
| `Ctrl-v` | Open in vertical split |
| `Ctrl-x` | Open in horizontal split |
| `Ctrl-t` | Open in new tab |
| `Ctrl-q` | Send to quickfix |
| `Tab` | Toggle selection |
| `Ctrl-a` | Select all |
| `Esc` | Cancel |
| `Ctrl-u` | Clear query |

## Search Modifiers

### In Query

| Prefix | Effect |
|--------|--------|
| `!pattern` | Exclude pattern |
| `'pattern` | Literal match |
| `^pattern` | Start of line |
| `pattern$` | End of line |

### Toggle Keys

| Key | Effect |
|-----|--------|
| `Ctrl-r` | Toggle regex mode |
| `Ctrl-c` | Toggle case sensitivity |
| `Ctrl-w` | Toggle whole word |
| `Ctrl-h` | Toggle hidden files |

## File Type Filtering


### Common Types

| Type | Extensions |
|------|------------|
| `rust` | .rs |
| `python` | .py |
| `javascript` | .js, .jsx |
| `typescript` | .ts, .tsx |
| `markdown` | .md |
| `json` | .json |
| `yaml` | .yaml, .yml |

## Path Filtering


## Preview Window


## Replace Mode

Live grep with replace:


Interface:

## Keybindings Configuration


## Integration with Finder


## Performance Tuning


## Ignore Patterns

