# Search Count Display

Showing match position and total count information.

## Count Format

Standard display format:


## Display Location

### Command Line

After search:

### statusline

In status bar:

### Virtual Text

Inline with matches:

## Configuration


## Format Options

| Format | Example |
|--------|---------|
| `[%d/%d]` | [3/10] |
| `%d of %d` | 3 of 10 |
| `(%d/%d)` | (3/10) |
| `Match %d/%d` | Match 3/10 |

### Custom Format


## Count Update Events

Count updates on:
- New search pattern entered
- Navigation (`n`, `N`)
- Pattern register change
- Buffer modification

## Performance Optimization

### Counting Limits


### Large Files


## Navigation Commands

| Key | Action |
|-----|--------|
| `n` | Next match (updates count) |
| `N` | Previous match |
| `[count]n` | Jump n matches forward |
| `[count]N` | Jump n matches backward |

## Searchcount() Function

Access count programmatically:


### Return Values

| Field | Description |
|-------|-------------|
| `current` | Current match index |
| `total` | Total matches |
| `incomplete` | 0=complete, 1=recomputing, 2=exceeded |
| `maxcount` | Configured max count |

## statusline Integration

### Basic


### Conditional


## Visual Feedback

### Current Match Indicator


### Progress While Counting


## Wrap-Around Notification


## Command-Line Display

### During Incremental Search


### After Search Complete


## API Reference

