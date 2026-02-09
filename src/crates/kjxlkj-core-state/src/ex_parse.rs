//! Range parsing for ex commands.
//! Ranges select line spans. The result is a (start_line, end_line) pair (0-indexed).

/// Parsed range for an ex command.
///
/// Both `start` and `end` are 0-indexed inclusive line numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExRange {
    /// Start line (0-indexed inclusive).
    pub start: usize,
    /// End line (0-indexed inclusive).
    pub end: usize,
}

impl ExRange {
    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }

    pub fn all(line_count: usize) -> Self {
        Self {
            start: 0,
            end: line_count.saturating_sub(1),
        }
    }

    pub fn clamp(self, line_count: usize) -> Self {
        let max = line_count.saturating_sub(1);
        Self {
            start: self.start.min(max),
            end: self.end.min(max),
        }
    }

    pub fn line_count(&self) -> usize {
        if self.end >= self.start {
            self.end - self.start + 1
        } else {
            0
        }
    }
}

/// Parse a range prefix from an ex command string.
/// Returns (optional range, remaining command string).
pub fn parse_range(
    input: &str,
    current_line: usize,
    total_lines: usize,
) -> (Option<ExRange>, &str) {
    let input = input.trim_start();
    if input.is_empty() {
        return (None, input);
    }

    // Check for % (whole file)
    if let Some(rest) = input.strip_prefix('%') {
        return (Some(ExRange::all(total_lines)), rest);
    }

    // Try to parse address,address
    let (addr1, rest1) = parse_address(input, current_line, total_lines);
    if let Some(start) = addr1 {
        let rest1 = rest1.trim_start();
        if let Some(rest2) = rest1.strip_prefix(',') {
            let rest2 = rest2.trim_start();
            let (addr2, rest3) = parse_address(rest2, current_line, total_lines);
            if let Some(end) = addr2 {
                return (Some(ExRange { start, end }.clamp(total_lines)), rest3);
            }
            // Comma with no second address: use current line as end
            return (
                Some(
                    ExRange {
                        start,
                        end: current_line,
                    }
                    .clamp(total_lines),
                ),
                rest2,
            );
        }
        // Single address = single line
        return (Some(ExRange::single(start).clamp(total_lines)), rest1);
    }

    (None, input)
}

/// Parse a single line address. Returns (line_number_0_indexed, remaining_input).
fn parse_address(input: &str, current_line: usize, total_lines: usize) -> (Option<usize>, &str) {
    let input = input.trim_start();
    if input.is_empty() {
        return (None, input);
    }

    let first = input.as_bytes()[0];

    // Current line
    if first == b'.' {
        let (offset, rest) = parse_offset(&input[1..]);
        let line = (current_line as isize + offset).max(0) as usize;
        return (Some(line), rest);
    }

    // Last line
    if first == b'$' {
        let (offset, rest) = parse_offset(&input[1..]);
        let last = total_lines.saturating_sub(1) as isize;
        let line = (last + offset).max(0) as usize;
        return (Some(line), rest);
    }

    // Line number (1-indexed in vim, we convert to 0-indexed)
    if first.is_ascii_digit() {
        let end = input
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input.len());
        if let Ok(n) = input[..end].parse::<usize>() {
            let line = n.saturating_sub(1); // Convert 1-indexed to 0-indexed
            let (offset, rest) = parse_offset(&input[end..]);
            let line = (line as isize + offset).max(0) as usize;
            return (Some(line), rest);
        }
    }

    (None, input)
}

/// Parse an optional +N or -N offset after an address.
fn parse_offset(input: &str) -> (isize, &str) {
    let input = input.trim_start();
    if input.is_empty() {
        return (0, input);
    }

    let first = input.as_bytes()[0];
    if first == b'+' || first == b'-' {
        let sign: isize = if first == b'+' { 1 } else { -1 };
        let rest = &input[1..];
        let end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        if end == 0 {
            return (sign, rest);
        }
        if let Ok(n) = rest[..end].parse::<isize>() {
            return (sign * n, &rest[end..]);
        }
    }

    (0, input)
}
