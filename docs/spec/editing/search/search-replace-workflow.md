# Search and Replace Workflow

Common patterns for finding and replacing text.

## Basic Workflow

1. Search with `/pattern` to verify matches
2. Replace with `:%s/pattern/replacement/gc`
3. Review confirmations (or use `g` flag for all)

## Confirmation Keys

| Key | Action |
|---|---|
| `y` | Replace this match |
| `n` | Skip this match |
| `a` | Replace all remaining |
| `q` | Quit substitution |
| `l` | Replace this and quit (last) |
| `Ctrl-E` | Scroll up |
| `Ctrl-Y` | Scroll down |

## Using gn Motion

`gn` selects the next search match visually. Combined with `cgn`, this creates a repeatable replace:

1. `/pattern` to set search
2. `cgn` to change first match
3. Type replacement text, press `Esc`
4. `.` repeats on next match, `n` skips

Advantages: no need to retype pattern, repeatable with `.`, can skip with `n`.

## Multi-File Replace

| Command | Scope |
|---|---|
| `:argdo %s/old/new/ge` | All arglist files |
| `:bufdo %s/old/new/ge` | All open buffers |
| `:cfdo %s/old/new/ge` | All quickfix files |

Add `\| update` to save each file after replacing.

## Case-Preserving Replace

The `\u`, `\l`, `\U`, `\L` modifiers transform case in replacements:

| Modifier | Effect |
|---|---|
| `\u` | Uppercase next character |
| `\l` | Lowercase next character |
| `\U` | Uppercase until `\E` or end |
| `\L` | Lowercase until `\E` or end |
| `\E` | End case modification |

## Capture Groups

Use `\(...\)` to capture, `\1`..`\9` to reference in replacement.

In very-magic mode (`\v`): `(...)` captures, same back-references.

## Common Patterns

| Task | Command |
|---|---|
| Remove trailing whitespace | `:%s/\s\+$//g` |
| Convert tabs to spaces | `:%s/\t/    /g` |
| Delete empty lines | `:g/^$/d` |
| Remove duplicate lines | `:sort u` |
| Wrap in quotes | `:%s/.*/"\0"/g` |
| Append semicolons | `:%s/$/;/g` |
| Swap words | `:%s/\v(foo)(bar)/\2\1/g` |

## Interactive Preview

When `inccommand` is enabled, substitutions show a live preview: original text highlighted, replacement shown inline, match count displayed.

## Undo Integration

Each `:substitute` command is a single undo step. Use `u` to undo the entire substitution. `:argdo` / `:bufdo` create one undo step per buffer.

## Related

- Substitute command: [/docs/spec/editing/text-manipulation/substitute.md](/docs/spec/editing/text-manipulation/substitute.md)
- Search: [/docs/spec/editing/search/README.md](/docs/spec/editing/search/README.md)
