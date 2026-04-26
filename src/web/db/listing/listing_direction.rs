#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListDirection {
    Next,
    Prev,
}

impl ListDirection {
    pub fn resolve(value: Option<&str>, cursor: Option<&str>) -> Self {
        if cursor.is_none() {
            return Self::Next;
        }
        match value {
            Some("prev") => Self::Prev,
            _ => Self::Next,
        }
    }
}
