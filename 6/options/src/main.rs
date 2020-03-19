fn main() {
    // rust desnt have nulls
    // options<T> provides enum that can encode the concept of a value being present or absent
    // Some and None
    let some_number = Some(5);
    let some_string = Some("Good Morning");
    let absent_number: Option<i32> = None; // null

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
