//! Popular-note window selection

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopularWindow {
    Days1,
    Days7,
    Days30,
    Days90,
    All,
}

impl PopularWindow {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "1d" => Some(Self::Days1),
            "7d" => Some(Self::Days7),
            "30d" => Some(Self::Days30),
            "90d" => Some(Self::Days90),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Days1 => "1d",
            Self::Days7 => "7d",
            Self::Days30 => "30d",
            Self::Days90 => "90d",
            Self::All => "all",
        }
    }

    pub fn button_label(self) -> &'static str {
        match self {
            Self::Days1 => "1d",
            Self::Days7 => "7d",
            Self::Days30 => "30d",
            Self::Days90 => "90d",
            Self::All => "All time",
        }
    }

    pub fn days(self) -> Option<i32> {
        match self {
            Self::Days1 => Some(1),
            Self::Days7 => Some(7),
            Self::Days30 => Some(30),
            Self::Days90 => Some(90),
            Self::All => None,
        }
    }

    pub fn metric_label(self) -> &'static str {
        match self {
            Self::Days1 => "1d views",
            Self::Days7 => "7d views",
            Self::Days30 => "30d views",
            Self::Days90 => "90d views",
            Self::All => "Views",
        }
    }
}
