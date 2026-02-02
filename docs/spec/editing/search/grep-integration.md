# Grep Integration

Using external grep tools for project-wide search.

## Built-in :grep Command


## Quickfix Integration

Grep results populate quickfix list:

| Command | Action |
|---------|--------|
| `:grep pattern` | Search and fill quickfix |
| `:copen` | Open quickfix window |
| `:cnext` / `:cn` | Go to next result |
| `:cprev` / `:cp` | Go to previous result |
| `:cc N` | Go to result N |

## Grep Program Configuration


### Using ripgrep


### Using ag (The Silver Searcher)


## Grepprg Format


## Grepformat

Output parsing format:


| Token | Meaning |
|-------|---------|
| `%f` | Filename |
| `%l` | Line number |
| `%c` | Column number |
| `%m` | Message/content |
| `%n` | Error number |

## Async Grep

Run grep in background:



## Grep Keybindings

| Key | Action |
|-----|--------|
| `<Leader>gg` | Grep word under cursor |
| `<Leader>gw` | Grep current word (whole) |
| `<Leader>gv` | Grep visual selection |
| `<Leader>gf` | Grep in current file |


## Grep Commands

| Command | Action |
|---------|--------|
| `:grep` | External grep to quickfix |
| `:lgrep` | External grep to location list |
| `:vimgrep` | Internal grep (slower, more features) |
| `:lvimgrep` | Internal grep to location list |
| `:grepadd` | Add to existing quickfix |

## Vimgrep vs Grep

| Feature | :grep | :vimgrep |
|---------|-------|----------|
| Speed | Fast (external) | Slower (internal) |
| Syntax | Program-specific | Vim regex |
| Multi-line | Depends | Supported |
| Binary | May include | Excluded |

## Search Patterns

### Basic Patterns


### File Patterns


## Location List Variant

Per-window results:


## Grepadd - Append Results


## Filtering Results


## Common Configurations

### Fast ripgrep Setup


### Git Grep


## Error Handling

