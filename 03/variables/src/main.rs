fn main() {
    const PI: f64 = 3.14; // const => always immutable
    println!("PI: {}", PI);
    let x = 5; // let => default to immutable
    println!("The value of x is: {}", x);
    let x = x + 1; //shadowing
    println!("The value of x is: {}", x);
    let spaces = " ";
    let spaces = spaces.len(); //shadowing allowing changing data type
    println!("The value of 'spaces' is: {}", spaces);
}
