use alpaca_api_client::trading::{
    order::{
        CreateOrderQuery, OrderClass, OrderSide, OrderType, StopLoss, TakeProfit, TimeInForce,
    },
    AccountType,
};
use rust_decimal_macros::dec;

fn main() {
    create_market_order();
    create_limit_order();
    create_stop_order();
    create_stop_limit_order();
    create_trailing_stop_order();
    create_bracket_order();
    create_oto_order();
    create_oco_order();
}

fn create_market_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::Market)
        .time_in_force(TimeInForce::Day)
        .qty(dec!(1))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}
fn create_limit_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::Limit)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .limit_price(dec!(100))
        .qty(dec!(1))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}

fn create_stop_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::Stop)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .stop_price(dec!(100))
        .qty(dec!(1))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}

fn create_stop_limit_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::StopLimit)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .stop_price(dec!(100))
        .limit_price(dec!(200))
        .qty(dec!(1))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}

fn create_trailing_stop_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::TrailingStop)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .qty(dec!(1))
        .trail_percent(dec!(10))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}

fn create_bracket_order() {
    CreateOrderQuery::builder()
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
        .unwrap();
}

fn create_oco_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::Limit)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .qty(dec!(1))
        .order_class(OrderClass::OneCancelsOther)
        .take_profit(TakeProfit::new(dec!(199)))
        .stop_loss(StopLoss::new(dec!(200), dec!(201)))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}

fn create_oto_order() {
    CreateOrderQuery::builder()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .r#type(OrderType::Market)
        .time_in_force(TimeInForce::GoodTilCanceled)
        .qty(dec!(1))
        .order_class(OrderClass::OneTriggersOther)
        .stop_loss(StopLoss::new(dec!(200), dec!(189)))
        .build()
        .send(AccountType::Paper)
        .unwrap();
}
