// string literals are slices
// let s = "hello world" -> slices, thus immutable
fn main() {
    let s = String::from("Hello world");

    // string slices
    let hello = &s[..5]; // 0-4
    let world = &s[6..11]; // 6-10
    println!("{} {}", hello, world);

    let fword = first_word(&s[..]);
    println!("first word: {}", fword);
    let sword = second_word(&s[..]);
    println!("second_word: {}", sword);

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    // &str signifies string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}
