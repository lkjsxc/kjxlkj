use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderDiagnostics {
    pub bounds_ok: bool,
    pub cursor_visible: bool,
    pub cursor_on_continuation: bool,
    pub cursor_span: u8,
    pub wrap_signature: u64,
}

#[derive(Debug, Clone, Copy)]
struct CursorState {
    visible: bool,
    on_continuation: bool,
    span: u8,
}

pub fn compute_render_diagnostics(
    line: &str,
    cursor_offset: usize,
    cols: u16,
    rows: u16,
) -> RenderDiagnostics {
    if cols == 0 || rows == 0 {
        return RenderDiagnostics {
            bounds_ok: true,
            cursor_visible: false,
            cursor_on_continuation: false,
            cursor_span: 0,
            wrap_signature: 0,
        };
    }

    let chars: Vec<char> = line.chars().collect();
    let clamped_cursor = cursor_offset.min(chars.len());
    let mut row: u16 = 0;
    let mut col: u16 = 0;
    let mut bounds_ok = true;
    let mut cursor = CursorState {
        visible: false,
        on_continuation: false,
        span: 0,
    };
    let mut wraps: Vec<(u16, u16, usize)> = Vec::new();
    let mut owner_col: Option<(u16, u16)> = None;
    let mut owner_span: Option<u8> = None;

    for (idx, ch) in chars.iter().copied().enumerate() {
        let width = display_width(ch);
        if width == 0 {
            if idx == clamped_cursor {
                if let Some((r, c)) = owner_col {
                    cursor.visible = r < rows && c < cols;
                    cursor.on_continuation = false;
                    cursor.span = owner_span.unwrap_or(1);
                }
            }
            continue;
        }

        if width == 2 && cols.saturating_sub(col) == 1 {
            if row < rows {
                wraps.push((row, col, idx));
            }
            col = col.saturating_add(1);
            if col >= cols {
                row = row.saturating_add(1);
                col = 0;
            }
        }

        if row >= rows {
            break;
        }
        if row >= rows || col >= cols {
            bounds_ok = false;
            break;
        }
        if idx == clamped_cursor {
            cursor.visible = true;
            cursor.on_continuation = false;
            cursor.span = if width == 2 { 2 } else { 1 };
        }
        owner_col = Some((row, col));
        owner_span = Some(if width == 2 { 2 } else { 1 });
        wraps.push((row, col, idx));
        col = col.saturating_add(1);
        if col >= cols {
            row = row.saturating_add(1);
            col = 0;
        }

        if width == 2 {
            if row >= rows {
                break;
            }
            if row >= rows || col >= cols {
                bounds_ok = false;
                break;
            }
            if idx == clamped_cursor {
                cursor.on_continuation = false;
            }
            wraps.push((row, col, idx));
            col = col.saturating_add(1);
            if col >= cols {
                row = row.saturating_add(1);
                col = 0;
            }
        }
    }

    if clamped_cursor == chars.len() {
        cursor.visible = row < rows && col < cols;
        cursor.on_continuation = false;
        cursor.span = 1;
    }
    if chars.is_empty() && clamped_cursor == 0 {
        cursor.visible = true;
        cursor.on_continuation = false;
        cursor.span = 1;
    }

    let mut hasher = DefaultHasher::new();
    cols.hash(&mut hasher);
    rows.hash(&mut hasher);
    wraps.hash(&mut hasher);
    let wrap_signature = hasher.finish();
    RenderDiagnostics {
        bounds_ok,
        cursor_visible: cursor.visible,
        cursor_on_continuation: cursor.on_continuation,
        cursor_span: cursor.span,
        wrap_signature,
    }
}

fn display_width(ch: char) -> u16 {
    if ch.is_control() {
        return 0;
    }
    if is_combining_mark(ch) {
        return 0;
    }
    if ch.is_ascii() {
        return 1;
    }
    2
}

fn is_combining_mark(ch: char) -> bool {
    let code = ch as u32;
    (0x0300..=0x036F).contains(&code)
        || (0x1AB0..=0x1AFF).contains(&code)
        || (0x1DC0..=0x1DFF).contains(&code)
        || (0x20D0..=0x20FF).contains(&code)
        || (0xFE20..=0xFE2F).contains(&code)
}
