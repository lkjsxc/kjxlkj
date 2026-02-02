# Tag Text Objects

Text objects for markup tags.

## Overview

Select content inside or around
HTML/XML/JSX style tags.

## Tag Definitions

### Valid Tags


### Self-Closing Tags


## Inner Tag

### Command


### Behavior

Selects everything between
opening and closing tags.

### Example


## Around Tag

### Command


### Behavior

Selects entire element:
opening tag + content + closing tag.

### Example


## Nested Tags

### Same Type


With cursor on "inner":

### Different Types


## Multiline Content

### Block Elements


With cursor on item:

## Tag Attributes

### Preserved with `it`


### Removed with `at`


## Self-Closing Tags

### No Content


### Deletion


## JSX/TSX Support

### Component Tags


Works same as HTML tags.

### Fragments


## XML Support

### Namespaced Tags


### Processing Instructions


Not matched by tag objects.

## Finding Tags

### Search Algorithm

1. Find nearest enclosing tags
2. Match tag names
3. Handle nesting properly

### Cursor Position

Cursor can be anywhere inside:
- Opening tag
- Content
- Closing tag (usually)

