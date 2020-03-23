#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn main() {
    let some_u8_values = Some(0u8);

    match some_u8_values {
        Some(3) => println!("three"),
        _ => (),
    }

    // code above too verbose since you need to add _ in order to make this code works,
    // instead you can use if let, example below

    if let Some(3) = some_u8_values {
        println!("three");
    }

    // you can think of if let as an syntax sugar for a match
    // another example below

    let mut count: u32 = 0;
    let coin1: Coin = Coin::Quarter(UsState::Alabama);
    coin_func_match(&coin1, &mut count);
    coin_func_if_let(&coin1, &mut count);

    let coin2: Coin = Coin::Penny;
    coin_func_match(&coin2, &mut count);
    coin_func_if_let(&coin2, &mut count);
}

fn coin_func_match(coin: &Coin, count: &mut u32) {
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => {
            *count += 1;
            println!("count: {}", count);
        }
    }
}

fn coin_func_if_let(coin: &Coin, count: &mut u32) {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        *count += 1;
        println!("count: {}", count);
    }
}
