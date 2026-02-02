# Column/Block Editing

Visual block mode operations.

## Overview

Edit rectangular blocks of text for
column-oriented operations.

## Entering Block Mode

### Start Selection


### Extend Selection

| Key | Direction |
|-----|-----------|
| `h` | Left |
| `j` | Down |
| `k` | Up |
| `l` | Right |

### Select to End


## Block Operations

### Insert Before


Applies to all lines in block.

### Append After


### Change Block


### Delete Block


## Column Insert

### Example

Starting text:

With cursor on `f`, press `Ctrl-V`, `jj`, `I`, `- `, `Esc`:

## Column Delete

### Example


Select leading spaces with `Ctrl-V`, `jj`, `l`, `d`:

## Column Replace

### Replace Characters


### Example


Select column of `|`, press `r `:

## Column Yank/Put

### Yank Block


### Put Block


Block put maintains rectangular shape.

## Visual Block Selection

### Start from Any Corner


### Select Entire Lines


### Select Characters


## Column Numbers

### Insert Sequence


Example with zeros:

Select, `g Ctrl-A`:

## Column Alignment

### Align Block Right

Select block, then:

### Align Block Left


### Center Block


## Rectangular Paste

