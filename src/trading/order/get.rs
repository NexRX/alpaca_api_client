#![allow(clippy::result_large_err)]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{AllOrders, Order};
use crate::{request, trading::AccountType};

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct GetOrdersQuery<'a> {
    #[builder(!default, setter(!strip_option, transform = |account_type: AccountType| match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/orders",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
    }))]
    pub url: &'a str,
    pub status: Option<&'a str>,
    pub limit: Option<usize>,
    pub after: Option<&'a str>,
    pub until: Option<&'a str>,
    pub direction: Option<&'a str>,
    pub nested: Option<bool>,
    pub symbols: Option<Vec<&'a str>>,
    pub side: Option<&'a str>,
}

impl<'a> GetOrdersQuery<'a> {
    fn build(self) -> String {
        let mut query = String::new();
        if let Some(status) = self.status {
            query.push_str(&format!("&status={}", status));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        if let Some(after) = self.after {
            query.push_str(&format!("&after={}", after));
        }
        if let Some(until) = self.until {
            query.push_str(&format!("&until={}", until));
        }
        if let Some(direction) = self.direction {
            query.push_str(&format!("&direction={}", direction));
        }
        if let Some(nested) = self.nested {
            query.push_str(&format!("&nested={}", nested));
        }
        if let Some(symbols) = self.symbols {
            query.push_str(&format!("&symbols={}", symbols.join(",")));
        }
        if let Some(side) = self.side {
            query.push_str(&format!("&side={}", side));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn get_by_id(self, id: &'a str, nested: bool) -> Result<Order, ureq::Error> {
        let route = format!("{}/{}?&nested={}", self.url, id, nested);
        let response = request("GET", &route).call()?;
        let orders: Order = response.into_json()?;
        Ok(orders)
    }

    pub fn send(self) -> Result<AllOrders, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let orders: AllOrders = response.into_json()?;
        Ok(orders)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_orders_query() {
        let res = GetOrdersQuery::builder()
            .url(AccountType::Paper)
            .status("closed")
            .build()
            .send()
            .unwrap();

        dbg!(&res);
        assert!(!res.is_empty());
    }

    #[test]
    fn test_get_order_by_id() {
        let res = GetOrdersQuery::builder()
            .url(AccountType::Paper)
            .build()
            .get_by_id("3c9067a5-6553-40e5-ba56-b4fec94119dd", true);

        dbg!(&res);
        assert!(matches!(res, Err(ureq::Error::Status(404, _))))
    }
}
