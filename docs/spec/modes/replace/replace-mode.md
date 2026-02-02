# Replace Mode

Overtype existing characters.

## Overview

Replace mode overwrites characters
instead of inserting new ones.

## Enter Replace Mode

### From Normal Mode


### Single Character


## Replace Mode Behavior

### Typing

Each character replaces
the character under cursor.

### Example


## Movement

### Cursor Moves Right

After each character typed,
cursor advances to next position.

### At Line End

New characters extend the line.

## Backspace

### Undo Replace


### Behavior

Backspace restores the original
character, not just deletes.

### Example


## Exit Replace Mode

### Commands


## Single Replace

### One Character


### Example


### With Count


## Replace Newline

### Special Case


### Effect

Splits line at cursor position.

## Replace Tab

### Behavior


Tab may replace multiple
display columns.

## Replace Mode Variants

### Standard Replace


### Virtual Replace


## Virtual Replace Mode

### Enter


### Behavior

Considers tabs as spaces.
Doesn't corrupt tab alignment.

### Example


## Virtual Replace Character

### Single


### Tab Handling

Replaces virtual position,
preserves surrounding tabs.

## Replace vs Insert

### Replace

- Overwrites existing
- Length may change at EOL
- Original recoverable with BS

### Insert

- Shifts text right
- Length always increases
- No overwrite

## Common Operations

### Fix Single Char


### Overtype Section

