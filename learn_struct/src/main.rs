fn main() {
    println!("Hello, world!");

    // Instantiating a struct
    let user1 = User {
        active: true,
        username: String::from("Godwin"),
        email: String::from("admin@philstech.com"),
        sign_in_count: 1
    };

    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);


    // mutable instances must have the mut keyword
    let mut user2 = User {
        active: true,
        username: String::from("Philstech"),
        email: String::from("admin@philstech.com"),
        sign_in_count: 1
    };

    user2.email = String::from("admin@agriok.org");

    println!("{}", user2.email);

    let user3 = build_user(String::from("admin@agriok.org"), String::from("Philstech"));

    println!("{}", user3.email);

    // using struct update syntax
    let user4 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!("{user4:#?}");

    // tuple structs need no field names, just types
    struct Color(i32, i32, i32); // the tuple struct is named Color
    struct Point(i32, i32, i32);

    // Unit-like structs have no fields!
    struct AlwaysEqual;
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// A function can also return a struct instance

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// The function below uses field init shorthand for the struct field names

fn build_new_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
