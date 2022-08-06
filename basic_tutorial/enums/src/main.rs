enum IpAddreKind {
    V4(u8,u8, u8, u8), 
    V6(String),
}

fn main() {
    let localhost = IpAddreKind::V4(127,0,0,1);

    let x  = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    let cents = value_in_cents(Coin::Dime);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

