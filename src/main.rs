use std::fs;
use serde::{Deserialize, Serialize};

fn main() {
    let json_file_path = fs::canonicalize("./orders.json").unwrap();
    let file = fs::File::open(json_file_path).unwrap();
    
    #[derive(Debug, Deserialize, Serialize)]
    struct Order {
        type_op: String,
        account_id: String,
        amount: String,
        order_id: String,
        pair: String,
        limit_price: String,
        side: String
    }
    let orders:Vec<Order> = serde_json::from_reader(file).expect("error while reading json");
    println!("{:#?}", orders);
}
