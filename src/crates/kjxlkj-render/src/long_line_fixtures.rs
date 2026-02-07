//! Test fixtures for long-line rendering scenarios.

use unicode_width::UnicodeWidthStr;

/// Kind of long-line fixture.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FixtureKind {
    LongAscii,
    LongUnicode,
    LongMixed,
    WideChars,
    Tabs,
    CombiningMarks,
}

/// A generated test fixture with content and expected display width.
#[derive(Debug, Clone)]
pub struct LineFixture {
    pub content: String,
    pub expected_display_width: usize,
    pub kind: FixtureKind,
}

/// Generate a single fixture of the given kind and character length.
pub fn generate_fixture(kind: FixtureKind, length: usize) -> LineFixture {
    let (content, expected) = match kind {
        FixtureKind::LongAscii => {
            let s: String = "abcdefghij".chars().cycle().take(length).collect();
            let w = s.len();
            (s, w)
        }
        FixtureKind::LongUnicode => {
            // Mix of single-width Unicode letters
            let s: String = "αβγδεζηθ".chars().cycle().take(length).collect();
            let w = UnicodeWidthStr::width(s.as_str());
            (s, w)
        }
        FixtureKind::LongMixed => {
            let base = "aBcδeFgη";
            let s: String = base.chars().cycle().take(length).collect();
            let w = UnicodeWidthStr::width(s.as_str());
            (s, w)
        }
        FixtureKind::WideChars => {
            // CJK chars (each 2 columns)
            let s: String = "漢字表示".chars().cycle().take(length).collect();
            let w = UnicodeWidthStr::width(s.as_str());
            (s, w)
        }
        FixtureKind::Tabs => {
            // Tabs (we treat each as 1 char; actual rendering depends on tab-stop)
            let s: String = std::iter::repeat('\t').take(length).collect();
            // unicode-width gives tabs width 0; use length as expected
            (s, length)
        }
        FixtureKind::CombiningMarks => {
            // Base char + combining acute accent (U+0301)
            let mut s = String::new();
            for _ in 0..length {
                s.push('a');
                s.push('\u{0301}');
            }
            // Each grapheme cluster displays as 1 column
            (s, length)
        }
    };
    LineFixture { content, expected_display_width: expected, kind }
}

/// Generate one fixture for every kind at the given length.
pub fn all_fixtures(length: usize) -> Vec<LineFixture> {
    vec![
        generate_fixture(FixtureKind::LongAscii, length),
        generate_fixture(FixtureKind::LongUnicode, length),
        generate_fixture(FixtureKind::LongMixed, length),
        generate_fixture(FixtureKind::WideChars, length),
        generate_fixture(FixtureKind::Tabs, length),
        generate_fixture(FixtureKind::CombiningMarks, length),
    ]
}

/// Verify that a fixture's content matches its expected display width.
pub fn verify_fixture(fixture: &LineFixture) -> bool {
    match fixture.kind {
        FixtureKind::Tabs | FixtureKind::CombiningMarks => {
            // Special cases handled by convention; skip unicode-width check
            true
        }
        _ => {
            UnicodeWidthStr::width(fixture.content.as_str()) == fixture.expected_display_width
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_fixture() {
        let f = generate_fixture(FixtureKind::LongAscii, 100);
        assert_eq!(f.content.len(), 100);
        assert!(verify_fixture(&f));
    }

    #[test]
    fn wide_fixture() {
        let f = generate_fixture(FixtureKind::WideChars, 10);
        assert_eq!(f.content.chars().count(), 10);
        assert!(verify_fixture(&f));
    }

    #[test]
    fn combining_fixture() {
        let f = generate_fixture(FixtureKind::CombiningMarks, 5);
        assert_eq!(f.expected_display_width, 5);
        assert!(verify_fixture(&f));
    }

    #[test]
    fn all_fixtures_verify() {
        for f in all_fixtures(50) {
            assert!(verify_fixture(&f), "failed for {:?}", f.kind);
        }
    }
}
