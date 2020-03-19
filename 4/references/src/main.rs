fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);
    // by passing the references, the calculate_length do not take the ownership
    // of s1

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("World");
    change(&mut s2); // pass mut if you want to modify references
    println!("s2: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    // & here refers to borrowing
    // borrowed value cannot be modified
    s.len()
}

fn change(s: &mut String) {
    s.push_str("!");
}
