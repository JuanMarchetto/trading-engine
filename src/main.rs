use std::fs;
use std::io::Write;
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

#[derive(Debug, Serialize)]
struct Orderbook {
    sells: Vec<Order>,
    buys: Vec<Order>
}

#[derive(Debug, Serialize)]
struct Trade {
    sell: Order,
    buy: Order
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
    // Read the file
    let json_file_path = fs::canonicalize("./orders.json").unwrap();
    let file = fs::File::open(json_file_path).unwrap();
    // Deserialize the file
    let orders:Vec<Order> = serde_json::from_reader(file).expect("error while reading json");
    // Initialize an empty orderbook and a the trades
    let mut orderbook = Orderbook { sells: vec![], buys: vec![] };
    let mut trades = vec![];
    // Handle the orders
    for order in orders {
        match order.side.as_str() {
            "SELL" => {
                // IF there is a buy order with the same price or higher,
                // then we have a trade
                // else we add the order to the sells side of the orderbook
                let mut found_buy = false;
                for buy in orderbook.buys.iter() {
                    if buy.limit_price >= order.limit_price {
                        found_buy = true;
                        break;
                    }
                }
                if found_buy {
                    let trade = Trade { sell: order, buy: orderbook.buys.pop().unwrap() };
                    trades.push(trade);
                } else {
                    orderbook.sells.push(order);
                }
            },
            "BUY" => {
                // IF there is a sell order with the same price or lower,
                // then we have a trade
                // else we add the order to the buys to the buys side of the orderbook
                let mut found_sell = false;
                for sell in orderbook.sells.iter() {
                    if sell.limit_price <= order.limit_price {
                        found_sell = true;
                        break;
                    }
                }
                if found_sell {
                    let trade = Trade { sell: orderbook.sells.pop().unwrap(), buy: order };
                    trades.push(trade);
                } else {
                    orderbook.buys.push(order);
                }
            },
            _ => panic!("unknown side: {}", order.side)
        }
    }
    // Save the trades and orderbook to JSON files
    let trades_json = serde_json::to_string(&trades).unwrap();
    let orderbook_json = serde_json::to_string(&orderbook).unwrap();
    let mut trades_file = fs::File::create("./trades.json").unwrap();
    let mut orderbook_file = fs::File::create("./orderbook.json").unwrap();
    trades_file.write_all(trades_json.as_bytes()).unwrap();
    orderbook_file.write_all(orderbook_json.as_bytes()).unwrap();
}
