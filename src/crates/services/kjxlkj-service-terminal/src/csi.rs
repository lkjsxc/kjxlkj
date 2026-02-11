//! CSI dispatch and SGR parameter handling for the VT100 parser.
use crate::screen::{Screen, Attr};

fn param(params: &[u16], idx: usize, default: u16) -> u16 {
    params.get(idx).copied().filter(|&v| v > 0).unwrap_or(default)
}

pub fn dispatch(params: &[u16], private_marker: bool, final_byte: u8, screen: &mut Screen) {
    if private_marker {
        let mode = param(params, 0, 0);
        match final_byte {
            b'h' => screen.set_private_mode(mode, true),
            b'l' => screen.set_private_mode(mode, false),
            _ => {}
        }
        return;
    }
    match final_byte {
        b'A' => screen.move_cursor_up(param(params, 0, 1)),
        b'B' => screen.move_cursor_down(param(params, 0, 1)),
        b'C' => screen.move_cursor_right(param(params, 0, 1)),
        b'D' => screen.move_cursor_left(param(params, 0, 1)),
        b'E' => { screen.move_cursor_down(param(params, 0, 1)); screen.carriage_return(); }
        b'F' => { screen.move_cursor_up(param(params, 0, 1)); screen.carriage_return(); }
        b'G' => screen.set_cursor_col(param(params, 0, 1).saturating_sub(1)),
        b'H' | b'f' => screen.set_cursor_pos(
            param(params, 0, 1).saturating_sub(1), param(params, 1, 1).saturating_sub(1)),
        b'J' => screen.erase_display(param(params, 0, 0)),
        b'K' => screen.erase_line(param(params, 0, 0)),
        b'L' => screen.insert_lines(param(params, 0, 1)),
        b'M' => screen.delete_lines(param(params, 0, 1)),
        b'P' => screen.delete_chars(param(params, 0, 1)),
        b'S' => screen.scroll_up(param(params, 0, 1)),
        b'T' => screen.scroll_down(param(params, 0, 1)),
        b'X' => screen.erase_chars(param(params, 0, 1)),
        b'@' => screen.insert_chars(param(params, 0, 1)),
        b'd' => screen.set_cursor_row(param(params, 0, 1).saturating_sub(1)),
        b'h' | b'l' => {} // standard modes (ignored for now)
        b'm' => dispatch_sgr(params, screen),
        b'n' => {} // DSR ignored
        b'r' => screen.set_scroll_region(
            param(params, 0, 1).saturating_sub(1), param(params, 1, 0)),
        b's' => screen.save_cursor(),
        b'u' => screen.restore_cursor(),
        _ => {} // unknown CSI → discard
    }
}

fn dispatch_sgr(params: &[u16], screen: &mut Screen) {
    if params.is_empty() { screen.reset_attr(); return; }
    let mut i = 0;
    while i < params.len() {
        match params[i] {
            0 => screen.reset_attr(),
            1 => screen.set_attr(Attr::Bold, true),
            2 => screen.set_attr(Attr::Dim, true),
            3 => screen.set_attr(Attr::Italic, true),
            4 => screen.set_attr(Attr::Underline, true),
            7 => screen.set_attr(Attr::Reverse, true),
            9 => screen.set_attr(Attr::Strikethrough, true),
            22 => { screen.set_attr(Attr::Bold, false); screen.set_attr(Attr::Dim, false); }
            23 => screen.set_attr(Attr::Italic, false),
            24 => screen.set_attr(Attr::Underline, false),
            27 => screen.set_attr(Attr::Reverse, false),
            29 => screen.set_attr(Attr::Strikethrough, false),
            30..=37 => screen.set_fg(params[i] - 30),
            38 => {
                if i + 1 < params.len() && params[i + 1] == 5 {
                    if i + 2 < params.len() { screen.set_fg_256(params[i + 2]); }
                    i += 2;
                } else if i + 1 < params.len() && params[i + 1] == 2 {
                    if i + 4 < params.len() {
                        screen.set_fg_rgb(params[i+2], params[i+3], params[i+4]);
                    }
                    i += 4;
                }
            }
            39 => screen.reset_fg(),
            40..=47 => screen.set_bg(params[i] - 40),
            48 => {
                if i + 1 < params.len() && params[i + 1] == 5 {
                    if i + 2 < params.len() { screen.set_bg_256(params[i + 2]); }
                    i += 2;
                } else if i + 1 < params.len() && params[i + 1] == 2 {
                    if i + 4 < params.len() {
                        screen.set_bg_rgb(params[i+2], params[i+3], params[i+4]);
                    }
                    i += 4;
                }
            }
            49 => screen.reset_bg(),
            90..=97 => screen.set_fg(params[i] - 90 + 8),
            100..=107 => screen.set_bg(params[i] - 100 + 8),
            _ => {}
        }
        i += 1;
    }
}
