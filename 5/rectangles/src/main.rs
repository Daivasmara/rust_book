#[derive(Debug)] // to be able to use display traits for structs

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // methods first parameters will always be &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is not a method, this is asscociated function, like String::from
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 100,
    };
    let rect4 = Rectangle::square(30); // using associated function

    println!("rect1 is {:#?}", rect1); // pretty display
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("This is a square rect4: {:#?}", rect4);
}
