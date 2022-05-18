use std::collections::HashMap;
use std::io::{BufRead, Error, ErrorKind, self};

use crate::{N, C, Order, Output};
use crate::process_order::{process_n,process_c,process_f};


pub fn parse_csv<R: BufRead>(reader: &mut R) -> io::Result<Vec<Output>> {
    let mut orders_hashmap = HashMap::<String, (Vec<Order>, Vec<Order>)>::new();
    let mut output = Vec::<Output>::new();
    for line in reader.lines() {
        let line = line?;

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue
        }
        
        let items: Vec<&str> = trimmed.split(", ").collect();
        match items[0] {
            "N" => {
                process_n(N{
                    user: items[1].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                    symbol: items[2].to_string(),
                    price: items[3].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                    qty: items[4].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                    side: items[5].parse::<char>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                    user_order_id: items[6].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                },&mut orders_hashmap, &mut output);
            }
            "C" => {
                process_c(C {
                    user: items[1].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                    user_order_id: items[2].parse::<i64>().map_err(|e| Error::new(ErrorKind::Other, e))?,
                },&mut orders_hashmap, &mut output);
            }
            "F" => {process_f(&mut orders_hashmap);},
            _ => {},
        }
    }
    
    Ok(output)
}