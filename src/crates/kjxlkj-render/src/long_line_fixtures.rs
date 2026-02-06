/// Minimal reproduction fixtures for long line testing.

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FixtureKind {
    LongAscii,
    LongUnicode,
    LongMixed,
    WideChars,
    Tabs,
    CombiningMarks,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LineFixture {
    pub(crate) kind: FixtureKind,
    pub(crate) content: String,
    pub(crate) expected_display_width: usize,
}

pub(crate) fn long_ascii_line(cols: usize) -> String {
    "a".repeat(cols)
}

pub(crate) fn long_unicode_line(cols: usize) -> String {
    let pattern = ['あ', 'い', 'う', 'え', 'お'];
    pattern.iter().cycle().take(cols).collect()
}

pub(crate) fn long_mixed_line(cols: usize) -> String {
    let mut s = String::new();
    for i in 0..cols {
        if i % 2 == 0 { s.push('a'); } else { s.push('あ'); }
    }
    s
}

pub(crate) fn wide_char_line(cols: usize) -> String {
    std::iter::repeat('字').take(cols).collect()
}

pub(crate) fn tab_heavy_line(count: usize) -> String {
    let mut s = String::new();
    for i in 0..count {
        s.push('\t');
        s.push_str(&format!("w{}", i));
    }
    s
}

pub(crate) fn generate_fixture(kind: FixtureKind, length: usize) -> LineFixture {
    match kind {
        FixtureKind::LongAscii => {
            let content = long_ascii_line(length);
            LineFixture { kind: FixtureKind::LongAscii, expected_display_width: length, content }
        }
        FixtureKind::LongUnicode => {
            let content = long_unicode_line(length);
            LineFixture { kind: FixtureKind::LongUnicode, expected_display_width: length * 2, content }
        }
        FixtureKind::LongMixed => {
            let content = long_mixed_line(length);
            let wide_count = length / 2;
            LineFixture { kind: FixtureKind::LongMixed, expected_display_width: length + wide_count, content }
        }
        FixtureKind::WideChars => {
            let content = wide_char_line(length);
            LineFixture { kind: FixtureKind::WideChars, expected_display_width: length * 2, content }
        }
        FixtureKind::Tabs => {
            let content = tab_heavy_line(length);
            LineFixture { kind: FixtureKind::Tabs, expected_display_width: length * 8 + length * 2, content }
        }
        FixtureKind::CombiningMarks => {
            let content: String = (0..length).map(|_| "e\u{0301}").collect();
            LineFixture { kind: FixtureKind::CombiningMarks, expected_display_width: length, content }
        }
    }
}

pub(crate) fn all_fixtures() -> Vec<LineFixture> {
    vec![
        generate_fixture(FixtureKind::LongAscii, 1000),
        generate_fixture(FixtureKind::LongUnicode, 1000),
        generate_fixture(FixtureKind::LongMixed, 1000),
        generate_fixture(FixtureKind::WideChars, 1000),
        generate_fixture(FixtureKind::Tabs, 1000),
        generate_fixture(FixtureKind::CombiningMarks, 1000),
    ]
}

pub(crate) fn verify_fixture(fixture: &LineFixture) -> bool {
    !fixture.content.is_empty() && fixture.expected_display_width > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_fixture() {
        let f = generate_fixture(FixtureKind::LongAscii, 100);
        assert_eq!(f.content.len(), 100);
        assert_eq!(f.expected_display_width, 100);
    }

    #[test]
    fn unicode_fixture() {
        let f = generate_fixture(FixtureKind::LongUnicode, 50);
        assert_eq!(f.content.chars().count(), 50);
        assert_eq!(f.expected_display_width, 100);
    }

    #[test]
    fn mixed_fixture() {
        let f = generate_fixture(FixtureKind::LongMixed, 20);
        assert_eq!(f.content.chars().count(), 20);
        assert!(f.expected_display_width > 20);
    }

    #[test]
    fn wide_char_fixture() {
        let f = generate_fixture(FixtureKind::WideChars, 30);
        assert_eq!(f.content.chars().count(), 30);
        assert_eq!(f.expected_display_width, 60);
    }

    #[test]
    fn tab_fixture() {
        let f = generate_fixture(FixtureKind::Tabs, 10);
        assert!(f.content.contains('\t'));
        assert!(f.expected_display_width > 0);
    }

    #[test]
    fn all_fixtures_count() {
        assert_eq!(all_fixtures().len(), 6);
    }

    #[test]
    fn verify_all_fixtures() {
        for f in all_fixtures() {
            assert!(verify_fixture(&f), "Failed for {:?}", f.kind);
        }
    }
}
