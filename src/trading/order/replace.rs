#![allow(clippy::result_large_err)]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Order;
use crate::{request, trading::AccountType};

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct ReplaceOrderQuery<'a> {
    #[serde(skip_serializing)]
    #[builder(!default, setter(!strip_option))]
    pub order_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<&'a str>,
}

impl ReplaceOrderQuery<'_> {
    pub fn send(self, account_type: AccountType) -> Result<Order, ureq::Error> {
        let url = match account_type {
            AccountType::Live => format!("https://api.alpaca.markets/v2/orders/{}", self.order_id),
            AccountType::Paper => format!(
                "https://paper-api.alpaca.markets/v2/orders/{}",
                self.order_id
            ),
        };
        let response = request("PATCH", &url)
            .set("Content-Type", "application/json")
            .send_json(&self)?;

        let order = response.into_json()?;
        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_order() {
        //! Will fail if order is not found or correct type
        let res = ReplaceOrderQuery::builder()
            .order_id("615bbc4d-966c-470e-bc37-fd0ae3218927")
            .qty("2")
            .build()
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&res);
        assert!(res.id == "615bbc4d-966c-470e-bc37-fd0ae3218927");
    }
}
