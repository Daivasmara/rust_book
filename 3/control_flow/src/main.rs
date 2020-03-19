fn main() {
    // IF ELSE
    let number = 1;

    if number % 3 == 0 {
        println!("First condition was true");
    } else if number % 2 == 0 {
        println!("Second condition was true");
    } else {
        println!("Conditions was false");
    }

    let condition = true;

    let x = if condition { 5 } else { 6 };
    println!("x: {}", x);

    // LOOP
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter: {}", counter);

        if counter == 10 {
            break counter * 2; // return value from loops
        }
    };

    println!("The result is {}", result);

    // WHILE
    let mut n = 0;

    while n != 3 {
        println!("{}!", n);
        n += 1;
    }

    println!("n: {}", n);

    // FOR
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("END!");
}
