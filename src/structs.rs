use serde::{Deserialize, Serialize};

use crate::deserializers::*;

#[derive(Deserialize, Serialize)]
pub struct Order {
    pub type_op: String,
    #[serde(deserialize_with = "de_u128_from_str")]
    pub account_id: u128,
    #[serde(deserialize_with = "de_f64_from_str")]
    pub amount: f64,
    #[serde(deserialize_with = "de_u128_from_str")]
    pub order_id: u128,
    pub pair: String,
    #[serde(deserialize_with = "de_f64_from_str")]
    pub limit_price: f64,
    pub side: String
}

#[derive(Serialize)]
pub struct Orderbook {
    pub sells: Vec<Order>,
    pub buys: Vec<Order>
}

#[derive(Serialize)]
pub struct Trade {
    pub sell: Order,
    pub buy: Order
}
