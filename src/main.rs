use std::fs;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    type_op: String,
    #[serde(deserialize_with = "de_u128_from_str")]
    account_id: u128,
    #[serde(deserialize_with = "de_f64_from_str")]
    amount: f64,
    #[serde(deserialize_with = "de_u128_from_str")]
    order_id: u128,
    pair: String,
    #[serde(deserialize_with = "de_f64_from_str")]
    limit_price: f64,
    side: String
}

fn de_u128_from_str<'de, D>(deserializer: D) -> Result<u128, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    u128::from_str(&s).map_err(de::Error::custom)
}

fn de_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

fn main() {
    let json_file_path = fs::canonicalize("./orders.json").unwrap();
    let file = fs::File::open(json_file_path).unwrap();
    let orders:Vec<Order> = serde_json::from_reader(file).expect("error while reading json");
    println!("{:#?}", orders);
}
