// borrow checker -> compares scopes to determine whether all borrows are valid
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// this means parameters that has 'a has the same lifetime, therefore it will compile
// because the compiler no longer confused which value will return because both
// had different lifetime
// note: this doesn't change the lifetime of any of its, it just so computer
// are able to understand
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
