# Mark Persistence

Saving and restoring marks across editor sessions.

## What Gets Persisted

| Mark Type | Default Persistence |
|-----------|---------------------|
| Lowercase (a-z) | No |
| Uppercase (A-Z) | Yes |
| Numbered (0-9) | Yes |
| Special | `'"` only |

## Configuration

### Basic Settings


### Advanced Settings


## Persistence File Format

### Location


### Structure


## When Marks Are Saved

### On Exit

All persistent marks saved when editor closes normally.

### Periodic Save


### On Buffer Close

Buffer position (`'"`) saved when buffer is closed.

## When Marks Are Loaded

### On Startup

Global marks and file history loaded at editor start.

### On Buffer Open

Buffer-specific marks loaded when file is opened.

## Handling Missing Files

When a marked file no longer exists:


| Option | Behavior |
|--------|----------|
| `keep` | Preserve mark, may fail on jump |
| `remove` | Silently remove invalid marks |
| `ask` | Prompt user on first access |

## File Move/Rename Detection


## Backup


Creates:

## Commands

### Force Save


### Force Load


### Clear Persistent Marks


## Local Mark Persistence

To persist local marks (a-z):


| Scope | Storage |
|-------|---------|
| `file` | With each file's metadata |
| `workspace` | Per workspace folder |
| `global` | Single global store |

## API Reference

