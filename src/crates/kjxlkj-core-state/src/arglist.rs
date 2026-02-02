//! Argument list implementation.
//!
//! Manages the list of files passed on the command line.

use std::path::PathBuf;

/// The argument list.
#[derive(Debug, Clone, Default)]
pub struct ArgList {
    /// Files in the argument list.
    files: Vec<PathBuf>,
    /// Current position.
    current: usize,
}

impl ArgList {
    /// Creates a new empty argument list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an argument list from files.
    pub fn from_files(files: Vec<PathBuf>) -> Self {
        Self { files, current: 0 }
    }

    /// Adds a file to the argument list.
    pub fn add(&mut self, path: PathBuf) {
        self.files.push(path);
    }

    /// Removes a file from the argument list.
    pub fn remove(&mut self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
            if self.current >= self.files.len() && self.current > 0 {
                self.current = self.files.len() - 1;
            }
        }
    }

    /// Returns the number of files.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Returns the current file.
    pub fn current_file(&self) -> Option<&PathBuf> {
        self.files.get(self.current)
    }

    /// Returns the current index (0-based).
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Moves to the next file.
    pub fn next(&mut self) -> Option<&PathBuf> {
        if self.current + 1 < self.files.len() {
            self.current += 1;
        }
        self.current_file()
    }

    /// Moves to the previous file.
    pub fn prev(&mut self) -> Option<&PathBuf> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.current_file()
    }

    /// Moves to the first file.
    pub fn first(&mut self) -> Option<&PathBuf> {
        self.current = 0;
        self.current_file()
    }

    /// Moves to the last file.
    pub fn last(&mut self) -> Option<&PathBuf> {
        if !self.files.is_empty() {
            self.current = self.files.len() - 1;
        }
        self.current_file()
    }

    /// Moves to a specific index (0-based).
    pub fn goto(&mut self, index: usize) -> Option<&PathBuf> {
        if index < self.files.len() {
            self.current = index;
        }
        self.current_file()
    }

    /// Returns all files.
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    /// Clears the argument list.
    pub fn clear(&mut self) {
        self.files.clear();
        self.current = 0;
    }

    /// Returns formatted string like "[1 of 3]".
    pub fn status(&self) -> String {
        if self.files.is_empty() {
            String::new()
        } else {
            format!("[{} of {}]", self.current + 1, self.files.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_files() -> Vec<PathBuf> {
        vec![
            PathBuf::from("a.rs"),
            PathBuf::from("b.rs"),
            PathBuf::from("c.rs"),
        ]
    }

    #[test]
    fn test_arglist_from_files() {
        let list = ArgList::from_files(sample_files());
        assert_eq!(list.len(), 3);
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_arglist_navigation() {
        let mut list = ArgList::from_files(sample_files());

        list.next();
        assert_eq!(list.current_index(), 1);
        list.prev();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_arglist_first_last() {
        let mut list = ArgList::from_files(sample_files());

        list.last();
        assert_eq!(list.current_index(), 2);
        list.first();
        assert_eq!(list.current_index(), 0);
    }

    #[test]
    fn test_arglist_remove() {
        let mut list = ArgList::from_files(sample_files());
        list.goto(2);

        list.remove(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.current_index(), 1);
    }

    #[test]
    fn test_arglist_status() {
        let mut list = ArgList::from_files(sample_files());
        assert_eq!(list.status(), "[1 of 3]");

        list.next();
        assert_eq!(list.status(), "[2 of 3]");
    }

    #[test]
    fn test_arglist_add() {
        let mut list = ArgList::new();
        list.add(PathBuf::from("new.rs"));
        assert_eq!(list.len(), 1);
    }
}
