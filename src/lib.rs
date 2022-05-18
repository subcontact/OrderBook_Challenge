use std::fs::File;
use std::{env, io::BufReader};
use std::fmt;
use csv_parse::parse_csv;

mod csv_parse;
mod process_order;


pub fn lib_main(){
    let args: Vec<String> = env::args().collect();
    let csv_file = args[1].to_string();
    
    let file = File::open(csv_file).unwrap();
    let mut reader = BufReader::new(file);
    let lines: Vec<Output> = parse_csv(&mut reader).unwrap();
    for line in lines {
        println!("{}", line);
    }
}

#[derive(PartialEq,Debug)]
pub enum Output {
    A(i64, i64),
    B(char, i64, i64),
    R(i64, i64),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Output::A(user_id,user_order_id) => write!(f, "A, {}, {}", user_id, user_order_id),
            Output::B(side, price, total_quantity) => write!(
                f, "B, {}, {}, {}", 
                side, 
                if price > 0 {
                    price.to_string()
                } else {
                    "-".to_string()
                }, 
                if total_quantity > 0 {
                    total_quantity.to_string()
                } else {
                    "-".to_string()
                }),
            Output::R(user_id,user_order_id) => write!(f, "R, {}, {}", user_id, user_order_id),
       }
    }
}

pub struct N{
    user: i64, 
    symbol: String, 
    price:i64, 
    qty:i64, 
    side:char, 
    user_order_id:i64
}

pub struct C{
    user:i64,
    user_order_id:i64
}

pub struct Order {
    user: i64,
    price: i64,
    qty: i64,
    user_order_id: i64
}

#[test]
fn scenario_1() {
    let mut str_ = 
b"
#name: scenario 1
#descr:balanced book

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102 

# hit book on each side, generate reject
N, 1, IBM, 11, 100, B, 3
N, 2, IBM, 10, 100, S, 103

# replenish book on each side, TOB = 10/11
N, 1, IBM, 10, 100, B, 4
N, 2, IBM, 11, 100, S, 104
F
" as &[u8];

    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
      };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(1, 3),
        Output::R(2, 103),
        Output::A(1, 4),
        Output::B('B', 10, 200),
        Output::A(2, 104),
        Output::B('S', 11, 200)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_2() {
    let mut str_ = 
b"
#name: scenario 2
#descr: shallow bid

# build book, shallow bid, TOB = 10/11
N, 1, AAPL, 10, 100, B, 1
N, 1, AAPL, 12, 100, S, 2
N, 2, AAPL, 11, 100, S, 102

# hit bid, generate reject
N, 2, AAPL, 10, 100, S, 103

#  increase volume to Bid TOB 10, 200
N, 1, AAPL, 10, 100, B, 3
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(2, 103),
        Output::A(1, 3),
        Output::B('B', 10, 200)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_3() {
    let mut str_ = 
b"
#name: scenario 3
#descr: shallow ask

# build book, shallow ask, TOB = 10/11
N, 1, VAL, 10, 100, B, 1
N, 2, VAL, 9, 100, B, 101
N, 2, VAL, 11, 100, S, 102

# hit ask, generate reject
N, 1, VAL, 11, 100, B, 2

# increase volume to Ask TOB 10, 200
N, 2, VAL, 11, 100, S, 103
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(1, 2),
        Output::A(2, 103),
        Output::B('S', 11, 200)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_4() {
    let mut str_ = 
b"
#name: scenario 4
#descr: balanced book, limit below best bid

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# limit below best bid, generate reject
N, 2, IBM, 9, 100, S, 103
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(2, 103)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_5() {
    let mut str_ = 
b"
#name: scenario 5
#descr: balanced book, limit above best ask

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# limit above best ask, generate reject
N, 1, IBM, 12, 100, B, 103
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(1, 103)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_6() {
    let mut str_ = 
b"
#name: scenario 6
#descr: tighten spread through new limit orders

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 16, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 15, 100, S, 102

# new bid, ask TOB = 11/14
N, 2, IBM, 11, 100, B, 103
N, 1, IBM, 14, 100, S, 3
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 16, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 15, 100),
        Output::A(2, 103),
        Output::B('B', 11, 100),
        Output::A(1, 3),
        Output::B('S', 14, 100)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_7() {
    let mut str_ = 
b"
#name: scenario 7
#descr: balanced book, limit sell partial

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# limit sell, generate reject
N, 2, IBM, 10, 20, S, 103
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(2, 103)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_8() {
    let mut str_ = 
b"
#name: scenario 8
#descr: balanced book, limit buy partial

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# limit buy, generate reject
N, 1, IBM, 11, 20, B, 3
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::R(1, 3)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_9() {
    let mut str_ = 
b"
#name: scenario 9
#descr: balanced book, cancel best bid and offer

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# cancel best bid and offer
C, 1, 1
C, 2, 102
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::A(1, 1),
        Output::B('B', 9, 100),
        Output::A(2, 102),
        Output::B('S', 12, 100)];
    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_10() {
    let mut str_ = 
b"
#name: scenario 10
#descr: balanced book, cancel behind best bid and offer

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# cancel orders, TOB = 10/11
C, 1, 2
C, 2, 101
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::A(1, 2),
        Output::A(2, 101)];
    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_11() {
    let mut str_ = 
b"
#name: scenario 11
#descr: balanced book, cancel all bids

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# cancel all bids, TOB = -/11
C, 1, 1
C, 2, 101
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::A(1, 1),
        Output::B('B', 9, 100),
        Output::A(2, 101),
        Output::B('B', 0, 0)];

    assert_eq!(&output[..], &correct[..]);
}

#[test]
fn scenario_12() {
    let mut str_ = 
b"
#name: scenario 12
#descr: balanced book, TOB volume changes

# build book, TOB = 10/11
N, 1, IBM, 10, 100, B, 1
N, 1, IBM, 12, 100, S, 2
N, 2, IBM, 9, 100, B, 101
N, 2, IBM, 11, 100, S, 102

# increase and decrease the TOB volume
N, 2, IBM, 11, 100, S, 103
C, 2, 103

# cancel all asks
C, 2, 102
C, 1, 2
F
" as &[u8];
    let output = match parse_csv(&mut str_) {
        Ok(res) => res,
        Err(_) => todo!(),
    };
    let correct = vec![
        Output::A(1, 1),
        Output::B('B', 10, 100),
        Output::A(1, 2),
        Output::B('S', 12, 100),
        Output::A(2, 101),
        Output::A(2, 102),
        Output::B('S', 11, 100),
        Output::A(2, 103),
        Output::B('S', 11, 200),
        Output::A(2, 103),
        Output::B('S', 11, 100),
        Output::A(2, 102),
        Output::B('S', 12, 100),
        Output::A(1, 2),
        Output::B('S', 0, 0)];

    assert_eq!(&output[..], &correct[..]);
}
