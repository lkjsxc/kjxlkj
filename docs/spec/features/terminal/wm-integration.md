# Window Manager Integration

Integrating kjxlkj with desktop window managers.

## Terminal Selection

### Recommended Terminals

| Terminal | Platform | Features |
|----------|----------|----------|
| Kitty | Linux/macOS | Best performance, true color, ligatures |
| Alacritty | Cross-platform | GPU-accelerated, minimal |
| WezTerm | Cross-platform | Multiplexing, Lua config |
| iTerm2 | macOS | Native integration, tmux control |
| Windows Terminal | Windows | GPU-accelerated, profiles |
| foot | Linux (Wayland) | Lightweight, fast |

### Minimum Requirements

Terminals must support: 256 colors (true color preferred),
alternate screen buffer, bracketed paste mode, focus
events, and Unicode.

## Desktop Entries

### Linux .desktop File

Location: `~/.local/share/applications/kjxlkj.desktop`
or `/usr/share/applications/kjxlkj.desktop`.

Contents:
- `[Desktop Entry]` section
- `Name=kjxlkj`
- `Exec=kitty kjxlkj %F` (wraps in preferred terminal)
- `Type=Application`
- `Categories=TextEditor;Development;`
- `MimeType=text/plain;text/x-rust;...`
- `Terminal=false` (already launches in terminal)
- `Icon=kjxlkj`

## File Associations

### Set Default Editor

On Linux: `xdg-mime default kjxlkj.desktop text/plain`
Or set `EDITOR=kjxlkj` and `VISUAL=kjxlkj` in shell profile.

### MIME Types

The `.desktop` file declares supported MIME types.
Common types: `text/plain`, `text/x-c`, `text/x-rust`,
`text/x-python`, `application/json`, `text/markdown`,
`text/x-shellscript`, `text/html`, `text/css`.

## Keybindings

### GNOME

Settings > Keyboard > Custom Shortcuts:
Name: kjxlkj, Command: `kitty kjxlkj`, Shortcut: chosen key.

### KDE

Settings > Shortcuts > Custom Shortcuts > New > Global.
Set command to terminal wrapper launching kjxlkj.

### i3/Sway

Config line: `bindsym $mod+e exec kitty kjxlkj`
For floating: `for_window [title="kjxlkj"] floating enable`

## macOS Integration

### App Bundle

Create an app bundle wrapping the terminal launch:
`kjxlkj.app/Contents/MacOS/kjxlkj-launcher` (shell script
that opens the configured terminal with kjxlkj).

### Default Editor

`duti -s com.kjxlkj .rs editor` sets kjxlkj as default
for Rust files. Also set via Finder > Get Info.

## Windows Integration

### Context Menu

Registry entries under `HKEY_CLASSES_ROOT\*\shell\kjxlkj`
add "Open with kjxlkj" to right-click menu.

### Windows Terminal Profile

Add a profile in Windows Terminal settings JSON:
`name: "kjxlkj"`, `commandline: "kjxlkj.exe"`,
`icon: "path/to/icon"`.

## Terminal Flags

### Quick Open

`kjxlkj +{line} {file}` opens a file at a specific line.
`kjxlkj -O file1 file2` opens files in vertical splits.
`kjxlkj -o file1 file2` opens files in horizontal splits.

### Floating Window

On tiling WMs, mark the kjxlkj window as floating for
popup usage: `kjxlkj --class kjxlkj-float {file}` with
WM rule `for_window [app_id="kjxlkj-float"] floating enable`.

## Environment

### Editor Variables

Set these in shell profile for tool integration:
- `EDITOR=kjxlkj` (for short-lived editing, e.g., git commit)
- `VISUAL=kjxlkj` (for interactive editing)
- `SUDO_EDITOR=kjxlkj` (for sudoedit)

### Git Integration

Git uses `$EDITOR` for commit messages, interactive rebase,
and other editing tasks. kjxlkj exits with code 0 on `:wq`
and non-zero on `:cq` for aborting.

## Focus Handling

### Focus Events

When the terminal supports focus events (`CSI I` / `CSI O`),
kjxlkj receives focus-gained and focus-lost notifications.

### On Focus Lost

Configurable behavior: auto-save modified buffers, check
for external file changes (`:checktime`), or do nothing.
Setting: `focus_lost_autosave = true`.

### On Focus Gained

Re-check file modification times. If a file changed
externally, prompt to reload or auto-reload based on
`autoread` setting.

## Multiple Instances

### Single Instance

Not enforced by default. Each invocation starts a new
process. No IPC between instances.

### New Window

`kjxlkj --remote {file}` could send files to an existing
instance (future feature). Currently each invocation is
independent.
