// traits -> functionality a particular type has and can be shared with other types
// to define shared behaviour in an abstract way
// consist of the methods
use std::fmt::Display;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;

    // trait can also define default behaviour
    // we can use this by defining empty impl blocks
    // impl Summary for NewsArticle {}
    fn summarize_author(&self) -> String;
    fn read_more(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("daivasmara"),
        content: String::from("Gempa di bogor!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available, {}", tweet.read_more());
    notify(&tweet);
    notify2(&tweet);
}

// using Summary trait as parameter
// acceptring parameter item that implement summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the code above is syntax sugar/equivalent to this implementation
// called trait bounds
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// more than one traits
// accepting item parameter that implement both Summary and Display trait
pub fn notify3(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// clearer trait bounds with where clauses
pub fn some_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + Summary,
{
    format!("{}, {}", t, u)
}

// returning types that implement traits
pub fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("daivasmara"),
        content: String::from("Gempa di bogor!"),
        reply: false,
        retweet: false,
    }
}

// largest using trait bounds
// only works for list which implements PartialOrd and Copy
// try to remove the Copy and see the error and understand why it is what it is
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// using trait bounds to conditionally implement methods
struct _Pair<T> {
    x: T,
    y: T,
}

impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> _Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// example of implementing traits only for type that implement a traits
// impl<T: Display> ToString for T {}
// implement ToString traits for types that implements Display traits
