//! Accessibility model — contrast, focus visibility, and screen reader hints.

/// Contrast ratio result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContrastRatio {
    pub ratio: f64,
    pub passes_aa: bool,
    pub passes_aaa: bool,
}

/// Compute relative luminance from sRGB (0-255).
fn luminance(r: u8, g: u8, b: u8) -> f64 {
    let convert = |c: u8| {
        let s = c as f64 / 255.0;
        if s <= 0.03928 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * convert(r) + 0.7152 * convert(g) + 0.0722 * convert(b)
}

/// Calculate contrast ratio between two colors (WCAG 2.1).
pub fn contrast_ratio(fg: (u8, u8, u8), bg: (u8, u8, u8)) -> ContrastRatio {
    let l1 = luminance(fg.0, fg.1, fg.2);
    let l2 = luminance(bg.0, bg.1, bg.2);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    let ratio = (lighter + 0.05) / (darker + 0.05);
    ContrastRatio { ratio, passes_aa: ratio >= 4.5, passes_aaa: ratio >= 7.0 }
}

/// Focus indicator requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusIndicator { CursorBlock, CursorLine, CursorUnderline, Highlight, Border }

/// Accessibility check result.
#[derive(Debug, Clone)]
pub struct A11yCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

/// Run accessibility checks for a color scheme.
pub fn check_color_scheme(pairs: &[(&str, (u8, u8, u8), (u8, u8, u8))]) -> Vec<A11yCheck> {
    pairs.iter().map(|(name, fg, bg)| {
        let cr = contrast_ratio(*fg, *bg);
        A11yCheck {
            name: name.to_string(),
            passed: cr.passes_aa,
            detail: format!("ratio {:.1}:1 (AA={})", cr.ratio, if cr.passes_aa { "pass" } else { "fail" }),
        }
    }).collect()
}

/// Screen reader hint for a UI element.
#[derive(Debug, Clone)]
pub struct AriaHint {
    pub role: String,
    pub label: String,
    pub live: bool,
}

impl AriaHint {
    pub fn status(label: &str) -> Self { Self { role: "status".into(), label: label.into(), live: true } }
    pub fn editor(label: &str) -> Self { Self { role: "textbox".into(), label: label.into(), live: false } }
    pub fn menu(label: &str) -> Self { Self { role: "menu".into(), label: label.into(), live: false } }
}

/// Check that focus is always visible and unambiguous.
pub fn check_focus_visible(has_cursor: bool, indicator: Option<FocusIndicator>) -> A11yCheck {
    let passed = has_cursor && indicator.is_some();
    A11yCheck {
        name: "focus_visible".into(), passed,
        detail: if passed { "cursor and indicator present".into() } else { "focus not visible".into() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_on_white() {
        let cr = contrast_ratio((0, 0, 0), (255, 255, 255));
        assert!(cr.ratio > 20.0);
        assert!(cr.passes_aa && cr.passes_aaa);
    }

    #[test]
    fn white_on_white_fails() {
        let cr = contrast_ratio((255, 255, 255), (255, 255, 255));
        assert!((cr.ratio - 1.0).abs() < 0.01);
        assert!(!cr.passes_aa);
    }

    #[test]
    fn mid_gray_contrast() {
        let cr = contrast_ratio((128, 128, 128), (255, 255, 255));
        assert!(cr.ratio > 1.0 && cr.ratio < 10.0);
    }

    #[test]
    fn color_scheme_check() {
        let pairs = vec![
            ("text", (200, 200, 200), (30, 30, 30)),
            ("dim", (100, 100, 100), (90, 90, 90)),
        ];
        let results = check_color_scheme(&pairs);
        assert!(results[0].passed); // high contrast
        assert!(!results[1].passed); // low contrast
    }

    #[test]
    fn focus_visibility() {
        let ok = check_focus_visible(true, Some(FocusIndicator::CursorBlock));
        assert!(ok.passed);
        let fail = check_focus_visible(false, None);
        assert!(!fail.passed);
    }

    #[test]
    fn aria_hints() {
        let s = AriaHint::status("Ready");
        assert!(s.live);
        let e = AriaHint::editor("Buffer");
        assert!(!e.live);
        assert_eq!(AriaHint::menu("Completion").role, "menu");
    }

    #[test]
    fn symmetry() {
        let cr1 = contrast_ratio((255, 0, 0), (0, 0, 255));
        let cr2 = contrast_ratio((0, 0, 255), (255, 0, 0));
        assert!((cr1.ratio - cr2.ratio).abs() < 0.01);
    }
}
