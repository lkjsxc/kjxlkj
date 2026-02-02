# Macro Recording and Playback

kjxlkj supports vim-style macro recording.

## Recording Macros

### Start Recording


Press `qa` to start recording into register 'a'.

Status bar shows: `recording @a`

### Stop Recording

Press `q` again to stop recording.

### Execute Macro


- `@a` - Play macro in register 'a'
- `@@` - Repeat last played macro
- `5@a` - Play macro 5 times

## Practical Examples

### Add Semicolon to Lines

1. `qa` - Start recording to 'a'
2. `A;` - Append semicolon
3. `j` - Move down
4. `q` - Stop recording
5. `10@a` - Apply to 10 lines

### Wrap Words in Quotes

1. `qa` - Start recording
2. `viw` - Select word
3. `S"` - Surround with quotes
4. `w` - Next word
5. `q` - Stop
6. `100@a` - Apply many times

## Recursive Macros

Macro can call itself:

1. `qa` - Start recording
2. Do operations
3. `@a` - Call self (before stopping!)
4. `q` - Stop
5. `@a` - Runs until error

## Editing Macros

Macros stored in registers. Edit like any register:


Or paste, edit, yank back:
1. `"ap` - Paste register 'a'
2. Edit the text
3. `"ayy` - Yank back to 'a'

## Viewing Macros


## Commands

| Command | Description |
|---------|-------------|
| `:normal @a` | Run macro in command mode |
| `:'<,'>normal @a` | Run on visual selection |

## Configuration


## Keybindings


## Tips

### Count Before Recording

`5@a` executes 5 times. Alternative:
Record once, then `5@a`.

### Visual Mode

In visual mode:
`:'<,'>normal @a`
Runs macro on each selected line.

### Error Handling

Macros stop on error. To continue:


## Persistence

Save macros between sessions:

