use std::fs::{canonicalize, File};
use std::io::Write;

mod deserializers;
mod handle_orders;
mod structs;

use handle_orders::handle_orders;
use structs::{Order, Orderbook, Trade};

fn main() {
    // Read the file
    let json_file_path = canonicalize("./orders.json").unwrap();
    let file = File::open(json_file_path).unwrap();
    // Deserialize the file
    let orders: Vec<Order> = serde_json::from_reader(file).expect("error while reading json");
    // Handle the orders
    let (orderbook, trades): (Orderbook, Vec<Trade>) = handle_orders(orders);
    // Save the trades and orderbook to JSON files
    let trades_json = serde_json::to_string(&trades).unwrap();
    let orderbook_json = serde_json::to_string(&orderbook).unwrap();
    let mut trades_file = File::create("./trades.json").unwrap();
    let mut orderbook_file = File::create("./orderbook.json").unwrap();
    trades_file.write_all(trades_json.as_bytes()).unwrap();
    orderbook_file.write_all(orderbook_json.as_bytes()).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_orders() {
        let orders = vec![
                Order {
                    type_op: "CREATE".to_string(),
                    account_id: 1,
                    amount: 0.00230,
                    order_id: 1,
                    pair: "BTC/USDC".to_string(),
                    limit_price: 63500.00,
                    side: "SELL".to_string()
                },
                Order {
                    type_op: "CREATE".to_string(),
                    account_id: 2,
                    amount: 0.00230,
                    order_id: 2,
                    pair: "BTC/USDC".to_string(),
                    limit_price: 63500.00,
                    side:"BUY".to_string()
                }];
        let (orderbook, trades): (Orderbook, Vec<Trade>) = handle_orders(orders);
        assert_eq!(orderbook.sells.len(), 0);
        assert_eq!(orderbook.buys.len(), 0);
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].sell.order_id, 1);
        assert_eq!(trades[0].buy.order_id, 2);
    }
}
