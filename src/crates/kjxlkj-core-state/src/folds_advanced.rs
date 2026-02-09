//! Advanced fold features.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum FoldMethod {
    Manual, Indent, Marker, Syntax, Expr, Diff,
}

impl Default for FoldMethod {
    fn default() -> Self {
        FoldMethod::Manual
    }
}

#[derive(Debug, Clone)]
pub struct FoldRegion {
    pub start: usize,
    pub end: usize,
    pub level: u8,
    pub closed: bool,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FoldState {
    pub regions: Vec<FoldRegion>,
    pub method: FoldMethod,
    pub fold_column: u8,
    pub fold_level: u8,
    pub fold_min_lines: usize,
    pub markers: (String, String),
}

impl FoldState {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            method: FoldMethod::Manual,
            fold_column: 0,
            fold_level: 20,
            fold_min_lines: 1,
            markers: ("{{{".to_string(), "}}}".to_string()),
        }
    }

    pub fn create_fold(&mut self, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let level = self.nesting_level(start) + 1;
        self.regions.push(FoldRegion {
            start,
            end,
            level,
            closed: true,
            text: None,
        });
        self.regions.sort_by_key(|r| r.start);
    }

    pub fn delete_fold(&mut self, line: usize) {
        self.regions.retain(|r| !(r.start <= line && line <= r.end));
    }

    pub fn toggle_fold(&mut self, line: usize) {
        for region in &mut self.regions {
            if region.start <= line && line <= region.end {
                region.closed = !region.closed;
                return;
            }
        }
    }

    pub fn open_all(&mut self) {
        for r in &mut self.regions {
            r.closed = false;
        }
    }

    pub fn close_all(&mut self) {
        for r in &mut self.regions {
            r.closed = true;
        }
    }

    pub fn is_folded(&self, line: usize) -> bool {
        self.regions
            .iter()
            .any(|r| r.closed && r.start < line && line <= r.end)
    }

    pub fn fold_at(&self, line: usize) -> Option<&FoldRegion> {
        self.regions.iter().find(|r| r.start == line)
    }

    pub fn nesting_level(&self, line: usize) -> u8 {
        self.regions
            .iter()
            .filter(|r| r.start <= line && line <= r.end)
            .count() as u8
    }

    pub fn detect_indent_folds(&mut self, lines: &[&str], tab_size: usize) {
        self.regions.clear();
        if lines.is_empty() {
            return;
        }
        let indents: Vec<usize> = lines
            .iter()
            .map(|l| {
                let trimmed = l.trim_start();
                if trimmed.is_empty() {
                    usize::MAX
                } else {
                    l.len() - trimmed.len()
                }
            })
            .collect();

        let mut stack: Vec<(usize, usize)> = Vec::new();
        for (i, &indent) in indents.iter().enumerate() {
            if indent == usize::MAX {
                continue;
            }
            let level = indent / tab_size.max(1);
            while let Some(&(_, sl)) = stack.last() {
                if sl >= level {
                    let (start, _) = stack.pop().unwrap();
                    if i > start + self.fold_min_lines {
                        self.regions.push(FoldRegion {
                            start,
                            end: i.saturating_sub(1),
                            level: sl as u8 + 1,
                            closed: false,
                            text: None,
                        });
                    }
                } else {
                    break;
                }
            }
            stack.push((i, level));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_fold() {
        let mut fs = FoldState::new();
        fs.create_fold(5, 10);
        assert!(fs.is_folded(7));
        assert!(!fs.is_folded(5)); // start line not hidden
        fs.toggle_fold(5);
        assert!(!fs.is_folded(7));
    }

    #[test]
    fn open_close_all() {
        let mut fs = FoldState::new();
        fs.create_fold(0, 5);
        fs.create_fold(10, 15);
        assert!(fs.is_folded(3));
        fs.open_all();
        assert!(!fs.is_folded(3));
        fs.close_all();
        assert!(fs.is_folded(3));
    }

    #[test]
    fn nesting_level() {
        let mut fs = FoldState::new();
        fs.create_fold(0, 20);
        fs.create_fold(5, 10);
        assert_eq!(fs.nesting_level(7), 2);
        assert_eq!(fs.nesting_level(15), 1);
    }
}
