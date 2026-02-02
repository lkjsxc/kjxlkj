# Keybindings: Text Objects

Complete nvim-compatible text object keybindings.

## Word Objects

| Object | Inner | Around | Description |
|--------|-------|--------|-------------|
| Word | `iw` | `aw` | Word (letters, digits, underscores) |
| WORD | `iW` | `aW` | WORD (non-whitespace sequence) |

## Quote Objects

| Object | Inner | Around | Description |
|--------|-------|--------|-------------|
| Double quote | `i"` | `a"` | Double-quoted string |
| Single quote | `i'` | `a'` | Single-quoted string |
| Backtick | `` i` `` | `` a` `` | Backtick-quoted string |

## Bracket Objects

| Object | Inner | Around | Description |
|--------|-------|--------|-------------|
| Parentheses | `i(` `i)` `ib` | `a(` `a)` `ab` | Parentheses content |
| Brackets | `i[` `i]` | `a[` `a]` | Square bracket content |
| Braces | `i{` `i}` `iB` | `a{` `a}` `aB` | Curly brace content |
| Angle brackets | `i<` `i>` | `a<` `a>` | Angle bracket content |

## Tag Objects (HTML/XML)

| Object | Inner | Around | Description |
|--------|-------|--------|-------------|
| Tag | `it` | `at` | HTML/XML tag content |

## Block Objects

| Object | Inner | Around | Description |
|--------|-------|--------|-------------|
| Sentence | `is` | `as` | Sentence |
| Paragraph | `ip` | `ap` | Paragraph |

## Usage Examples

| Command | Action |
|---------|--------|
| `diw` | Delete inner word |
| `daw` | Delete a word (with whitespace) |
| `ci"` | Change inside double quotes |
| `ca"` | Change around double quotes |
| `yi(` | Yank inside parentheses |
| `da{` | Delete around curly braces |
| `vit` | Visual select inside tag |
| `dip` | Delete inner paragraph |
| `yap` | Yank a paragraph |
| `cis` | Change inner sentence |

## Text Object Modifiers

| Modifier | Description |
|----------|-------------|
| `i` | Inner (excludes delimiters) |
| `a` | Around (includes delimiters) |

## Combining with Counts

| Command | Action |
|---------|--------|
| `d2iw` | Delete 2 inner words |
| `c2a"` | Change 2 quoted strings |
| `y3ip` | Yank 3 paragraphs |

