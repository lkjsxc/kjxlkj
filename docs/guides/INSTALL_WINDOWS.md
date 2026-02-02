# Windows Installation

Multiple ways to install kjxlkj on Windows.

## Pre-built Binaries

Download from [GitHub Releases](https://github.com/kjxlkj/kjxlkj/releases):

1. Download `kjxlkj-vX.Y.Z-windows-x86_64.zip`
2. Extract to a folder (e.g., `C:\Program Files\kjxlkj`)
3. Add to PATH (System Properties → Environment Variables)

## Winget (Windows Package Manager)


## Scoop


## Cargo (Build from Source)

Requires Rust toolchain:


## Configuration

Config location: `%APPDATA%\kjxlkj\config.toml`

Or: `C:\Users\<username>\AppData\Roaming\kjxlkj\config.toml`

## Terminal Recommendations

For best experience, use a modern terminal:

- **Windows Terminal** (recommended)
- **Alacritty**
- **WezTerm**

### Windows Terminal Settings

For proper key handling, add to settings.json:


## Troubleshooting

### Arrow keys not working
Use Windows Terminal instead of cmd.exe or PowerShell ISE.

### Colors look wrong
Ensure terminal supports 256 colors or true color.

### Slow startup
Check antivirus isn't scanning the binary on each launch.

## Uninstall

### Manual
Delete the installation folder and remove from PATH.

### Winget

### Scoop
