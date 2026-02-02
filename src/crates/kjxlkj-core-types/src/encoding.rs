//! File encoding support.
//!
//! Detection and conversion of file encodings.

use serde::{Deserialize, Serialize};

/// Known file encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Encoding {
    /// UTF-8 (default).
    #[default]
    Utf8,
    /// UTF-8 with BOM.
    Utf8Bom,
    /// UTF-16 Little Endian.
    Utf16Le,
    /// UTF-16 Big Endian.
    Utf16Be,
    /// Latin-1 (ISO-8859-1).
    Latin1,
    /// Windows-1252.
    Windows1252,
    /// ASCII.
    Ascii,
}

impl Encoding {
    /// Returns the encoding name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Utf8Bom => "utf-8-bom",
            Self::Utf16Le => "utf-16le",
            Self::Utf16Be => "utf-16be",
            Self::Latin1 => "latin1",
            Self::Windows1252 => "cp1252",
            Self::Ascii => "ascii",
        }
    }

    /// Parses an encoding from name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "utf-8" | "utf8" => Some(Self::Utf8),
            "utf-8-bom" | "utf8-bom" => Some(Self::Utf8Bom),
            "utf-16le" | "utf16le" => Some(Self::Utf16Le),
            "utf-16be" | "utf16be" => Some(Self::Utf16Be),
            "latin1" | "iso-8859-1" | "iso88591" => Some(Self::Latin1),
            "cp1252" | "windows-1252" => Some(Self::Windows1252),
            "ascii" | "us-ascii" => Some(Self::Ascii),
            _ => None,
        }
    }
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LineEnding {
    /// Unix (LF).
    #[default]
    Lf,
    /// Windows (CRLF).
    CrLf,
    /// Classic Mac (CR).
    Cr,
}

impl LineEnding {
    /// Returns the line ending string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
            Self::Cr => "\r",
        }
    }

    /// Returns the display name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Lf => "LF",
            Self::CrLf => "CRLF",
            Self::Cr => "CR",
        }
    }

    /// Detects line ending from content.
    pub fn detect(content: &str) -> Self {
        if content.contains("\r\n") {
            Self::CrLf
        } else if content.contains('\r') {
            Self::Cr
        } else {
            Self::Lf
        }
    }
}

/// Detects encoding from byte content.
pub fn detect_encoding(bytes: &[u8]) -> Encoding {
    // Check BOM.
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Encoding::Utf8Bom;
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return Encoding::Utf16Le;
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return Encoding::Utf16Be;
    }

    // Check if valid UTF-8.
    if std::str::from_utf8(bytes).is_ok() {
        // Check if pure ASCII.
        if bytes.iter().all(|&b| b < 128) {
            return Encoding::Ascii;
        }
        return Encoding::Utf8;
    }

    // Assume Latin-1 for non-UTF-8.
    Encoding::Latin1
}

/// File metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File encoding.
    pub encoding: Encoding,
    /// Line ending style.
    pub line_ending: LineEnding,
    /// Final newline present.
    pub final_newline: bool,
}

impl FileMetadata {
    /// Creates new metadata from content.
    pub fn from_content(bytes: &[u8], text: &str) -> Self {
        Self {
            encoding: detect_encoding(bytes),
            line_ending: LineEnding::detect(text),
            final_newline: text.ends_with('\n'),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_name() {
        assert_eq!(Encoding::Utf8.name(), "utf-8");
        assert_eq!(Encoding::Utf16Le.name(), "utf-16le");
    }

    #[test]
    fn test_encoding_from_name() {
        assert_eq!(Encoding::from_name("utf-8"), Some(Encoding::Utf8));
        assert_eq!(Encoding::from_name("UTF8"), Some(Encoding::Utf8));
        assert_eq!(Encoding::from_name("unknown"), None);
    }

    #[test]
    fn test_line_ending_as_str() {
        assert_eq!(LineEnding::Lf.as_str(), "\n");
        assert_eq!(LineEnding::CrLf.as_str(), "\r\n");
    }

    #[test]
    fn test_line_ending_detect_lf() {
        assert_eq!(LineEnding::detect("line1\nline2"), LineEnding::Lf);
    }

    #[test]
    fn test_line_ending_detect_crlf() {
        assert_eq!(LineEnding::detect("line1\r\nline2"), LineEnding::CrLf);
    }

    #[test]
    fn test_detect_encoding_utf8() {
        let bytes = "hello".as_bytes();
        assert_eq!(detect_encoding(bytes), Encoding::Ascii);
    }

    #[test]
    fn test_detect_encoding_utf8_bom() {
        let bytes = &[0xEF, 0xBB, 0xBF, b'h', b'i'];
        assert_eq!(detect_encoding(bytes), Encoding::Utf8Bom);
    }

    #[test]
    fn test_file_metadata() {
        let text = "hello\nworld\n";
        let meta = FileMetadata::from_content(text.as_bytes(), text);
        assert_eq!(meta.encoding, Encoding::Ascii);
        assert_eq!(meta.line_ending, LineEnding::Lf);
        assert!(meta.final_newline);
    }
}
