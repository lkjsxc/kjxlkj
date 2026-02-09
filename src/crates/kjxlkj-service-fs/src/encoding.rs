//! Encoding detection for file reading.

/// Detected encoding of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Latin1,
    Unknown,
}

/// Detect the encoding of file content.
///
/// Uses BOM detection and UTF-8 validation.
pub fn detect_encoding(bytes: &[u8]) -> Encoding {
    // Check for UTF-8 BOM.
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Encoding::Utf8;
    }

    // Try UTF-8 validation.
    if std::str::from_utf8(bytes).is_ok() {
        return Encoding::Utf8;
    }

    // Fallback: assume Latin-1 (byte-transparent).
    Encoding::Latin1
}

/// Strip BOM from content if present.
pub fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{FEFF}').unwrap_or(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_detection() {
        assert_eq!(detect_encoding(b"hello world"), Encoding::Utf8);
    }

    #[test]
    fn bom_detection() {
        let with_bom = b"\xef\xbb\xbfhello";
        assert_eq!(detect_encoding(with_bom), Encoding::Utf8);
    }

    #[test]
    fn strip_bom_present() {
        assert_eq!(strip_bom("\u{FEFF}hello"), "hello");
    }

    #[test]
    fn strip_bom_absent() {
        assert_eq!(strip_bom("hello"), "hello");
    }
}
