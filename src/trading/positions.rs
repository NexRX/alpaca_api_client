use std::fmt::Display;

use super::{order::Order, AccountType};
use crate::request;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type AllPositions = Vec<Position>;
pub type AllClosedPositions = Vec<ClosedPosition>;

#[derive(Deserialize, Debug)]
pub struct ClosedPosition {
    pub symbol: String,
    pub status: i32,
    pub body: Order,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Position {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub avg_entry_price: Decimal,
    pub qty: Decimal,
    pub qty_available: Decimal,
    pub side: PositionSide,
    pub market_value: Decimal,
    pub cost_basis: Decimal,
    pub unrealized_pl: Decimal,
    pub unrealized_plpc: Decimal,
    pub unrealized_intraday_pl: Decimal,
    pub unrealized_intraday_plpc: Decimal,
    pub current_price: Decimal,
    pub lastday_price: Decimal,
    pub change_today: Decimal,
}

pub struct PositionsQuery<'a> {
    url: &'a str,
}

#[allow(clippy::result_large_err)]
impl<'a> PositionsQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/positions",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/positions",
            },
        }
    }

    pub fn get_all_open_positions(&self) -> Result<AllPositions, ureq::Error> {
        let response = request("GET", self.url).call()?;
        let positions = response.into_json()?;

        Ok(positions)
    }

    pub fn get_position_by_symbol(&self, symbol: &'a str) -> Result<Position, ureq::Error> {
        let route = format!("{}/{}", self.url, symbol);
        let response = request("GET", &route).call()?;
        let position = response.into_json()?;

        Ok(position)
    }

    pub fn get_position_by_id(&self, id: &'a str) -> Result<Position, ureq::Error> {
        let route = format!("{}/{}", self.url, id);
        let response = request("GET", &route).call()?;
        let position = response.into_json()?;

        Ok(position)
    }

    pub fn close_all_positions(
        &self,
        cancel_orders: bool,
    ) -> Result<AllClosedPositions, ureq::Error> {
        let query = format!("?cancel_orders={}", cancel_orders);
        let route = format!("{}{}", self.url, query);
        let response: ureq::Response = request("DELETE", &route).call()?;

        if response.status() != 200 && response.status() != 207 {
            return Err(ureq::Error::from(response));
        }

        Ok(response.into_json()?)
    }

    pub fn close_position_by_id_or_symbol(
        &self,
        id_or_symbol: &'a str,
        qty: Option<f32>,
        percentage: Option<f32>,
    ) -> Result<Order, ureq::Error> {
        let url = format!("{}/{}", self.url, id_or_symbol);
        let mut query = String::new();
        if let Some(qty) = qty {
            query.push_str(&format!("&qty={}", qty));
        }
        if let Some(percentage) = percentage {
            query.push_str(&format!("&percentage={}", percentage));
        }
        let route = format!("{}?{}", url, query);
        let response = request("DELETE", &route).call()?;
        let position = response.into_json()?;

        Ok(position)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PositionSide {
    Long,
    Short,
}

impl Display for PositionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Long => "long",
                Self::Short => "short",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_positions_query() {
        let res = PositionsQuery::new(AccountType::Paper)
            .get_all_open_positions()
            .unwrap();
        dbg!(&res);
        assert!(!res.is_empty());
    }

    #[test]
    fn test_get_position_by_symbol() {
        let res = PositionsQuery::new(AccountType::Paper)
            .get_position_by_symbol("AAPL")
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "AAPL");
    }

    #[test]
    fn test_get_position_by_id() {
        let res = PositionsQuery::new(AccountType::Paper)
            .get_position_by_id("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415")
            .unwrap();
        dbg!(&res);
        assert!(res.asset_id == "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415");
    }

    #[test]
    fn test_close_all_positions() {
        let res = PositionsQuery::new(AccountType::Paper)
            .close_all_positions(true)
            .unwrap();
        dbg!(&res);
        assert!(!res.is_empty());
    }

    #[test]
    fn test_close_position_by_id_or_symbol() {
        let res = PositionsQuery::new(AccountType::Paper)
            .close_position_by_id_or_symbol("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415", Some(1.0), None)
            .unwrap();
        dbg!(&res);
        assert!(res.asset_id == Some("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415".to_string()));
    }
}
