// vec<T>
// vector can only store one type
// to store multiple types we can use enum

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut empty_vector: Vec<i32> = Vec::new(); // added type annotation using generic because it's empty and it needs to know what data it will be stored in
    let mut v = vec![1, 2, 3]; // type infered as Vec<i32>
    let _row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello world!")),
    ]; // using vector with enum types

    empty_vector.push(5); // add to last element
    empty_vector.push(6);
    empty_vector.push(7);
    empty_vector.pop(); //remove and return last element

    // getting the index element of vector using reference
    // by using this method if you access index larger than vector length, it will panic
    let second_element = &empty_vector[1];
    println!("The second element of empty_vector is: {}", second_element);

    // getting the n index element of vector using get
    // by using this method if you access index larger than vector length, it won't panic
    // it will give you None
    match empty_vector.get(100) {
        Some(6) => println!("The second element of empty_vector is: {}", second_element),
        None => println!("There is no 100th element"),
        _ => (),
    }

    // this code wont work because it has immutable and mutable borrow
    // when you push an element to the end of vector
    // the vector might require allocating new memory and copying the old elements to the new space
    //let first = &v[0];
    //v.push(4);
    //println!("The first element of v is: {}", first);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
