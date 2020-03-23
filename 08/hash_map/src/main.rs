use std::collections::HashMap;

fn main() {
    // key and value gotta be the same type
    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Blue"), 25); // overwritting value

    // another method of creating hashmap using collect
    // collect is a method to gathers data into a number of collection types
    // including HashMap
    let teams = vec![String::from("Red"), String::from("Blue")];
    let initial_scores = vec![10, 20];

    let _scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ownership, for types that implement Copy trait like i32, the values are copied
    // for owned values like String, the values will be moved
    let field_name = String::from("Red");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field value are invalid at this point

    // getting the value using get method
    // the value will be Some or None depends if the value exist or not
    let team_name = String::from("Red");
    match scores.get(&team_name) {
        Some(n) => println!("Scores of team {}: {}", &team_name, n),
        None => (),
    }

    let _test_doang = String::from("test doang");
    let test_lagi = String::from("test lagi dong");
    println!("{}", test_lagi);

    // we can also use for loop to iterate each key/value pair in HashMap
    for (key, value) in &scores {
        println!("Scores of team {}: {}", key, value);
    }

    // entry -> to check whether a particular key has value already, and if it doesn't
    // insert a value for it
    scores.entry(String::from("Red")).or_insert(50); // not inserted
    scores.entry(String::from("Yellow")).or_insert(50); // inserted
    println!("{:?}", scores);

    // entry also returns mutable reference (&mut V) to the value for the key
    // therefore we can use it to update value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // this return &mut V
        *count += 1; // deference count
    }

    println!("{:?}", map);
}
