#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = value_in_cents(Coin::Penny);
    println!("coin1: {}", coin1);
    let coin2 = value_in_cents(Coin::Nickel);
    println!("coin2: {}", coin2);
    let coin3 = value_in_cents(Coin::Dime);
    println!("coin3: {}", coin3);
    let coin4 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("coin4: {}", coin4);
    let coin5 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("coin5: {}", coin5);

    let x = 100;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!(), // _ match any value // () unit value, nothing will happen
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
