# Digraphs

Back: [/docs/spec/editing/text-manipulation/README.md](/docs/spec/editing/text-manipulation/README.md)

Two-character input sequences for entering special characters not available on the keyboard.

## Overview

Digraphs map two ASCII characters to a Unicode character. In Insert mode, pressing `Ctrl-k` followed by two characters inserts the corresponding special character.

## Input method

| Key sequence | Action |
|---|---|
| `Ctrl-k {char1} {char2}` | Insert the digraph defined by `{char1}{char2}` |

After `Ctrl-k`, the next two keystrokes are interpreted as a digraph pair. If the pair is not defined, no character is inserted and a bell is emitted.

## Common digraphs

### Currency

| Digraph | Character | Description |
|---|---|---|
| `Eu` | € | Euro sign |
| `Pd` | £ | Pound sign |
| `Ye` | ¥ | Yen sign |
| `Ct` | ¢ | Cent sign |

### Punctuation

| Digraph | Character | Description |
|---|---|---|
| `<<` | « | Left guillemet |
| `>>` | » | Right guillemet |
| `--` | — | Em dash |
| `-.` | – | En dash |
| `..` | … | Ellipsis |
| `!I` | ¡ | Inverted exclamation |
| `?I` | ¿ | Inverted question |

### Math and logic

| Digraph | Character | Description |
|---|---|---|
| `+-` | ± | Plus-minus |
| `*X` | × | Multiplication |
| `-:` | ÷ | Division |
| `!=` | ≠ | Not equal |
| `=<` | ≤ | Less or equal |
| `>=` | ≥ | Greater or equal |
| `00` | ∞ | Infinity |

### Accented characters

| Digraph | Character | Description |
|---|---|---|
| `a'` | á | A acute |
| `e'` | é | E acute |
| `a!` | à | A grave |
| `a>` | â | A circumflex |
| `a:` | ä | A diaeresis |
| `a~` | ã | A tilde |
| `n~` | ñ | N tilde |

### Greek letters

| Digraph | Character | Description |
|---|---|---|
| `a*` | α | Alpha |
| `b*` | β | Beta |
| `g*` | γ | Gamma |
| `d*` | δ | Delta |
| `p*` | π | Pi |
| `s*` | σ | Sigma |

## Commands

| Command | Description |
|---|---|
| `:digraphs` | List all defined digraphs |
| `:digraph {c1}{c2} {codepoint}` | Define a custom digraph |

## Custom digraphs

Users can define custom digraphs in configuration:

| Setting | Type | Description |
|---|---|---|
| `digraphs` | table | Custom digraph definitions: key is the 2-char pair, value is the Unicode code point |

## Related

- Insert digraphs: [/docs/spec/modes/insert/input/insert-digraphs.md](/docs/spec/modes/insert/input/insert-digraphs.md)
- Unicode input: [/docs/spec/modes/insert/input/insert-unicode.md](/docs/spec/modes/insert/input/insert-unicode.md)
- Special characters: [/docs/spec/modes/insert/input/insert-special-chars.md](/docs/spec/modes/insert/input/insert-special-chars.md)
