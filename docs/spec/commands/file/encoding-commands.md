# Encoding Commands

Managing file character encodings.

## Overview

Handle different character
encodings when reading/writing.

## Key Options

### Three Settings

| Option         | Purpose               |
|----------------|----------------------|
| `encoding`     | Internal encoding    |
| `fileencoding` | File's encoding      |
| `fileencodings`| Detection order      |

## Internal Encoding

### System Wide


### Recommendation

Always use UTF-8 internally.

## File Encoding

### Per File


### Check


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


### Priority

Tries each in order until valid.

## Reading Files

### Force Encoding


### Re-read


## Writing Files

### Force Encoding


### Convert


## BOM Handling

### Byte Order Mark


### Check


### Force BOM


## Line Endings

### File Format


### Check


### Force on Read


### Force on Write


## Detection Order

### Format Detection


## Conversion Commands

### To UTF-8


### Batch Convert


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

