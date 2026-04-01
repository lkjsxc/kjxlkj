//! Popular-note window selection

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopularWindow {
    Days7,
    Days30,
    Days90,
}

impl PopularWindow {
    pub fn resolve(value: Option<&str>) -> Self {
        match value {
            Some("7d") => Self::Days7,
            Some("90d") => Self::Days90,
            _ => Self::Days30,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Days7 => "7d",
            Self::Days30 => "30d",
            Self::Days90 => "90d",
        }
    }

    pub fn days(self) -> i32 {
        match self {
            Self::Days7 => 7,
            Self::Days30 => 30,
            Self::Days90 => 90,
        }
    }

    pub fn metric_label(self) -> &'static str {
        match self {
            Self::Days7 => "7d views",
            Self::Days30 => "30d views",
            Self::Days90 => "90d views",
        }
    }
}
