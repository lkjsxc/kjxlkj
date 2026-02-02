# Semantic Tokens

Enhanced syntax highlighting from LSP.

## Overview

Semantic tokens provide language-aware highlighting
beyond regex-based syntax highlighting.

## Comparison

### Regex Syntax


### Semantic Tokens


## Enabling


## Token Types

| Type | Description |
|------|-------------|
| namespace | Modules, packages |
| type | Types, classes |
| class | Class definitions |
| enum | Enum types |
| interface | Interfaces |
| struct | Struct types |
| typeParameter | Generic parameters |
| parameter | Function parameters |
| variable | Variables |
| property | Properties |
| enumMember | Enum variants |
| function | Functions |
| method | Methods |
| macro | Macros |
| keyword | Keywords |
| comment | Comments |
| string | String literals |
| number | Number literals |

## Token Modifiers

| Modifier | Description |
|----------|-------------|
| declaration | Definition site |
| definition | Same as declaration |
| readonly | Immutable |
| static | Static/class level |
| deprecated | Deprecated item |
| async | Async function |
| modification | Write access |

## Styling


## Priority

### Order

1. Semantic tokens (highest)
2. Tree-sitter syntax
3. Regex patterns (lowest)

### Configuration


## Performance

### Incremental

Only changed tokens updated.

### Viewport

Full semantic tokens for visible area.

## LSP Requirements

### Server Support

| Server | Semantic Tokens |
|--------|-----------------|
| rust-analyzer | ✓ |
| clangd | ✓ |
| typescript | ✓ |
| gopls | ✓ |

## Configuration


## Debugging

### Show Token Info


Shows token type and modifiers at cursor.

### Toggle


## Theme Integration

### Semantic Colors


## Tips

1. Enable for accurate highlighting
2. Check token info for debugging
3. Customize modifier styles
4. Combine with tree-sitter
