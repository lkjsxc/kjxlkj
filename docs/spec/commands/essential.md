# Essential commands
This is the minimum ex command surface expected in spec.

## File

- `:e {file}`
- `:w` / `:w {file}` / `:wa`
- `:q` / `:q!` / `:wq` / `:x` / `:qa`

## Buffers

- `:ls` / `:buffers`
- `:b {n|name}`
- `:bn` / `:bp`
- `:bd` / `:bw`

## Windows

- `:sp` / `:vsp`
- `:new` / `:vnew`
- `:close` / `:only`

## Options

- `:set {opt}` / `:set no{opt}` / `:set {opt}={val}` / `:set {opt}?`

## External

- `:! {cmd}` execute via terminal service
- `:[range]! {cmd}` filter via terminal service
- `:r !{cmd}` read output
