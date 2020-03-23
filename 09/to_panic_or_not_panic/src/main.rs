pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 {
            panic!(
                "Guess values must bee between 1 and 100, got {} instead.",
                value
            );
        }

        Guess { value }
    }

    // getter, because value on struct is private to prevent changing it forcefully not using new associated function
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(10);
    let _guess2 = Guess::new(101); // panic
    println!("Guess: {}", guess.value());
}
