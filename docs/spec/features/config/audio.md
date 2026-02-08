# Audio Feedback

Back: [/docs/spec/features/config/README.md](/docs/spec/features/config/README.md)

Optional audio and visual bell cues for accessibility and error notification.

## Overview

Audio feedback provides non-visual cues for mode changes, errors, and events. The editor delegates all audio to terminal bell capabilities.

## Enabling audio

| Setting | Default | Description |
|---|---|---|
| `bell.enabled` | `true` | Master switch for all bell output |
| `bell.type` | `audible` | `audible` (terminal bell) or `visual` (screen flash) |

## Terminal bell

### Basic bell

The editor emits the ASCII BEL character (`\x07`) to stdout when a bell-triggering event occurs. The terminal application is responsible for producing the actual sound or visual effect.

### Bell events

| Event | Default bell | Description |
|---|---|---|
| `error` | Yes | Invalid command or motion that fails |
| `search_wrap` | Yes | Search wraps past end/beginning of file |
| `motion_fail` | No | Motion hits buffer boundary (e.g., `k` at line 1) |
| `mode_change` | No | Transition between modes |
| `save_complete` | No | Successful `:w` |
| `macro_end` | No | Macro playback ends |

## Configuration

### Events

| Setting | Type | Description |
|---|---|---|
| `bell.on_error` | boolean | Bell on invalid commands |
| `bell.on_search_wrap` | boolean | Bell when search wraps |
| `bell.on_motion_fail` | boolean | Bell when motion fails |
| `bell.on_mode_change` | boolean | Bell on mode transitions |
| `bell.on_save` | boolean | Bell on successful save |

### Bell type

| Value | Behavior |
|---|---|
| `audible` | Emit `\x07` (terminal interprets as sound) |
| `visual` | Briefly invert the screen colors (flash) |
| `none` | Suppress all bell output |

## Visual bell

### Instead of sound

When `bell.type = "visual"`, the editor inverts all cell colors in the current frame for a brief flash, then restores them.

### Flash duration

| Setting | Default | Description |
|---|---|---|
| `bell.visual_duration_ms` | `100` | Duration of visual flash in milliseconds |

The implementation MUST render the inverted frame, wait the configured duration, then render the normal frame. This requires two render cycles.

## External sounds

### Command hook

| Setting | Type | Description |
|---|---|---|
| `bell.command` | string or null | External command to run instead of terminal bell |

When set, the editor spawns the command asynchronously instead of emitting `\x07`. The command runs fire-and-forget. Example: `bell.command = "paplay /usr/share/sounds/error.ogg"`.

### Per-event sounds

| Setting | Type | Description |
|---|---|---|
| `bell.sounds.error` | string or null | Command for error events |
| `bell.sounds.search_wrap` | string or null | Command for search wrap |

Per-event sound commands override the global `bell.command` for that event.

## Screen reader integration

### Terminal requirements

The editor does not implement speech synthesis directly. It relies on the terminal emulator and system screen reader to provide accessibility. The editor ensures all content is rendered as text cells that screen readers can read.

### Supported screen readers

| OS | Screen reader | Notes |
|---|---|---|
| Linux | Orca | Works via AT-SPI with accessible terminals |
| Windows | NVDA | Works with Windows Terminal and ConEmu |
| macOS | VoiceOver | Works with Terminal.app and iTerm2 |

## Configuration reference

| Setting | Type | Default | Description |
|---|---|---|---|
| `bell.enabled` | boolean | `true` | Master bell switch |
| `bell.type` | string | `audible` | `audible`, `visual`, or `none` |
| `bell.visual_duration_ms` | integer | `100` | Flash duration for visual bell |
| `bell.command` | string | null | External bell command |
| `bell.on_error` | boolean | `true` | Bell on errors |
| `bell.on_search_wrap` | boolean | `true` | Bell on search wrap |
| `bell.on_motion_fail` | boolean | `false` | Bell on failed motions |
| `bell.on_mode_change` | boolean | `false` | Bell on mode change |
| `bell.on_save` | boolean | `false` | Bell on save |

## Related

- Accessibility: [/docs/spec/ux/accessibility.md](/docs/spec/ux/accessibility.md)
- Configuration: [/docs/spec/features/config/implementation.md](/docs/spec/features/config/implementation.md)
