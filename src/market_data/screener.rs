use crate::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ActiveStock {
    pub symbol: String,
    pub volume: u64,
    pub trade_count: u64,
}

#[derive(Deserialize, Debug)]
pub struct ActiveStocksResponse {
    pub most_actives: Vec<ActiveStock>,
    pub last_updated: String,
}

pub struct ActiveStocksQuery<'a> {
    pub url: &'a str,
    by: Option<&'a str>,
    top: Option<i32>,
}
impl Default for ActiveStocksQuery<'_> {
    fn default() -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/screener/stocks/most-actives",
            by: None,
            top: None,
        }
    }
}

#[allow(clippy::result_large_err)]
impl<'a> ActiveStocksQuery<'a> {
    pub fn by(mut self, by: &'a str) -> Self {
        self.by = Some(by);
        self
    }

    pub fn top(mut self, top: i32) -> Self {
        self.top = Some(top);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();

        if let Some(by) = self.by {
            query.push_str(&format!("&by={by}"));
        }
        if let Some(top) = self.top {
            query.push_str(&format!("&top={top}"));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<Vec<ActiveStock>, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let stocks: ActiveStocksResponse = response.into_json()?;
        Ok(stocks.most_actives)
    }
}

#[derive(Deserialize, Debug)]
pub struct TopMover {
    pub symbol: String,
    pub percent_change: f64,
    pub change: f64,
    pub price: f64,
}

#[derive(Deserialize, Debug)]
pub struct TopMoverResponse {
    pub gainers: Vec<TopMover>,
    pub losers: Vec<TopMover>,
    pub market_type: String,
    pub last_updated: String,
}

pub struct TopMoversQuery {
    url: String,
    top: Option<i32>,
}

pub enum MarketType {
    Stocks,
    Crypto,
}

use std::fmt;

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let market_type = match self {
            MarketType::Stocks => "stocks",
            MarketType::Crypto => "crypto",
        };
        write!(f, "{}", market_type)
    }
}

#[allow(clippy::result_large_err)]
impl TopMoversQuery {
    pub fn new(market_type: MarketType) -> Self {
        Self {
            url: format!("https://data.alpaca.markets/v1beta1/screener/{market_type}/movers"),
            top: None,
        }
    }

    pub fn top(mut self, top: i32) -> Self {
        self.top = Some(top);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();
        if let Some(top) = self.top {
            query.push_str(&format!("&top={top}"));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<TopMoverResponse, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let movers: TopMoverResponse = response.into_json()?;
        Ok(movers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_stocks_query() {
        let query = ActiveStocksQuery::default()
            .by("volume")
            .top(5)
            .send()
            .unwrap();

        dbg!(&query);
        assert!(!query.is_empty());
    }

    #[test]
    fn test_top_movers_query() {
        let query = TopMoversQuery::new(MarketType::Stocks)
            .top(5)
            .send()
            .unwrap();
        dbg!(&query);
        assert!(!query.gainers.is_empty());
    }
}
