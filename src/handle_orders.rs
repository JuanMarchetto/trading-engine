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
                // IF there is a buy order with the same limit_price or higher,
                // then we have a trade between the two orders
                // and we remove the buy order from the orderbook
                // else we add the order to the sells side of the orderbook
                if let Some(buy_index) = orderbook
                    .buys
                    .iter()
                    .position(|buy_order| buy_order.limit_price >= order.limit_price)
                {
                    let buy_order = orderbook.buys.remove(buy_index);
                    trades.push(Trade {
                        sell: order,
                        buy: buy_order,
                    });
                } else {
                    orderbook.sells.push(order);
                }
            }
            "BUY" => {
                // IF there is a sell order with the same price or lower,
                // then we have a trade
                // else we add the order to the buys side of the orderbook
                if let Some(sell_index) = orderbook
                    .sells
                    .iter()
                    .position(|sell_order| sell_order.limit_price <= order.limit_price)
                {
                    let sell_order = orderbook.sells.remove(sell_index);
                    trades.push(Trade {
                        sell: sell_order,
                        buy: order,
                    });
                } else {
                    orderbook.buys.push(order);
                }
            }
            _ => panic!("unknown side: {}", order.side),
        }
    });
    (orderbook, trades)
}
