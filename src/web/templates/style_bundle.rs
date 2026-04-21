//! Minified inline stylesheet bundle

use once_cell::sync::Lazy;

const BASE_CSS: &str = include_str!("base.css");
const CONTENT_CSS: &str = include_str!("content.css");
const EXTERNAL_EMBEDS_CSS: &str = include_str!("external_embeds.css");
const CONTROLS_CSS: &str = include_str!("controls.css");
const SHELL_CSS: &str = include_str!("shell.css");
const SURFACES_CSS: &str = include_str!("surfaces.css");
const RESOURCE_LIST_CSS: &str = include_str!("resource_list.css");
const SUMMARY_CARDS_CSS: &str = include_str!("summary_cards.css");
const OVERVIEW_CSS: &str = include_str!("overview.css");
const LIVE_CSS: &str = include_str!("live.css");
const RESPONSIVE_CSS: &str = include_str!("responsive.css");
const PAGE_CSS: &str = include_str!("page.css");
const FAVORITES_CSS: &str = include_str!("favorites.css");
const EDITOR_CSS: &str = include_str!("editor.css");
const EDITOR_EXTRA_CSS: &str = include_str!("editor_extra.css");
const RESOURCE_FOCUS_CSS: &str = include_str!("resource_focus.css");
const SETTINGS_FLAT_CSS: &str = include_str!("settings_flat.css");
const SETTINGS_CSS: &str = include_str!("settings.css");

static STYLESHEET: Lazy<String> = Lazy::new(|| minify_css(raw_css()));

pub fn stylesheet() -> &'static str {
    &STYLESHEET
}

fn raw_css() -> String {
    [
        BASE_CSS,
        CONTENT_CSS,
        EXTERNAL_EMBEDS_CSS,
        CONTROLS_CSS,
        SHELL_CSS,
        SURFACES_CSS,
        RESOURCE_LIST_CSS,
        SUMMARY_CARDS_CSS,
        OVERVIEW_CSS,
        LIVE_CSS,
        RESPONSIVE_CSS,
        PAGE_CSS,
        FAVORITES_CSS,
        EDITOR_CSS,
        EDITOR_EXTRA_CSS,
        RESOURCE_FOCUS_CSS,
        SETTINGS_FLAT_CSS,
        SETTINGS_CSS,
    ]
    .join("\n")
}

fn minify_css(input: String) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut pending_space = false;
    while let Some(ch) = chars.next() {
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            skip_comment(&mut chars);
            continue;
        }
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && needs_space(output.chars().last(), ch) {
            output.push(' ');
        }
        pending_space = false;
        if matches!(ch, ';' | ',' | '{' | '}' | '>' | '+' | '~') && output.ends_with(' ') {
            output.pop();
        }
        output.push(ch);
    }
    output
}

fn skip_comment<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'/') {
            chars.next();
            return;
        }
    }
}

fn needs_space(previous: Option<char>, next: char) -> bool {
    let Some(previous) = previous else {
        return false;
    };
    if matches!(
        previous,
        '{' | '}' | ':' | ';' | ',' | '>' | '+' | '~' | '('
    ) {
        return false;
    }
    if matches!(next, '{' | '}' | ';' | ',' | '>' | '+' | '~' | ')') {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::minify_css;

    #[test]
    fn minify_preserves_selector_spacing() {
        let css = concat!(
            ".pager .btn { width: 100%; }",
            ".prose :is(p, li) { margin: 0; }",
            "body.rail-open .shell-rail { transform: none; }",
            ".public-resource-grid .resource-row { gap: 1rem; }",
        );

        let minified = minify_css(css.to_string());

        assert!(minified.contains(".pager .btn"));
        assert!(minified.contains(".prose :is"));
        assert!(minified.contains("body.rail-open .shell-rail"));
        assert!(minified.contains(".public-resource-grid .resource-row"));
    }
}
