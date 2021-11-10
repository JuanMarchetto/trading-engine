use crate::structs::{Order, Orderbook, Trade};

pub fn handle_orders(orders: Vec<Order>) -> (Orderbook, Vec<Trade>) {
    let mut orderbook = Orderbook {
        sells: vec![],
        buys: vec![],
    };
    let mut trades = vec![];
    orders.into_iter().for_each(|order| {
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
                    let trade = Trade {
                        sell: order,
                        buy: orderbook.buys.pop().unwrap(),
                    };
                    trades.push(trade);
                } else {
                    orderbook.sells.push(order);
                }
            }
            "BUY" => {
                // IF there is a sell order with the same price or lower,
                // then we have a trade
                // else we add the order to the buys side of the orderbook
                let mut found_sell = false;
                for sell in orderbook.sells.iter() {
                    if sell.limit_price <= order.limit_price {
                        found_sell = true;
                        break;
                    }
                }
                if found_sell {
                    let trade = Trade {
                        sell: orderbook.sells.pop().unwrap(),
                        buy: order,
                    };
                    trades.push(trade);
                } else {
                    orderbook.buys.push(order);
                }
            }
            _ => panic!("unknown side: {}", order.side),
        }
    });
    (orderbook, trades)
}
