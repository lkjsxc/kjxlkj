# Spell Checking

Built-in spell checking for text documents.

## Enabling

`:set spell` enables spell checking. `:set nospell` disables it.

Toggle: `<leader>ss`.

## Navigation (normative)

| Key | Action |
|---|---|
| `]s` | Next misspelled word |
| `[s` | Previous misspelled word |
| `]S` | Next bad word (skip rare/regional) |
| `[S` | Previous bad word |

## Corrections (normative)

| Key | Action |
|---|---|
| `z=` | Show suggestion list for word under cursor |
| `1z=` | Apply first suggestion |
| `zg` | Add word to user dictionary (good) |
| `zw` | Mark word as wrong |
| `zug` | Undo `zg` (remove from dictionary) |
| `zuw` | Undo `zw` |

## Visual Indicators

| Underline Color | Meaning |
|---|---|
| Red | Unknown / misspelled word |
| Blue | Rare word |
| Yellow | Wrong capitalization |

## Languages

Default language: `en_US`. Change with `:set spelllang=de_DE`.

Supported: `en_US`, `en_GB`, `de_DE`, `fr_FR`, `es_ES`, and any Hunspell-compatible dictionary.

Multiple languages: `:set spelllang=en_US,de_DE` checks against both.

## Dictionary Management

### User Dictionary

Location: `~/.config/kjxlkj/spell/user.dic`. Words added with `zg` go here.

### Project Dictionary

Place a `.kjxlkj-spell` file in the project root for project-specific words.

## File Type Integration

Spell checking in code files only checks comments and strings (using tree-sitter to identify them). In prose files (Markdown, text), all content is checked.

## Suggestion Ranking

Suggestions are ranked by:

1. Edit distance (Levenshtein)
2. Keyboard proximity
3. Common substitution patterns

## Performance

Spell checking is incremental — only changed and visible lines are re-checked. Cached results are reused for unchanged content.

## Ignore Patterns

Built-in ignores: hex codes (`#ff0000`), paths (`/usr/bin/`), emails (`user@example.com`), URLs.

## Related

- Autocommands: [/docs/spec/features/config/autocommands.md](/docs/spec/features/config/autocommands.md)
