mod create;

use chrono::{DateTime, Utc};
pub use create::*;

mod delete;
pub use delete::*;

mod get;
pub use get::*;

mod replace;
pub use replace::*;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub type AllOrders = Vec<Order>;

/// API object for an Order
#[derive(Deserialize, Serialize, Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct Order {
    #[builder(setter(skip))]
    pub id: String,
    pub client_order_id: Option<String>,
    #[builder(default=Utc::now(), setter(skip))]
    pub created_at: DateTime<Utc>,
    #[builder(setter(skip))]
    pub updated_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub submitted_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub filled_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub expired_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub canceled_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub failed_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub replaced_at: Option<DateTime<Utc>>,
    #[builder(setter(skip))]
    pub replaced_by: Option<String>,
    #[builder(setter(skip))]
    pub replaces: Option<String>,
    pub asset_id: Option<String>,
    #[builder(!default, setter(!strip_option))]
    pub symbol: String,
    #[builder(setter(skip))]
    pub asset_class: Option<String>,
    pub notional: Option<String>,
    #[builder(setter(!strip_option))]
    pub qty: Decimal,
    #[builder(default=dec!(0), setter(skip))]
    pub filled_qty: Decimal,
    #[builder(default=dec!(0), setter(skip))]
    pub filled_avg_price: Decimal,
    pub order_class: Option<String>,
    // #[builder(!default, setter(!strip_option))]
    pub order_type: Option<OrderType>,
    #[builder(!default, setter(!strip_option))]
    pub r#type: OrderType,
    #[builder(!default, setter(!strip_option))]
    pub side: OrderSide,
    pub time_in_force: Option<TimeInForce>,
    pub limit_price: Option<Decimal>,
    pub stop_price: Option<Decimal>,
    #[builder(setter(skip))]
    pub status: String,
    #[builder(setter(skip))]
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub trail_percent: Option<Decimal>,
    pub trail_price: Option<Decimal>,
    pub hwm: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

use std::fmt;

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let side = match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };
        write!(f, "{}", side)
    }
}
