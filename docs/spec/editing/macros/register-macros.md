# Register Macros

Using registers for macro recording and playback.

## Recording Macros

### Start Recording


### Stop Recording


### Recording Indicator

Status line shows recording state:

## Playing Macros

### Basic Playback


### With Count


## Macro Storage

Macros are stored as keystrokes in registers:


Example content:
(`^[` represents Escape)

## Editing Macros

### Method 1: Paste, Edit, Yank


### Method 2: Command Line


### Escape Sequences

| Sequence | Key |
|----------|-----|
| `\<Esc>` | Escape |
| `\<CR>` | Enter |
| `\<Tab>` | Tab |
| `\<C-w>` | Ctrl-W |
| `\<Left>` | Left arrow |

## Appending to Macros

Use uppercase register to append:


## Recursive Macros

Macro that calls itself:


Run with count to limit iterations:

## Macro Best Practices

### Start at Known Position


### End at Next Start


### Use Relative Motions


## Example Macros

### Add Semicolon to End of Line


### Wrap Word in Quotes


### Delete First Word of Each Line


## Viewing Macros


## Saving Macros

### In Configuration


### Persistence


## Running Macro on Visual Selection

