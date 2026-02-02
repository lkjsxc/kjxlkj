# Quote Text Objects

Text objects for quoted strings.

## Overview

Select text inside or around
various quote characters.

## Double Quotes

### Inner


### Around


### Example


## Single Quotes

### Inner


### Around


### Example


## Backticks

### Inner


### Around


### Example


## Quote Detection

### Finding Pairs

Cursor doesn't need to be on quote.
Searches forward on line.


### Search Behavior

1. Check if cursor on quote
2. Search forward for opening
3. Find matching closing

## Nested Quotes

### Different Types


### Same Type (Escaped)


## Multiline Quotes

### Behavior


### When Enabled


## String Continuation

### Escaped Newlines


## Special Cases

### Empty Quotes


### Adjacent Quotes


## Language Context

### Python Triple Quotes


With tree-sitter:

### Raw Strings


Tree-sitter understands these.

## Quote on Line

### No Quotes Found

