//! Popular-note window selection

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PopularWindow {
    Days7,
    Days30,
    Days90,
}

impl PopularWindow {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "7d" => Some(Self::Days7),
            "30d" => Some(Self::Days30),
            "90d" => Some(Self::Days90),
            _ => None,
        }
    }

    pub fn resolve(value: Option<&str>) -> Self {
        value.and_then(Self::parse).unwrap_or(Self::Days30)
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
