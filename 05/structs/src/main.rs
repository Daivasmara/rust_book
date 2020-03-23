fn main() {
    // STRUCTS
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email, // shorthand just like es6 javascript
            username,
            sign_in_count: 1,
            active: true,
        }
    }

    let user1 = build_user(
        String::from("daivasmara@gmail.com"),
        String::from("daivasmara"),
    );

    let user2 = User {
        email: String::from("denaw@gmail.com"),
        username: String::from("denaw"),
        ..user1 // specifies the remaining fields that are not explicitly set to have the same values as user1
    };

    println!("{}", user1.username);
    println!("{}", user2.sign_in_count);

    // TUPLE STRUCTS
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black #0: {}", black.0);
}
