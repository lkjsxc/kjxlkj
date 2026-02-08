# Encoding Commands

Back: [/docs/spec/commands/file/README.md](/docs/spec/commands/file/README.md)

Commands for changing file encoding and line endings.

## File encoding

| Command | Description |
|---|---|
| `:set fileencoding={enc}` | Set the encoding for the current buffer |
| `:set fileencoding?` | Query the current encoding |
| `:e ++enc={enc} {file}` | Open file with specific encoding |
| `:w ++enc={enc}` | Write file with specific encoding |

### Supported encodings

| Encoding | Description |
|---|---|
| `utf-8` | UTF-8 (default) |
| `latin1` / `iso-8859-1` | Latin-1 / ISO 8859-1 |
| `utf-16` | UTF-16 with BOM |
| `utf-16le` | UTF-16 little-endian |
| `utf-16be` | UTF-16 big-endian |
| `shift_jis` / `sjis` | Shift JIS (Japanese) |
| `euc-jp` | EUC-JP (Japanese) |
| `euc-kr` | EUC-KR (Korean) |
| `gbk` / `gb2312` | GBK / GB2312 (Chinese) |

## Encoding detection

On file open, the editor attempts to detect the encoding:

1. Check for BOM (Byte Order Mark)
2. Try UTF-8
3. Fall back to `fileencodings` list (try each in order)
4. Fall back to `latin1` if all else fails

| Setting | Default | Description |
|---|---|---|
| `fileencodings` | `["utf-8", "sjis", "euc-jp", "latin1"]` | Encoding detection order |

## File format (line endings)

| Command | Description |
|---|---|
| `:set fileformat={ff}` | Set line ending format |
| `:set fileformat?` | Query current format |
| `:e ++ff={ff} {file}` | Open file with specific format |
| `:w ++ff={ff}` | Write with specific format |

| Format | Line ending | OS |
|---|---|---|
| `unix` | LF (`\n`) | Linux, macOS |
| `dos` | CRLF (`\r\n`) | Windows |

## BOM (Byte Order Mark)

| Setting | Default | Description |
|---|---|---|
| `bomb` | `false` | Write BOM at start of file |

BOM is detected on read regardless of this setting.

## Conversion

When `fileencoding` is changed for a buffer, the content remains the same internally (stored as UTF-8). The encoding is applied only when writing to disk.

## Related

- File operations: [/docs/spec/commands/file/file-operations.md](/docs/spec/commands/file/file-operations.md)
- Unicode: [/docs/technical/unicode.md](/docs/technical/unicode.md)
