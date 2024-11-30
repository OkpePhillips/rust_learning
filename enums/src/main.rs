fn main() {
    
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();
}


// Enum variants can have different types and amounts of associated data
enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

enum Message {
    Quit, // has no data
    Move { x: i32, y: i32 }, // has data in the form of struct
    Write(String), // similar to tuple struct
    ChangeColor(i32, i32, i32), // similar to tuple struct as well
}
    // We can define methods on enums

impl Message {
    fn call(&self) {
            // method body would be defined here
    }
}
