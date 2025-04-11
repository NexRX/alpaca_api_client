#![allow(clippy::result_large_err)]
use core::fmt;

use super::{Order, OrderSide};
use crate::{request, trading::AccountType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct CreateOrderQuery<'a> {
    #[builder(!default, setter(!strip_option))]
    symbol: &'a str,
    #[builder(!default, setter(!strip_option))]
    side: OrderSide,
    #[builder(!default, setter(!strip_option))]
    r#type: OrderType,
    #[builder(default=TimeInForce::Day, setter(!strip_option))]
    time_in_force: TimeInForce,
    #[builder(default=false, setter(!strip_option))]
    extend_hours: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notional: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_percent: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_class: Option<OrderClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    take_profit: Option<TakeProfit<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_loss: Option<StopLoss<'a>>,
}

impl CreateOrderQuery<'_> {
    pub fn send(self, account_type: AccountType) -> Result<Order, ureq::Error> {
        let url = match account_type {
            AccountType::Live => "https://api.alpaca.markets/v2/orders",
            AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
        };

        let response = request("POST", url)
            .set("Content-Type", "application/json")
            .send_json(&self)?;

        let order = response.into_json()?;
        Ok(order)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderType::Market => "market",
            OrderType::Limit => "limit",
            OrderType::Stop => "stop",
            OrderType::StopLimit => "stop_limit",
            OrderType::TrailingStop => "trailing_stop",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TimeInForce {
    Day,
    GoodTilCanceled,
    OpeningOrder,
    ClosingOrder,
    ImmediateOrCancel,
    FillOrKill,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TimeInForce::Day => "day".to_string(),
            TimeInForce::GoodTilCanceled => "gtc".to_string(),
            TimeInForce::OpeningOrder => "opg".to_string(),
            TimeInForce::ClosingOrder => "cls".to_string(),
            TimeInForce::ImmediateOrCancel => "ioc".to_string(),
            TimeInForce::FillOrKill => "fok".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderClass {
    Simple,
    Bracket,
    OneCancelsOther,
    OneTriggersOther,
}

impl fmt::Display for OrderClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderClass::Simple => "".to_string(),
            OrderClass::Bracket => "bracket".to_string(),
            OrderClass::OneCancelsOther => "oco".to_string(),
            OrderClass::OneTriggersOther => "oto".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakeProfit<'a> {
    pub limit_price: &'a str,
}

impl<'a> TakeProfit<'a> {
    pub fn new(limit_price: &'a str) -> Self {
        Self { limit_price }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopLoss<'a> {
    pub stop_price: &'a str,
    pub limit_price: &'a str,
}

impl<'a> StopLoss<'a> {
    pub fn new(stop_price: &'a str, limit_price: &'a str) -> Self {
        Self {
            stop_price,
            limit_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_market_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Market)
            .qty("1")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_limit_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Limit)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .limit_price("100")
            .qty("1")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_stop_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Stop)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .stop_price("100")
            .qty("1")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_stop_limit_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::StopLimit)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .stop_price("100")
            .limit_price("200")
            .qty("1")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_trailing_stop_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::TrailingStop)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .qty("1")
            .trail_percent("10")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_bracket_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Market)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .qty("1")
            .order_class(OrderClass::Bracket)
            .take_profit(TakeProfit::new("300"))
            .stop_loss(StopLoss::new("200", "199"))
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_oco_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Limit)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .qty("1")
            .order_class(OrderClass::OneCancelsOther)
            .take_profit(TakeProfit::new("199"))
            .stop_loss(StopLoss::new("200", "201"))
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_oto_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Market)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .qty("1")
            .order_class(OrderClass::OneTriggersOther)
            .stop_loss(StopLoss::new("200", "189"))
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }
}
