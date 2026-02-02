# Highlight Groups

Color and style definitions.

## Overview

Highlight groups define colors and styles
for syntax and UI elements.

## Core Groups

### Editor

| Group | Description |
|-------|-------------|
| `Normal` | Default text |
| `NormalNC` | Non-current window |
| `Visual` | Visual selection |
| `VisualNOS` | Not-owning selection |
| `Cursor` | Character under cursor |
| `CursorLine` | Cursor line |
| `CursorColumn` | Cursor column |
| `ColorColumn` | Color column |
| `LineNr` | Line numbers |
| `CursorLineNr` | Current line number |
| `SignColumn` | Sign column |
| `FoldColumn` | Fold column |
| `Folded` | Folded lines |
| `VertSplit` | Vertical split |
| `WinSeparator` | Window separator |

### Status

| Group | Description |
|-------|-------------|
| `StatusLine` | Status line |
| `StatusLineNC` | Inactive status |
| `TabLine` | Tab line |
| `TabLineFill` | Tab line fill |
| `TabLineSel` | Selected tab |
| `WildMenu` | Command completion |

### Messages

| Group | Description |
|-------|-------------|
| `ModeMsg` | Mode message |
| `MoreMsg` | More prompt |
| `Question` | Questions |
| `WarningMsg` | Warnings |
| `ErrorMsg` | Errors |

### Search

| Group | Description |
|-------|-------------|
| `Search` | Search match |
| `IncSearch` | Incremental search |
| `CurSearch` | Current match |
| `Substitute` | Substitute preview |

### Popup/Float

| Group | Description |
|-------|-------------|
| `Pmenu` | Popup menu |
| `PmenuSel` | Selected item |
| `PmenuSbar` | Scrollbar |
| `PmenuThumb` | Scrollbar thumb |
| `FloatBorder` | Float border |
| `FloatTitle` | Float title |

## Syntax Groups

### Standard

| Group | Description |
|-------|-------------|
| `Comment` | Comments |
| `Constant` | Constants |
| `String` | Strings |
| `Character` | Characters |
| `Number` | Numbers |
| `Boolean` | Booleans |
| `Float` | Floats |

### Identifiers

| Group | Description |
|-------|-------------|
| `Identifier` | Variables |
| `Function` | Functions |
| `Statement` | Statements |
| `Conditional` | if/else |
| `Repeat` | Loops |
| `Label` | Labels |
| `Operator` | Operators |
| `Keyword` | Keywords |

### Types

| Group | Description |
|-------|-------------|
| `Type` | Types |
| `StorageClass` | static/const |
| `Structure` | struct/class |
| `Typedef` | Type aliases |

### Special

| Group | Description |
|-------|-------------|
| `Special` | Special chars |
| `SpecialChar` | Escape chars |
| `Tag` | Tags |
| `Delimiter` | Delimiters |
| `SpecialComment` | Doc comments |

## Diagnostic Groups

| Group | Description |
|-------|-------------|
| `DiagnosticError` | Errors |
| `DiagnosticWarn` | Warnings |
| `DiagnosticInfo` | Information |
| `DiagnosticHint` | Hints |
| `DiagnosticUnderlineError` | Error underline |
| `DiagnosticUnderlineWarn` | Warning underline |

## Git Groups

| Group | Description |
|-------|-------------|
| `DiffAdd` | Added lines |
| `DiffChange` | Changed lines |
| `DiffDelete` | Deleted lines |
| `DiffText` | Changed text |
| `GitSignsAdd` | Add sign |
| `GitSignsChange` | Change sign |
| `GitSignsDelete` | Delete sign |

## Definition

### Basic


### Attributes

| Attribute | Description |
|-----------|-------------|
| `fg` | Foreground color |
| `bg` | Background color |
| `sp` | Special (underline) |
| `bold` | Bold text |
| `italic` | Italic text |
| `underline` | Underline |
| `undercurl` | Wavy underline |
| `strikethrough` | Strike through |
| `reverse` | Swap fg/bg |

### Linking


## Colors

### Named Colors


### Hex Colors


### ANSI Colors


## Semantic Tokens

| Group | Description |
|-------|-------------|
| `@variable` | Variables |
| `@function` | Functions |
| `@type` | Types |
