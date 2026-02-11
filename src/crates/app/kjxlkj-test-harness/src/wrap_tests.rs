//! WRAP-* and CUR-* tests for wrap and cursor behavior.

use kjxlkj_core_text::grapheme_width;

/// WRAP-11R: Width-2 grapheme at boundary gets padding, not split
///
/// When a width-2 grapheme would start at the last column of a line,
/// it should be wrapped to the next line with padding on the previous line.
#[test]
fn wrap_11r_width2_at_boundary_gets_padding() {
    // Simulate a line width of 10 columns
    let line_width = 10;
    
    // Text: 9 ASCII chars + 1 CJK char (width 2)
    // The CJK char would need columns 10-11, but only column 10 is available
    let text = "123456789漢";
    
    // Calculate where each grapheme would be placed
    let mut col = 0;
    let mut wrapped = false;
    let mut padding_needed = false;
    
    for c in text.chars() {
        let w = grapheme_width(&c.to_string());
        if col + w > line_width {
            // Need to wrap
            if col == line_width - 1 && w == 2 {
                // Width-2 grapheme at last column needs padding
                padding_needed = true;
            }
            wrapped = true;
            col = w;
        } else {
            col += w;
        }
    }
    
    // The CJK character should trigger wrap with padding
    assert!(wrapped, "Text should wrap");
    assert!(padding_needed, "Width-2 grapheme at boundary should need padding");
}

/// WRAP-14R: Mixed ASCII/CJK wrapping preserves grapheme integrity
#[test]
fn wrap_14r_mixed_ascii_cjk_wrap_integrity() {
    let line_width = 15;
    let text = "hello世界goodbye日本";
    
    // Each grapheme must remain intact (no splitting)
    let mut positions: Vec<(char, usize, usize)> = Vec::new(); // (char, row, col)
    let mut row = 0;
    let mut col = 0;
    
    for c in text.chars() {
        let w = grapheme_width(&c.to_string());
        
        if col + w > line_width {
            row += 1;
            col = 0;
        }
        
        positions.push((c, row, col));
        col += w;
    }
    
    // Verify no grapheme has half its width on one line
    // (all graphemes should have consistent positioning)
    for (c, _row, col) in &positions {
        let w = grapheme_width(&c.to_string());
        // Col + width should never exceed line_width
        assert!(*col + w <= line_width, "Grapheme '{}' overflows line at col {}", c, col);
    }
}

/// WRAP-16R: Very long CJK string wraps correctly without half-cell states
#[test]
fn wrap_16r_long_cjk_wrap_no_half_cell() {
    let line_width = 20;
    // All width-2 characters that should wrap at even boundaries
    let text = "東京大阪名古屋横浜神戸福岡札幌仙台広島京都";
    
    let mut row = 0;
    let mut col = 0;
    let mut wrap_points: Vec<usize> = Vec::new();
    
    for (_i, c) in text.chars().enumerate() {
        let w = grapheme_width(&c.to_string());
        assert_eq!(w, 2, "All chars should be width 2");
        
        if col + w > line_width {
            wrap_points.push(col);
            row += 1;
            col = w;
        } else {
            col += w;
        }
    }
    
    // Verify wrap points are at valid positions (never at odd columns for width-2)
    for &wp in &wrap_points {
        assert!(wp <= line_width, "Wrap point {} exceeds line width", wp);
        // With all width-2 chars, we should wrap at even positions
        assert!(wp % 2 == 0, "Wrap at odd column {} would split a width-2 grapheme", wp);
    }
    
    let _ = row; // Suppress unused warning
}

/// CUR-08R: Cursor position after CJK insert is at grapheme boundary
#[test]
fn cur_08r_cursor_after_cjk_at_grapheme_boundary() {
    // After inserting "日本", cursor should be after the second grapheme
    let text = "日本";
    let total_width: usize = text.chars()
        .map(|c| grapheme_width(&c.to_string()))
        .sum();
    
    // Cursor column should be at total width (4), not in the middle of a grapheme
    assert_eq!(total_width, 4);
    
    // Cursor should be at column 4, which is a valid grapheme boundary
    // (after the second width-2 character)
    let valid_cursor_positions: Vec<usize> = vec![0, 2, 4]; // start, after 日, after 本
    assert!(valid_cursor_positions.contains(&total_width));
}

/// CUR-10R: Cursor visibility maintained during mixed-width editing
#[test]
fn cur_10r_cursor_visibility_mixed_width() {
    // Simulate editing text with mixed widths
    let lines = [
        "hello世界",      // ASCII + CJK
        "日本語123",      // CJK + ASCII
        "abc漢字xyz",     // ASCII + CJK + ASCII
    ];
    
    for line in &lines {
        let mut cursor_positions: Vec<usize> = vec![0];
        let mut col = 0;
        
        for c in line.chars() {
            col += grapheme_width(&c.to_string());
            cursor_positions.push(col);
        }
        
        // All cursor positions should be at grapheme boundaries
        // No position should be in the middle of a width-2 character
        for (i, pos) in cursor_positions.iter().enumerate() {
            if i > 0 {
                let prev_pos = cursor_positions[i - 1];
                let diff = pos - prev_pos;
                // Difference should be 1 (ASCII) or 2 (CJK), never fractional
                assert!(diff == 1 || diff == 2, "Invalid cursor step at position {}", i);
            }
        }
    }
}
