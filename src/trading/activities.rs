#![allow(clippy::result_large_err)]
use super::{order::OrderSide, AccountType};
use crate::request;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Deserialize, Debug)]
pub struct TradeActivity {
    pub activity_type: Option<String>,
    pub id: String,
    pub cum_qty: Option<Decimal>,
    pub leaves_qty: Option<Decimal>,
    pub price: Option<Decimal>,
    pub qty: Option<Decimal>,
    pub side: Option<OrderSide>,
    pub symbol: Option<String>,
    pub transaction_time: Option<DateTime<Utc>>,
    pub order_id: Option<String>,
    pub r#type: Option<String>,
    pub order_status: Option<String>,
    pub date: Option<NaiveDate>,
    pub net_amount: Option<Decimal>,
    pub per_share_amount: Option<Decimal>,
    pub group_id: Option<String>,
    pub status: Option<String>,
}

pub type TradeActivities = Vec<TradeActivity>;

#[derive(Serialize, Deserialize, Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct ActivitiesQuery<'a> {
    #[builder(!default, setter(!strip_option, transform = |account_type: AccountType| match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/orders",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
    }))]
    url: &'a str,
    activity_types: Option<Vec<&'a str>>,
    category: Option<&'a str>,
    date: Option<&'a str>,
    until: Option<&'a str>,
    after: Option<&'a str>,
    direction: Option<&'a str>,
    page_size: Option<usize>,
    limit: Option<usize>,
}

impl ActivitiesQuery<'_> {
    fn build(&self) -> String {
        let mut query = String::new();
        if let Some(activity_types) = &self.activity_types {
            query.push_str(&format!("&activity_types={}", activity_types.join(",")));
        }
        if let Some(category) = self.category {
            query.push_str(&format!("&category={}", category));
        }
        if let Some(date) = self.date {
            query.push_str(&format!("&date={}", date));
        }
        if let Some(until) = self.until {
            query.push_str(&format!("&until={}", until));
        }
        if let Some(after) = self.after {
            query.push_str(&format!("&after={}", after));
        }
        if let Some(direction) = self.direction {
            query.push_str(&format!("&direction={}", direction));
        }
        if let Some(page_size) = self.page_size {
            query.push_str(&format!("&page_size={}", page_size));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<TradeActivities, ureq::Error> {
        let route = self.build();
        let mut trade_activities: TradeActivities = Vec::new();
        let mut page_token = None;

        let mut i = 0;
        let data_limit = self.limit.unwrap_or(1000);

        let expected_page_size = self.page_size.unwrap_or(100);

        loop {
            if i >= data_limit {
                break;
            }

            // If a token exists, append to address
            let temp_address = match page_token {
                Some(ref token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };

            let response = request("GET", &temp_address).call()?;
            let response: TradeActivities = response.into_json()?;
            let returned_page_size = response.len();

            for (index, item) in response.into_iter().enumerate() {
                i += 1;
                if index == returned_page_size && returned_page_size == expected_page_size {
                    page_token = Some(item.id.clone());
                }
                trade_activities.push(item);
            }

            if page_token.is_none() {
                break;
            }
        }

        Ok(trade_activities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_activities_query() {
        let query = ActivitiesQuery::builder()
            .url(AccountType::Paper)
            .build()
            .send()
            .unwrap();

        dbg!(&query);
        assert!(!query.is_empty());
    }
}
