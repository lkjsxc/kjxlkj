//! SGR (Select Graphic Rendition) parsing.

use crate::cell::Color;
use crate::screen::Screen;

/// Parse and apply SGR parameters to screen attributes.
pub fn dispatch_sgr(params: &[u16], screen: &mut Screen) {
    let attrs = &mut screen.attrs;
    if params.is_empty() {
        attrs.reset();
        return;
    }
    let mut i = 0;
    while i < params.len() {
        match params[i] {
            0 => attrs.reset(),
            1 => attrs.bold = true,
            2 => attrs.dim = true,
            3 => attrs.italic = true,
            4 => attrs.underline = true,
            7 => attrs.reverse = true,
            8 => attrs.hidden = true,
            9 => attrs.strikethrough = true,
            22 => {
                attrs.bold = false;
                attrs.dim = false;
            }
            23 => attrs.italic = false,
            24 => attrs.underline = false,
            27 => attrs.reverse = false,
            29 => attrs.strikethrough = false,
            30..=37 => attrs.fg = Color::Indexed(params[i] as u8 - 30),
            38 => {
                i += 1;
                parse_extended_color(params, &mut i, &mut attrs.fg);
                continue;
            }
            39 => attrs.fg = Color::Default,
            40..=47 => attrs.bg = Color::Indexed(params[i] as u8 - 40),
            48 => {
                i += 1;
                parse_extended_color(params, &mut i, &mut attrs.bg);
                continue;
            }
            49 => attrs.bg = Color::Default,
            90..=97 => attrs.fg = Color::Indexed(params[i] as u8 - 90 + 8),
            100..=107 => attrs.bg = Color::Indexed(params[i] as u8 - 100 + 8),
            _ => {}
        }
        i += 1;
    }
}

fn parse_extended_color(params: &[u16], i: &mut usize, color: &mut Color) {
    if *i >= params.len() {
        return;
    }
    match params[*i] {
        5 => {
            *i += 1;
            if *i < params.len() {
                *color = Color::Indexed(params[*i] as u8);
                *i += 1;
            }
        }
        2 => {
            if *i + 3 < params.len() {
                let r = params[*i + 1] as u8;
                let g = params[*i + 2] as u8;
                let b = params[*i + 3] as u8;
                *color = Color::Rgb(r, g, b);
                *i += 4;
            } else {
                *i = params.len();
            }
        }
        _ => {
            *i += 1;
        }
    }
}
