// generic in rust do not take a hit on performance, because rust use monomorphization
// which means when compiling, rust fill in the T with corresponding types

// generic functions
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic structs
struct Point1<T> {
    x: T,
    y: T,
}

// generic method, note we have to declare T just after impl so rust can identify
// that the type in the angle brackets in Point is a generic type rather than a concrete type
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we use concrete type here so we dont need to declare any type after impl
// this block of code means other Point<T> that is not the type f32 will not have this method defined
impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// generic enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {}
