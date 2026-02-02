# Spell Checking

Built-in spell checking for text documents.

## Enabling Spell Check

### Command


### Keybindings

| Key | Action |
|-----|--------|
| `<leader>ss` | Toggle spell check |
| `<leader>sl` | Change language |

## Navigation

| Key | Action |
|-----|--------|
| `]s` | Next misspelled word |
| `[s` | Previous misspelled word |
| `]S` | Next bad word (skip rare) |
| `[S` | Previous bad word |

## Corrections

| Key | Action |
|-----|--------|
| `z=` | Suggest corrections |
| `zg` | Add word to dictionary |
| `zw` | Mark word as wrong |
| `zug` | Undo add to dictionary |
| `zuw` | Undo mark as wrong |
| `1z=` | Use first suggestion |

## Visual Indicators

Misspelled words are underlined:

- **Red underline**: Unknown word
- **Blue underline**: Rare word
- **Yellow underline**: Wrong case

## Languages

### Configuration


### Available Languages

- `en_US` - English (US)
- `en_GB` - English (UK)
- `de_DE` - German
- `fr_FR` - French
- `es_ES` - Spanish

### Multiple Languages


## Dictionary Management

### User Dictionary

Location: `~/.config/kjxlkj/spell/user.dic`

### Adding Words

1. Navigate to word
2. Press `zg`

### Project Dictionary


## File Type Settings

### Enable for Specific Types


### Code Comments


## Suggestions

### Fuzzy Matching

Suggestions ranked by:
1. Edit distance
2. Keyboard proximity
3. Common patterns

### Quick Replace


## Performance

### Large Files

Spell check is incremental:
- Only visible lines checked
- Background processing
- Cached results

### Disable When Not Needed


## Ignore Patterns

### Configuration


### Built-in Ignores

- Hex codes: `#ff0000`
- Paths: `/usr/bin/`
- Emails: `user@domain.com`

## Integration

### LSP

Spell suggestions in completion menu.

### Status Line

Shows spell language when active.
