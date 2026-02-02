# Very Magic Mode

The `\v` pattern modifier for Perl-like regular expressions.

## Activating Very Magic


## Character Behavior Comparison

| Character | Normal | Very Magic (\v) |
|-----------|--------|-----------------|
| `(` `)` | Literal | Grouping |
| `{` `}` | Literal | Quantifier |
| `+` | Literal | One or more |
| `?` | Literal | Zero or one |
| `\|` | Literal | Alternation |
| `<` `>` | Literal | Word boundary |
| `=` | Literal | Zero or one (?) |
| `@` | Literal | Lookaround |

## Grouping

### Normal Mode


### Very Magic Mode


## Quantifiers

### Normal Mode


### Very Magic Mode


## Alternation

### Normal Mode


### Very Magic Mode


## Word Boundaries

### Normal Mode


### Very Magic Mode


## Complete Examples

### Match Function Calls


### Match Email Address


### Match IP Address


### Swap Two Words


## Characters Still Needing Escape

Even in very magic mode, some need escaping:

| Character | Reason |
|-----------|--------|
| `\` | Escape character itself |
| `/` | Search delimiter |
| `@` | Lookaround prefix |

## Lookaround in Very Magic


## Comparison Table

| Pattern | Normal | Very Magic |
|---------|--------|------------|
| Group | `\(...\)` | `(...)` |
| Or | `\|` | `\|` |
| One+ | `\+` | `+` |
| Zero/One | `\?` or `\=` | `?` |
| Count | `\{n,m\}` | `{n,m}` |
| Word start | `\<` | `<` |
| Word end | `\>` | `>` |
| Not | `\(...\)\@!` | `(...)@!` |

## When to Use Very Magic

### Recommended For

- Complex patterns with groups
- Patterns with quantifiers
- Perl/PCRE users
- Readable regex patterns

### Stick with Normal For

- Simple literal searches
- Single character matches
- Compatibility with scripts

## Configuration Default


Make very magic default:

## Escaping in Very Magic


## API Reference


