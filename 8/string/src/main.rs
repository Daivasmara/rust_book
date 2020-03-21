fn main() {
    let s1 = String::from("hello ");
    let s2 = "world!".to_string(); // the same thing as using String::from

    let mut foo = String::from("foo");
    foo.push_str("bar");
    println!("{}", foo);

    foo.push('s'); // push takes only char not string
    println!("{}", foo);

    let s3 = s1 + &s2; // note s1 has been moved here and no longer can be used
                       // why because + concatenation actually use this (not exactly) add function
                       // fn add(self, s: &str) -> String
                       // and even tho s2 is not &str but &String, the compiler can coerce &String argument into &str
    println!("{}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tictactoe = format!("{}-{}-{}", tic, tac, toe); // this macro return a String, this doesn't take ownership of any of its parameter
    println!("{}", tictactoe);
    println!("{}", tic); // proves that tic is still usable

    // String in rust doesn't support indexing, because of how rust stores strings in memory
    // String is a wrapper over a Vec<u8>
    let len1 = String::from("hello").len(); // the len is 5 therefore it's 5 bytes long, each letter 1 byte
    let len2 = String::from("Здравствуйте").len(); // the character is 12, but the len is 12 because it's 24 bytes long
                                                   // therefore string in rust doesn't support indexing, because it's unsafe
    println!("len1: {}, len2: {}", len1, len2); // prove that len2 is 24 bytes long

    // Another point about UTF-8 is that there are 3 relevant ways to look at strings from rust's
    // perspective: bytes, scalar values, and grapheme clusters (the closest thing to letters)

    // chars is for the scalar values
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    // this is for the bytes
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }
}
