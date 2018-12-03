enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Other1,
    Other2,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 100,
    }
}

fn main() {
    let o = Coin::Other1;

    let value = value_in_cents(o);

    println!("value: {}", value);
}
