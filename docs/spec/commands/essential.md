# Essential commands
This is the target minimum Ex command surface.

For the currently supported subset (when a reconstructed implementation exists), see [docs/reference/CONFORMANCE.md](/docs/reference/CONFORMANCE.md).

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
