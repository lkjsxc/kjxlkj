# Encoding Commands

Managing file character encodings.

## Key Options

### Three Settings

| Option         | Purpose               |
|----------------|----------------------|
| `encoding`     | Internal encoding    |
| `fileencoding` | File's encoding      |
| `fileencodings`| Detection order      |

## Internal Encoding

### System Wide

`:set encoding=utf-8` sets the internal representation for all
buffers, registers, and expressions. This is a global option; it
MUST be set before opening files. Changing it after startup
re-interprets existing buffer contents (may cause data loss).

### Recommendation

Always use UTF-8 internally.

## File Encoding

### Per File

`:set fileencoding=latin1` sets the encoding used when writing the
current buffer to disk. When empty, the value of `encoding` is
used. Changed per-buffer; does not affect other open files.

### Check

`:set fileencoding?` displays the detected or assigned encoding
for the current buffer. Abbreviated form: `:set fenc?`.

### Common Values

| Encoding   | Description          |
|------------|---------------------|
| utf-8      | Unicode (default)   |
| latin1     | ISO-8859-1          |
| utf-16     | Unicode 16-bit      |
| utf-16le   | UTF-16 Little Endian|
| cp1252     | Windows Western     |
| euc-jp     | Japanese            |
| shift-jis  | Japanese            |
| gbk        | Chinese             |
| big5       | Chinese Traditional |

## Detection Order

### Automatic

`:set fileencodings=ucs-bom,utf-8,latin1` defines the ordered list
of encodings to try when reading a file. The editor reads the raw
bytes from disk and attempts each encoding in sequence. The first
encoding that produces no invalid byte sequences is selected and
assigned to `fileencoding` for that buffer.

### Priority

Tries each in order until valid.

## Reading Files

### Force Encoding

`:e ++enc=latin1 file.txt` opens `file.txt` forcing Latin-1
decoding, overriding automatic detection. The `++enc` option
applies only to that single `:edit` invocation.

### Re-read

`:e ++enc=shift-jis` re-reads the current buffer from disk using
the specified encoding. The buffer contents are replaced with the
newly decoded text. Unsaved changes are discarded (use `!` if the
buffer is modified: `:e! ++enc=shift-jis`).

## Writing Files

### Force Encoding

`:w ++enc=utf-16le file.txt` writes the buffer to disk using
UTF-16LE encoding regardless of the current `fileencoding` value.
The buffer's `fileencoding` is NOT changed by this command.

### Convert

`:set fileencoding=utf-8 | w` converts the file by changing the
encoding setting then writing. The conversion is performed during
the write: internal UTF-8 text is re-encoded to the target. If
the target encoding cannot represent a character, the behavior is
controlled by the `bad_char` option (see Bad Characters below).

## BOM Handling

### Byte Order Mark

The `bomb` boolean option controls BOM (Byte Order Mark) insertion.
When `true`, a BOM is written at the start of the file for
encodings that support it (utf-8, utf-16, utf-16le, utf-16be).

### Check

`:set bomb?` shows whether the current buffer has a BOM. When
reading, a BOM is detected automatically if `ucs-bom` appears in
`fileencodings`; the `bomb` option is then set accordingly.

### Force BOM

`:set bomb | w` adds a BOM to the file on next write.
`:set nobomb | w` removes an existing BOM on next write.

## Line Endings

### File Format

`:set fileformat=unix` sets the line ending style. Allowed values:

| Value | Line Ending | Bytes |
|-------|-------------|-------|
| `unix` | LF | `0x0A` |
| `dos` | CRLF | `0x0D 0x0A` |
| `mac` | CR | `0x0D` |

### Check

`:set fileformat?` shows the current buffer's line ending style.
Abbreviated: `:set ff?`.

### Force on Read

`:e ++ff=dos file.txt` opens a file interpreting line endings as
CRLF, overriding automatic detection.

### Force on Write

`:set fileformat=unix | w` converts line endings on write. The
buffer's line endings are changed to LF and the file is rewritten.

## Detection Order

### Format Detection

`:set fileformats=unix,dos,mac` sets the detection priority for
line endings when reading files. The first format that produces a
consistent parse is used. Default: `unix,dos`.

## Conversion Commands

### To UTF-8

`:set fileencoding=utf-8 | w` converts the current file to UTF-8.
To also add a BOM: `:set bomb fileencoding=utf-8 | w`.
To convert and strip BOM: `:set nobomb fileencoding=utf-8 | w`.

### Batch Convert

`:argdo set fileencoding=utf-8 | w` converts all files in the
argument list. Combine with `:args *.txt` to target specific files.
Use `:bufdo set fenc=utf-8 | w` to convert all open buffers.

## Troubleshooting

### Garbled Text

1. Check encoding: `:set fenc?`
2. Re-read with correct: `:e ++enc=xxx`

### Common Issues

| Symptom        | Likely Cause          |
|----------------|-----------------------|
| `Ã©` for `é`   | UTF-8 as Latin-1      |
| `?` chars      | Encoding mismatch     |
| `<XX>` codes   | Invalid bytes         |

## Bad Characters

### Handling

When a byte sequence is invalid for the selected encoding, the
behavior depends on the `++bad` option:

| Value | Behavior |
|-------|----------|
| `++bad=?` | Replace invalid bytes with `?` (default) |
| `++bad=.` | Replace with `.` |
| `++bad=drop` | Silently discard invalid bytes |
| `++bad=keep` | Preserve raw bytes unchanged |

Example: `:e ++enc=utf-8 ++bad=? file.bin` opens with replacement.
On write, if `fileencoding` cannot represent a character, the same
substitution applies. A warning shows the count of replaced chars.
