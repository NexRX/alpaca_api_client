use super::AccountType;
use crate::{request, TimeFrame};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<f64>,
    pub profit_loss: Vec<f64>,
    pub profit_loss_pct: Vec<f64>,
    pub base_value: f64,
    #[builder(setter(strip_option))]
    pub base_value_asof: Option<String>,
    pub timeframe: String,
    #[builder(setter(strip_option))]
    pub cashflow: Option<String>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct PortfolioHistoryQuery<'a> {
    #[builder(!default, setter(!strip_option, transform = |account_type: AccountType| match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/orders",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
    }))]
    pub url: &'a str,
    pub period: Option<&'a str>,
    pub timeframe: Option<TimeFrame>,
    pub intraday_reporting: Option<&'a str>,
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
    pub pnl_reset: Option<&'a str>,
    pub date_end: Option<&'a str>,
    pub extended_hours: Option<&'a str>,
    pub cashflow_types: Option<Vec<&'a str>>,
}

impl PortfolioHistoryQuery<'_> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/account/portfolio/history",
                AccountType::Paper => {
                    "https://paper-api.alpaca.markets/v2/account/portfolio/history"
                }
            },
            period: None,
            timeframe: None,
            intraday_reporting: None,
            start: None,
            end: None,
            pnl_reset: None,
            date_end: None,
            extended_hours: None,
            cashflow_types: None,
        }
    }

    fn build(self) -> String {
        let mut query = String::new();
        if let Some(period) = self.period {
            query.push_str(&format!("&period={}", period));
        }
        if let Some(timeframe) = self.timeframe {
            query.push_str(&format!("&timeframe={}", timeframe));
        }
        if let Some(intraday_reporting) = self.intraday_reporting {
            query.push_str(&format!("&intraday_reporting={}", intraday_reporting));
        }
        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }
        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }
        if let Some(pnl_reset) = self.pnl_reset {
            query.push_str(&format!("&pnl_reset={}", pnl_reset));
        }
        if let Some(date_end) = self.date_end {
            query.push_str(&format!("&date_end={}", date_end));
        }
        if let Some(extended_hours) = self.extended_hours {
            query.push_str(&format!("&extended_hours={}", extended_hours));
        }
        if let Some(cashflow_types) = self.cashflow_types {
            query.push_str(&format!("&cashflow_types={}", cashflow_types.join(",")));
        }

        format!("{}?{}", self.url, query)
    }

    #[allow(clippy::result_large_err)]
    pub fn send(self) -> Result<PortfolioHistory, ureq::Error> {
        let url = self.build();
        let response = request("GET", &url).call()?;
        Ok(response.into_json()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_portfolio_history() {
        let query = PortfolioHistoryQuery::new(AccountType::Paper)
            .send()
            .unwrap();

        dbg!(&query);
        assert!(!query.timestamp.is_empty());
    }
}
