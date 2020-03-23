// statements => not returning values (e.g let x = 5)
// expressions => returning values (e.g 5)

fn main() {
    let x = five();
    let y = {
        let z = x;
        z + 5 // for return, no need to add semicolon
    };

    another_funtion(20, 55);
    println!("x: {}", x);
    println!("y: {}", y);
}

fn another_funtion(a: i32, b: i32) {
    println!("a: {}", a);
    println!("b: {}", b);
}

fn five() -> i32 {
    // explicitly define returned data type
    5
}
