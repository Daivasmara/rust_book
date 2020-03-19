fn main() {
    let x = 5; // this x is for main scope only
    println!("x: {}", x);
    second_main();
    println!("x: {}", x);
    hello();
    copy_stack_data();
    ownership();
    clone();

    // example of taking ownership using function
    let s = String::from("S ownership is moved to function");

    some_func_string(s);
    // s no longer valid here because data type is heap need allocation
    // needs allocations in runtime
    // println!("{}", s);

    let num = 5;

    some_func_int(num);
    // num is still valid here because data type is int, one of data type of many
    // most of them are scalar types that is able to be copied
    println!("{}", num);
}

fn second_main() {
    let x = 10; // this x is for hello scope only
    println!("x: {}", x);
}

fn hello() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);
}

fn copy_stack_data() {
    let x = "still valid"; // still valid because it stored in stack
    let y = x;

    println!("x: {}", x);
    println!("y: {}", y);
}

fn ownership() {
    let x = String::from("Ownership");
    let y = x;

    //println!("{}", x); // x no longer works, x moved to y because it uses heap
    println!("{}", y);
}

fn clone() {
    let s1 = String::from("Hello");
    let s1 = s1.clone(); // the heap data does get copied

    println!("s1: {}, s2: {}", s1, s1);
}

fn some_func_int(x: i32) {
    println!("{}", x);
}

fn some_func_string(x: String) {
    println!("{}", x);
}
