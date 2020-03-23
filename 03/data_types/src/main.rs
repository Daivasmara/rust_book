fn main() {
    // data types subsets: scalar and compound
    // --------------------------------------

    // SCALAR
    // represent single value, has four primary types
    // integers, floating points, numbers, booleans, characters

    // INTEGER
    // i => signed -> contain negative numbers, -(2^(n-1)) to 2^(n-1) - 1 inclusive
    // u => unsigned -> contain only positive numbers, 0 to 2^n - 1 inclusive
    let int = 5; // default to i32
    let x: u8 = 10; // u8 => 0 to 255
    println!("{} + {} = {}", int, x, int + x);

    const THOUSAND: u16 = 1_000; // '_' as visual separator
    println!("Thousand is: {}", THOUSAND);

    // FLOAT
    // f32 => single precistion float
    // f64 => double precision float
    let fl = 2.0; // default to f64
    let y: f32 = 3.0;
    println!("{} + {} = {}", fl, y, fl + y);

    // BOOLEAN
    // bool => one byte in size
    let b: bool = false;
    println!("true or false? {}", b);

    // CHAR
    // use single quotes, unlike string literals that use double quotes
    let c = 'c';
    println!("{}", c);

    // ------------------------------------

    // COMPOUND
    // can group multiple values into one type, has 2 primitive compound type
    // tuples and arrays

    // TUPLE
    // grouping together a number of values with a variety of types into one
    // has fixed length, once declared they can't grow or shink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("t1: {}", tup.0);
    let (_t1, t2, _t3) = tup; // desctructuring
    println!("t2: {}", t2);

    // ARRAY
    // unlike tuple, every element of arary must have the same type
    // has fixed length as well
    // use stack instead of heap
    let arr = [1, 2, 3, 4, 5];
    let _arr_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let _arr_with_default_value = [3, 5]; // [3, 3, 3, 3, 3];
    println!("first element of arr: {}", arr[0]);
}
