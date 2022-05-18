use crate::{C,N, Order, Output};
use std::collections::{hash_map::Entry, HashMap};

pub fn process_n(row:N,orders_hashmap: &mut HashMap::<String, (Vec<Order>, Vec<Order>)>,output: &mut Vec::<Output>)
{
    let row_order = Order { user: row.user, price: row.price, qty: row.qty, user_order_id: row.user_order_id };
    match orders_hashmap.entry(row.symbol) {
        Entry::Vacant(e) => {
            output.push(Output::A(row.user,row.user_order_id));
                match row.side {
                    'B' => {
                        e.insert((vec![row_order], vec![]));
                        output.push(Output::B('B',row.price,row.qty));
                    },
                    'S' => {
                        e.insert((vec![], vec![row_order]));
                        output.push(Output::B('B',row.price,row.qty));
                    },
                    _ => panic!("Unrecognized side: {}", row.side)
                };
        },
        Entry::Occupied(mut e) => {
            let buy_vec = &e.get().0;
            let sell_vec = &e.get().1;
            match row.side {
                'B' => {
                    if !sell_vec.is_empty() && row.price >= sell_vec[0].price {
                        output.push(Output::R(row.user,row.user_order_id));
                    } else {
                        output.push(Output::A(row.user,row.user_order_id));
                        if (!buy_vec.is_empty() && row.price >= buy_vec[buy_vec.len()-1].price) || buy_vec.is_empty() {
                            let mut qty = row.qty;
                            let mut iter = buy_vec.len() as i64 -1;
                            while iter >= 0 && row.price == buy_vec[iter as usize].price {
                                qty += buy_vec[iter as usize].qty;
                                iter -= 1;
                            }
                            output.push(Output::B('B',row.price,qty));
                        }
                        let mut iter = buy_vec.len() as i64 -1;
                        while iter > 0 && buy_vec[iter as usize].price > row_order.price {
                            iter -= 1;
                        }
                        e.get_mut().0.insert(iter as usize, row_order);
                    }
                },
                'S' => {
                    if !buy_vec.is_empty() && row.price <= buy_vec[buy_vec.len()-1].price {
                        output.push(Output::R(row.user,row.user_order_id));
                    } else {
                        output.push(Output::A(row.user,row.user_order_id));
                        if (!sell_vec.is_empty() && row.price <= sell_vec[0].price) || sell_vec.is_empty() {
                            let mut qty = row.qty;
                            let mut iter = 0;
                            while iter < sell_vec.len() && row.price == sell_vec[iter].price {
                                qty += sell_vec[iter].qty;
                                iter += 1;
                            }
                            output.push(Output::B('S',row.price,qty));
                        }
                        let mut iter = 0;
                        while iter < sell_vec.len() && sell_vec[iter].price < row_order.price {
                            iter += 1;
                        }
                        e.get_mut().1.insert(iter, row_order);
                    }
                },
                _ => panic!("Unrecognized side: {}", row.side)
            };
        }
    }
}

pub fn process_c(row:C,orders_hashmap: &mut HashMap::<String, (Vec<Order>, Vec<Order>)>,output: &mut Vec::<Output>)
{
    output.push(Output::A(row.user,row.user_order_id));
    for (_, (buy_vec, sell_vec)) in orders_hashmap {
        if let Some(i) = buy_vec.iter().position(|x| x.user == row.user && x.user_order_id == row.user_order_id) {
            let best_buy_price = buy_vec[buy_vec.len()-1].price;

            if buy_vec[i].price == best_buy_price {
                let counter = buy_vec.iter().filter(|x| x.price == best_buy_price).count();
                if counter == 1 && i > 0 {
                    output.push(Output::B('B',buy_vec[i-1].price, buy_vec[i-1].qty));
                } else if counter > 1 { 
                    let qty: i64 = buy_vec.iter().filter(|x| x.price == best_buy_price).map(|y| y.qty).sum();
                    output.push(Output::B('B',buy_vec[i].price, qty - buy_vec[i].qty));
                } else {
                    output.push(Output::B('B', 0, 0));
                }
            }
            buy_vec.remove(i);
        }

        if let Some(i) = sell_vec.iter().position(|x| x.user == row.user && x.user_order_id == row.user_order_id) {
            let best_sell_price = sell_vec[0].price;
            if sell_vec[i].price == best_sell_price {
                let counter = sell_vec.iter().filter(|x| x.price == best_sell_price).count();
                if counter == 1 && i < sell_vec.len()-1 {
                    output.push(Output::B('S',sell_vec[i+1].price, sell_vec[i+1].qty));
                } else if counter > 1 { 
                    let qty: i64 = sell_vec.iter().filter(|x| x.price == best_sell_price).map(|y| y.qty).sum();
                    output.push(Output::B('S',sell_vec[i].price, qty - sell_vec[i].qty));
                } else {
                    output.push(Output::B('S', 0, 0));
                }
            }
            sell_vec.remove(i);
        }
    }
}

pub fn process_f(orders_hashmap: &mut HashMap::<String, (Vec<Order>, Vec<Order>)>)
{
    orders_hashmap.clear();
}