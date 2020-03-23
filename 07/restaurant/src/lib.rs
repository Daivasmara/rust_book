// parent module
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // child module // sibling with mod hosting
    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {
            super::super::eat_at_restaurant(); // super is like .. in terminal
        }
    }

    pub mod parking {
        fn find_parking_space() {}
    }
}

mod back_of_house {
    // pub structs
    pub struct Breakfast {
        pub toast: String,      // still needs pub for the field
        seasonal_fruit: String, // not pub because it's decided by chefs based on the season
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // pub enum
    // only need pub before enum keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// using use, you still need to use the parent's module instead of directly
// using add_to_waitlist(), this is to make it clear that the function isn't locally defined
use crate::front_of_house::hosting;
// could also relative path for use
// use front_of_house::hosting;
// could also alias using as
// use front_of_house::hosting as FHosting

// nested paths
use crate::front_of_house::{hosting, serving};
// self nested paths
use crate::front_of_house::{self, hosting};
// glob operator
use crate::front_of_house::*;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path // eat_at_restaurant is front_of_house sibling, thus allowing it to access it
    front_of_house::hosting::add_to_waitlist();

    // using use
    hosting::add_to_waitlist();

    // STRUCTS
    // ordering rye toast
    let meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);

    // ENUM
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
