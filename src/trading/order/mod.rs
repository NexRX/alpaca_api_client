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
use serde::{Deserialize, Serialize};

pub type AllOrders = Vec<Order>;

/// API object for an Order
#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub id: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: Option<String>,
    pub symbol: String,
    pub asset_class: Option<String>,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: Option<String>,
    pub filled_avg_price: Option<String>,
    pub order_class: Option<String>,
    pub order_type: String,
    pub r#type: String,
    pub side: OrderSide,
    pub time_in_force: Option<String>,
    pub limit_price: Option<Decimal>,
    pub stop_price: Option<Decimal>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<Self>>,
    pub trail_percent: Option<Decimal>,
    pub trail_price: Option<Decimal>,
    pub hwm: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
