/// Trend enum
#[derive(Debug, PartialEq, Clone)]
pub enum Trend {
    Bullish,
    Bearish,
}

use std::fmt;

impl fmt::Display for Trend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trend_str = match self {
            Trend::Bullish => "bullish",
            Trend::Bearish => "bearish",
        };
        write!(f, "{}", trend_str)
    }
}
