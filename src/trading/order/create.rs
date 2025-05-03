#![allow(clippy::result_large_err)]
use core::fmt;

use super::{Order, OrderSide};
use crate::{request, trading::AccountType};
use rust_decimal::Decimal;
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
    #[builder(!default, setter(!strip_option))]
    qty: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notional: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_price: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_percent: Option<Decimal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_class: Option<OrderClass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    take_profit: Option<TakeProfit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_loss: Option<StopLoss>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    #[default]
    Market,
    Limit,
    Stop,
    StopLimit,
    /// Not an option for Options trading I believe
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum TimeInForce {
    #[default]
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "gtc")]
    GoodTilCanceled,
    #[serde(rename = "opg")]
    OpeningOrder,
    #[serde(rename = "cls")]
    ClosingOrder,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "fok")]
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
    #[serde(rename = "")]
    Simple,
    #[serde(rename = "bracket")]
    Bracket,
    #[serde(rename = "oco")]
    OneCancelsOther,
    #[serde(rename = "oto")]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TakeProfit {
    pub limit_price: Decimal,
}

impl TakeProfit {
    pub fn new(limit_price: Decimal) -> Self {
        Self { limit_price }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StopLoss {
    pub stop_price: Decimal,
    pub limit_price: Decimal,
}

impl StopLoss {
    pub fn new(stop_price: Decimal, limit_price: Decimal) -> Self {
        Self {
            stop_price,
            limit_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::panic_error_response;

    use super::*;

    #[test]
    fn test_create_market_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Buy)
            .r#type(OrderType::Market)
            .qty(dec!(1))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .limit_price(dec!(100))
            .qty(dec!(1))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .stop_price(dec!(100))
            .qty(dec!(1))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .stop_price(dec!(100))
            .limit_price(dec!(200))
            .qty(dec!(1))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .qty(dec!(1))
            .trail_percent(dec!(10))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .qty(dec!(1))
            .order_class(OrderClass::Bracket)
            .take_profit(TakeProfit::new(dec!(300)))
            .stop_loss(StopLoss::new(dec!(200), dec!(199)))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_oco_order() {
        let order = CreateOrderQuery::builder()
            .symbol("AAPL")
            .side(OrderSide::Sell)
            .r#type(OrderType::Limit)
            .time_in_force(TimeInForce::GoodTilCanceled)
            .qty(dec!(1))
            .order_class(OrderClass::OneCancelsOther)
            .take_profit(TakeProfit::new(dec!(201)))
            .stop_loss(StopLoss::new(dec!(200), dec!(199)))
            .build()
            .send(AccountType::Paper)
            .map_err(|err| panic_error_response!(err))
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
            .qty(dec!(1))
            .order_class(OrderClass::OneTriggersOther)
            .stop_loss(StopLoss::new(dec!(190), dec!(189)))
            .build()
            .send(AccountType::Paper)
            // .map_err(|err| panic_error_response!(err))
            .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }
}
