# Tag Text Objects

Select HTML/XML tag pairs and content.

## Overview

Tag text objects work with HTML, XML, JSX, and similar
markup languages to select tag pairs and their content.

## Commands

### Inner Tag

`it` selects the content between the opening and
closing tags, excluding the tags themselves.

### Around Tag

`at` selects from the start of the opening tag to
the end of the closing tag, including both tags.

## Examples

### Simple Tags

On `<div>hello world</div>`:
- `dit` selects `hello world`
- `dat` selects `<div>hello world</div>`

### Nested Tags

On `<div><span>text</span></div>`, cursor on `text`:
- `dit` selects `text` (innermost tag content)
- `dat` selects `<span>text</span>`
- `2dit` selects `<span>text</span>` (outer inner)
- `2dat` selects `<div><span>text</span></div>`

### With Attributes

On `<div class="box">content</div>`:
- `dat` includes the full opening tag with attributes

## Tag Detection

### Matching Algorithm

1. Search backward for `<` that starts a tag
2. Parse the tag name
3. Search forward for `</tagname>`
4. Match by tag name, handling nesting depth

### Self-Closing Tags

`<br/>`, `<img src="x"/>` are treated as single
elements. `it` on a self-closing tag is empty.
`at` selects the entire self-closing tag.

### Void Elements

HTML void elements (`<br>`, `<hr>`, `<img>`, `<input>`)
without explicit close are recognized.

## With Operators

### Common Operations

| Sequence | Effect |
|----------|--------|
| `dit` | Delete tag content |
| `dat` | Delete entire tag element |
| `cit` | Change tag content |
| `cat` | Change entire tag element |
| `yit` | Yank tag content |
| `vat` | Select entire tag visually |

## Multi-Line

### Block Tags

Tags spanning multiple lines are handled correctly.
The content between opening and closing tags is
selected regardless of line count.

### Indented Content

Indentation within the tag is part of `it` selection.
`dit` removes content including internal indentation.

## Count

### Nesting Levels

`{count}it` selects the content of the {count}th
enclosing tag. `2it` selects the parent tag's content.

## Tree-sitter Integration

### Improved Accuracy

With tree-sitter, tag matching uses the AST for
perfect accuracy in complex cases like self-closing
tags, comments containing tags, and string literals.

### JSX/TSX

Tree-sitter enables correct tag handling in JSX/TSX
where tags can contain expressions `{expr}`.

## Edge Cases

### Mismatched Tags

If no matching close tag is found, the text object
fails (no selection). No error is displayed.

### Comments

Tags within comments (`<!-- <div> -->`) are ignored
by the matching algorithm.

### Attributes with Special Characters

Attributes containing `>` or `<` in quoted values
do not interfere with tag detection.
