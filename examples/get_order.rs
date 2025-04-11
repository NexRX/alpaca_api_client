use alpaca_api_client::trading::{order::GetOrdersQuery, AccountType};

fn main() {
    get_all_orders_query();
    get_order_by_id();
}

fn get_all_orders_query() {
    GetOrdersQuery::builder()
        .url(AccountType::Paper)
        .status("closed")
        .nested(true)
        .side("buy")
        .build()
        .send()
        .unwrap();
}

fn get_order_by_id() {
    GetOrdersQuery::builder()
        .url(AccountType::Paper)
        .build()
        .get_by_id("3c9067a5-6553-40e5-ba56-b4fec94119dd", true)
        .unwrap();
}
