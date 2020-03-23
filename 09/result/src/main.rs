// result is used for recoverable errors
// enum Result<T, E> {
//  Ok(T),
//  Err(E),
// }

use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // example above are using match, which could be a bit verbose
    // instead we can use unwrap, which is a shortcut that will take the value
    // of Ok and will call the panic macro if the value is Err
    // another method is to use expect, which is the same with unwrap, but
    // we can add custom message

    let _f2 = File::open("hello.txt").unwrap();
    let _f3 = File::open("hello.txt").expect("Failed to open hello.txt");
}

// error propagation
fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// refactor using ? operator, which will return the value when got an Ok or Err
// the ? operator can be used in functions that have return type of Result
fn _read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// refactor again
fn _read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// refactor again
fn _read_username_from_file_final() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
