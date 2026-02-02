# Special Registers

System and special-purpose registers for various contexts.

## Register Overview

| Register | Name | Description |
|----------|------|-------------|
| `""` | Unnamed | Default register |
| `"-` | Small delete | Last delete <1 line |
| `".` | Last insert | Last inserted text |
| `"%` | Filename | Current filename |
| `"#` | Alternate | Alternate filename |
| `":` | Last command | Last command-line |
| `"/` | Last search | Last search pattern |
| `"=` | Expression | Expression result |
| `"*` | Selection | System selection |
| `"+` | Clipboard | System clipboard |
| `"_` | Black hole | Discards content |

## Unnamed Register ("")

Default for all operations:


### Syncing Options


## Small Delete Register (-)

Contains last delete smaller than one line:


## Last Insert Register (.)

Contains text from last insert mode:


Read-only. Updated on each insert mode exit.

## Filename Register (%)

Contains current buffer filename:


## Alternate File Register (#)

Contains alternate (previous) buffer filename:


## Last Command Register (:)

Contains last executed command-line:


## Last Search Register (/)

Contains last search pattern:


## Expression Register (=)

Evaluate expression and use result:


See [expression-register.md](expression-register.md) for details.

## Selection and Clipboard (+, *)

### Selection Register (*)

System primary selection (X11):


### Clipboard Register (+)

System clipboard:


See [clipboard-registers.md](clipboard-registers.md) for details.

## Black Hole Register (_)

Discards anything written to it:


See [blackhole-register.md](blackhole-register.md) for details.

## Using Special Registers

### In Insert Mode

| Key | Action |
|-----|--------|
| `Ctrl-R "` | Insert unnamed register |
| `Ctrl-R %` | Insert filename |
| `Ctrl-R /` | Insert search pattern |
| `Ctrl-R :` | Insert last command |
| `Ctrl-R .` | Insert last insert |
| `Ctrl-R =` | Expression prompt |

### In Command Line


## Read-Only Registers

These cannot be written to directly:

| Register | Reason |
|----------|--------|
| `".` | Updated by insert mode |
| `"%` | Reflects current file |
| `"#` | Reflects alternate file |
| `":` | Updated by command execution |

## Configuration


## API Reference

